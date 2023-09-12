FROM verus-github-ephemeral-runner-base:latest 

ENV CARGO_HOME=/cargo
ENV RUSTUP_HOME=/rustup

RUN \
    . /cargo/env && \
    rustup install 1.68.0 && \
    rustup component add --toolchain 1.68.0 llvm-tools && \
    rustup component add --toolchain 1.68.0 rustc-dev

COPY lambda_function.py /lambda/lambda_function.py

WORKDIR /lambda

ENTRYPOINT [ "/usr/bin/python3", "-m", "awslambdaric" ]

CMD [ "lambda_function.lambda_handler" ]
