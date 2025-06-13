# -------- Stage 1: Build --------
FROM rust:1.87 as builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制 Cargo.toml 和 Cargo.lock，先构建依赖层缓存
COPY Cargo.toml Cargo.lock ./

# 仅构建依赖，避免每次都重编译所有代码
RUN cargo build --release --bin capster || true

# 复制所有源码
COPY src ./src

# 重新编译（包含源码）
RUN cargo build --release --bin capster

# -------- Stage 2: 运行 --------
FROM debian:bookworm-slim

# 安装运行时所需的依赖，比如 ca-certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 拷贝编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/capster /usr/local/bin/capster

# 声明运行端口（可选）
EXPOSE 5000

# 运行程序
CMD ["capster"]
