## -------- Stage 1: Build --------
#FROM rust:1.87 as builder
#
## 设置工作目录
#WORKDIR /usr/src/app
#
## 复制 Cargo.toml 和 Cargo.lock，先构建依赖层缓存
#COPY Cargo.toml Cargo.lock ./
#
## 仅构建依赖，避免每次都重编译所有代码
#RUN cargo build --release --bin capster || true
#
## 复制所有源码
#COPY src ./src
#
## 重新编译（包含源码）
#RUN cargo build --release --bin capster
#
## -------- Stage 2: 运行 --------
#FROM debian:bookworm-slim
#
## 安装运行时所需的依赖，比如 ca-certificates
#RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
#
## 拷贝编译好的二进制文件
#COPY --from=builder /usr/src/app/target/release/capster /usr/local/bin/capster
#
## 声明运行端口（可选）
#EXPOSE 5000
#
## 运行程序
#CMD ["capster"]

# -------- Stage 1: Build --------
FROM rust:1.87 as builder

RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources && \
    sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources

# 安装 musl 工具链
RUN apt-get update && apt-get install -y musl-tools pkg-config clang && rustup target add x86_64-unknown-linux-musl

# 创建工作目录
WORKDIR /usr/src/app

# 先复制 Cargo.toml 和 Cargo.lock（利用缓存）
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

# 覆盖伪造的 src
COPY . .

# 编译为静态链接的二进制
RUN cargo build --release --target x86_64-unknown-linux-musl

# -------- Stage 2: Runtime --------
FROM alpine:latest

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories

# 安装运行时 CA 证书（如果需要发 HTTPS 请求）
RUN apk add --no-cache ca-certificates

# 复制编译好的静态二进制
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/capster /usr/local/bin/capster

# 暴露服务端口（可选）
EXPOSE 5000

# 启动服务
ENTRYPOINT ["/usr/local/bin/capster"]
