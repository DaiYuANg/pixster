FROM rust:1.87 as builder

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources && \
    sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources

RUN apt-get update && apt-get install -y g++-x86-64-linux-gnu libc6-dev-amd64-cross musl-tools pkg-config clang && rustup target add x86_64-unknown-linux-musl

ENV CC=musl-gcc

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static:nonroot

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/capster /usr/local/bin/capster

EXPOSE 5000

ENTRYPOINT ["/usr/local/bin/capster"]
