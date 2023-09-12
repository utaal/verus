FROM ubuntu:22.04 AS build

RUN \
    DEBIAN_FRONTEND=noninteractive \
    apt-get update && \
    apt-get install -y \
        build-essential \
        curl \
        python3

RUN mkdir /root/z3 && cd /root/z3 && \
    curl -LO https://github.com/Z3Prover/z3/archive/refs/tags/z3-4.12.2.tar.gz && \
    tar -xf z3-4.12.2.tar.gz && \
    rm z3-4.12.2.tar.gz && \
    cd z3-z3-4.12.2 && \
    python3 scripts/mk_make.py && \
    cd build && \
    make && \
    cp z3 ../../z3-4.12.2

FROM public.ecr.aws/lambda/provided:al2

ARG GIT_VERSION="2.26.2"

RUN \
    mkdir -p /opt/z3
    
COPY --from=build /root/z3/z3-4.12.2 /opt/z3/z3-4.12.2

RUN \
    yum install -y \
        wget \
        curl \
        tar \
        gzip \
        gcc \
        unzip \
        gettext \
        python3-pip \
        git

RUN \
    mkdir -p /cargo && \
    mkdir -p /rustup && \
    export CARGO_HOME=/cargo && \
    export RUSTUP_HOME=/rustup && \
    curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" | sh -s -- --default-toolchain none -y

RUN \
    mkdir /actions-runner && cd /actions-runner && \
    curl -o actions-runner-linux-arm64-2.308.0.tar.gz -L https://github.com/actions/runner/releases/download/v2.308.0/actions-runner-linux-arm64-2.308.0.tar.gz

RUN \
    mkdir /lambda && \
    pip3 install --target /lambda awslambdaric

COPY lambda_function.py /lambda/lambda_function.py

WORKDIR /lambda

ENTRYPOINT [ "/usr/bin/python3", "-m", "awslambdaric" ]

CMD [ "lambda_function.lambda_handler" ]

