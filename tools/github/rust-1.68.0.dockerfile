FROM verus-github-ephemeral-runner-base:latest 

RUN \
    export CARGO_HOME=/cargo && \
    export RUSTUP_HOME=/rustup && \
    export PATH="/cargo/bin:$PATH" && \
    rustup install 1.68.0 && \
    rustup component add --toolchain 1.68.0 llvm-tools && \
    rustup component add --toolchain 1.68.0 rustc-dev

WORKDIR /lambda

ENTRYPOINT [ "/usr/bin/python3", "-m", "awslambdaric" ]

CMD [ "lambda_function.lambda_handler" ]
