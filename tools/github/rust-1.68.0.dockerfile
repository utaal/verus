FROM verus-github-ephemeral-runner-base:latest 

RUN \
    mv /cargo /tmp/cargo && \
    mv /rustup /tmp/rustup && \
    export CARGO_HOME=/tmp/cargo && \
    export RUSTUP_HOME=/tmp/rustup && \
    . /tmp/cargo/env && \
    rustup install 1.68.0 && \
    mv /tmp/cargo /cargo && \
    mv /tmp/rustup /rustup

WORKDIR /lambda

ENTRYPOINT [ "/usr/bin/python3", "-m", "awslambdaric" ]

CMD [ "lambda_function.lambda_handler" ]
