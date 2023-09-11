import os
import subprocess
import json

def lambda_handler(event, context):
    try:
        github_token = event.get('github_token', '')
        repo_url = event.get('repo_url', '')

        if not github_token:
            return {
                'statusCode': 400,
                'body': 'GitHub token is missing.'
            }

        if not repo_url:
            return {
                'statusCode': 400,
                'body': 'Repository url is missing.'
            }

        runner_dir = "/tmp/actions-runner"

        try:
            subprocess.run(["cp", "-r", "/actions-runner", runner_dir])
        except Exception as e:
            return {
                'statusCode': 500,
                'body': 'Copy of actions-runner failed.'
            }

        os.chdir(runner_dir)

        try:
            subprocess.run(["tar", "xzf", "./actions-runner-linux-arm64-2.308.0.tar.gz"])
        except Exception as e:
            return {
                'statusCode': 500,
                'body': 'Cannot extract actions runner.'
            }

        env_vars = os.environ.copy()
        env_vars["RUNNER_ALLOW_RUNASROOT"] = "1"

        config_result = subprocess.run(
            [
                "./config.sh",
                "--url", repo_url,
                "--token", github_token,
                "--ephemeral",
                "--unattended",
                "--labels", "ephemeral-lambda",
                "--work", "/tmp/_work",
                "--disableupdate"
            ],
            capture_output=True,
            env=env_vars
        )

        if config_result.returncode != 0:
            return {
                'statusCode': 500,
                'body': f"Failed to configure runner: {config_result.stderr.decode('utf-8')}"
            }

        env_vars['CARGO_HOME'] = '/cargo'
        env_vars['RUSTUP_HOME'] = '/rustup'
        env_vars['PATH'] = '/cargo/bin:' + env_vars['PATH']

        run_result = subprocess.run(
            [
                "./run.sh",
            ],
            capture_output=True,
            env=env_vars
        )

        if run_result.returncode == 0:
            return {
                'statusCode': 200,
                'body': 'Successfully executed GitHub Actions runner.'
            }
        else:
            return {
                'statusCode': 500,
                'body': f"Failed to start runner: {run_result.stderr.decode('utf-8')}"
            }

    except Exception as e:
        return {
            'statusCode': 500,
            'body': f"Exception: {str(e)}"
        }
