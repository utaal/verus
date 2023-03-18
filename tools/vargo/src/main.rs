use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CompilerMessage {
    pub rendered: String,
}

#[derive(Debug, Deserialize)]
pub struct CompilerTarget {
    pub kind: Vec<String>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "reason")]
pub enum Message {
    #[serde(rename = "compiler-message")]
    CompilierMessage { message: CompilerMessage },
    #[serde(rename = "compiler-artifact")]
    CompilierArtifact {
        target: CompilerTarget,
        filenames: Vec<String>,
        executable: Option<String>,
    },
    #[serde(rename = "build-script-executed")]
    BuildScriptExecuted { },
    #[serde(rename = "build-finished")]
    BuildFinished { },
}

const RELEVANT_TARGET_NAMES: &[&str] = &[
    "builtin",
    "builtin_macros",
    "state_machines_macros",
    "rust_verify",
    "vstd",
];

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let release = args.iter().find(|x| x.as_str() == "--release").is_some();

    let toml = std::fs::read_to_string("Cargo.toml").expect("could not read Cargo.toml");
    assert_eq!(toml.lines().next().unwrap(), "# vargo: main workspace tag (do not modify this line)");

    let target_verus_dir = {
        let parent_dir = std::path::PathBuf::from("target-verus");
        if !parent_dir.exists() {
            std::fs::create_dir(&parent_dir).expect("could not create target-verus directory");
        }
        let target_verus_dir = parent_dir.join(if release { "release" } else { "debug" });
        if target_verus_dir.exists() {
            std::fs::remove_dir_all(&target_verus_dir).expect("could not remove target-verus directory");
        }
        std::fs::create_dir(&target_verus_dir).expect("could not create target-verus directory");
        target_verus_dir
    };

    let cmd_position = args.iter().position(|x| x == "build" || x == "test" || x == "clean").expect("no build, test, or clean command");
    let cmd = args[cmd_position].clone();
    if cmd == "test" {
        match args.iter().position(|x| x == "--") {
            Some(pos) => {
                args.insert(pos + 1, "--color=always".to_string());
            },
            None => {
                args.push("--".to_string());
                args.push("--color=always".to_string());
            }
        }
    }

    let _package = args.iter().position(|x| x == "--package" || x == "-p").map(|pos| args[pos + 1].clone());

    if cmd == "clean" {
        std::process::Command::new("cargo")
            .env("RUSTC_BOOTSTRAP", "1")
            .args(&args)
            .status()
            .expect("could not execute cargo");
        return;
    }

    args.insert(cmd_position + 1, "--message-format=json-diagnostic-rendered-ansi".to_string());
    let mut cargo = std::process::Command::new("cargo")
        .env("RUSTC_BOOTSTRAP", "1")
        .env("VERUS_IN_VARGO", "1")
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("could not execute cargo");
    use std::io::BufRead;
    let output = std::io::BufReader::new(cargo.stdout.take().unwrap());

    #[cfg(target_os = "macos")]
    let (pre, dl) = ("lib", "dylib");

    #[cfg(target_os = "linux")]
    let (pre, dl) = ("lib", "so");

    #[cfg(target_os = "windows")]
    let (pre, dl) = ("", "dll");

    let rlib_re = regex::Regex::new((pre.to_string() + r"([a-zA-Z0-9_]+)-([a-zA-Z0-9_]+)\.rlib").as_str()).unwrap();

    let proc_macro_re = regex::Regex::new((pre.to_string() + r"([a-zA-Z0-9_]+)-([a-zA-Z0-9_]+)\." + dl).as_str()).unwrap();

    let bin_re = regex::Regex::new(r"([a-zA-Z0-9_]+)-([a-zA-Z0-9_]+)(\.[a-zA-Z]+)?").unwrap();

    let mut finished = false;

    for l in output.lines() {
        let l = l.unwrap();
        if finished {
            println!("{}", l);
            continue;
        }
        let Ok(m): Result<Message, _> = serde_json::from_str(&l) else {
            panic!("could not parse json: `{}`", l);
        };
        match m {
            Message::CompilierMessage { message } => {
                println!("{}", message.rendered);
            }
            Message::CompilierArtifact { ref target, .. } if target.kind.len() == 1 && target.kind[0] == "test" => {
            }
            Message::CompilierArtifact { ref target, ref filenames, ref executable } if RELEVANT_TARGET_NAMES.contains(&target.name.as_str()) => {
                assert_eq!(target.kind.len(), 1);
                match target.kind[0].as_str() {
                    "lib" => {
                        for from_f in filenames {
                            let from_f = std::path::PathBuf::from(from_f);
                            let to_f = {
                                let name = from_f.file_name().unwrap();
                                let Some(matches) = rlib_re.captures(name.to_str().unwrap()) else {
                                    continue;
                                };
                                let name = matches.get(1).unwrap().as_str();
                                let name = format!("{}{}.rlib", pre, name);
                                target_verus_dir.join(name)
                            };
                            std::fs::copy(&from_f, &to_f).expect("could not copy file");
                        }
                    },
                    "proc-macro" => {
                        for from_f in filenames {
                            let from_f = std::path::PathBuf::from(from_f);
                            let to_f = {
                                let name = from_f.file_name().unwrap();
                                let Some(matches) = proc_macro_re.captures(name.to_str().unwrap()) else {
                                    continue;
                                };
                                let name = matches.get(1).unwrap().as_str();
                                let name = format!("{}{}.{}", pre, name, dl);
                                target_verus_dir.join(name)
                            };
                            std::fs::copy(&from_f, &to_f).expect("could not copy file");
                        }
                    }
                    "bin" => {
                        let from_f = std::path::PathBuf::from(executable.as_ref().unwrap());
                        let to_f = {
                            let name = from_f.file_name().unwrap();
                            let matches = bin_re.captures(name.to_str().unwrap()).unwrap();
                            let name = matches.get(1).unwrap().as_str();
                            let ext = matches.get(3).map(|x| x.as_str()).unwrap_or("");
                            let name = format!("{}{}", name, ext);
                            target_verus_dir.join(name)
                        };
                        std::fs::copy(&from_f, &to_f).expect("could not copy file");
                    }
                    _ => todo!("kind: {:?}", target.kind)
                }
            }
            Message::BuildFinished { .. } => {
                finished = true;
            }
            _ => {}
        }
    }
    std::process::exit(cargo.wait().unwrap().code().unwrap());
}
