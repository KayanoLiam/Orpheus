# 构建阶段
FROM rust:1.91 as builder
WORKDIR /usr/src/app

# 拷贝 Cargo 文件，先下载依赖
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# 拷贝 SQLx 查询缓存
COPY .sqlx ./.sqlx

# 拷贝源码
COPY src ./src

# 构建 release 版本
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim 
WORKDIR /usr/src/app

# 安装必要库
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# 从 builder 拷贝编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/orpheus .

# 设置环境变量（可覆盖）
ENV RUST_LOG=info

# 容器启动命令
CMD ["./orpheus"]
