FROM rust:1.45 as builder
WORKDIR /root

RUN mkdir -p /root/.cargo && \
printf '[source.crates-io]\n\
registry = "https://github.com/rust-lang/crates.io-index"\n\
replace-with = "ustc"\n\
[source.ustc]\n\
registry = "git://mirrors.ustc.edu.cn/crates.io-index"' > /root/.cargo/config

#RUN wget https://github.com/Kitware/CMake/releases/download/v3.15.2/cmake-3.15.2.tar.gz &&\
    #tar zxvf cmake-3.15.2.tar.gz &&\
    #cd cmake-3.15.2 &&\
    #./bootstrap &&\
    #make && make install

RUN apt-get update && apt-get install -y cmake musl-tools
RUN rustup toolchain install stable-x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl --toolchain stable

RUN USER=root cargo new virtual-mqtt-device
WORKDIR /root/virtual-mqtt-device
COPY Cargo.toml ./
COPY src ./
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/virtual-mqtt-device .
USER 1000
CMD ["./virtual-mqtt-device"]
