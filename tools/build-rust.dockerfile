FROM ubuntu

RUN apt update; apt install -y git python3 openssl libssl-dev cmake pkg-config curl llvm clang ninja-build; rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

WORKDIR /build

RUN git clone https://github.com/secure-foundations/rust.git; \
    cd verus; \
    bash tools/set-up-rust.sh; \
    bash tools/update-rust.sh

CMD cd verus; \
    bash tools/update-rust.sh; \
    cd rust/install; tar -cvzf ../../rust_install.tar.gz .; cd ../..; \
    mv rust_install.tar.gz /build/rust_install_$(git rev-parse --short HEAD).tar.gz
