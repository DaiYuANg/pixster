# Rust Captcha Service

基于 Rust 和 Axum 框架的验证码服务，支持文本验证码和图片验证码生成，支持验证码校验，支持自定义验证码参数配置。

---

## 功能

- 生成验证码文本及唯一 token
- 生成验证码图片及唯一 token（支持参数自定义）
- 校验验证码结果
- 支持 Redis 或 Moka 作为缓存存储后端
- 内置 Prometheus 指标和 Swagger UI 文档

---

## 技术栈

- Rust 1.70+
- Tokio 异步运行时
- Axum Web 框架
- captcha_rs 生成验证码
- Moka / Redis 缓存存储
- utoipa + Swagger UI 自动生成接口文档
- Prometheus 监控指标

---

## 环境依赖

- Rust 编译环境，建议版本 1.70 以上
- Redis（可选，若使用 Redis 缓存）

---

## API 文档

1. 生成文本验证码
   请求 GET /captcha/generate

```json
{
  "token": "uuid-token",
  "captcha": "ABCDE"
}
```

2. 生成图片验证码 GET /captcha/generate/image
   参数 类型 说明 默认值
   length usize 验证码长度 5
   width u32 图片宽度 130
   height u32 图片高度 40
   complexity u32 验证码复杂度 1
   compression u8 图片压缩率（1-99） 99

返回 PNG 格式验证码图片，HTTP 头部包含 X-Captcha-Token，值为验证码 Token。

3. 验证验证码
   请求

bash
复制
编辑
GET /captcha/verify?token=xxx&value=xxx
查询参数

参数 类型 说明
token string 验证码令牌
value string 用户输入的值

响应

json
复制
编辑
true
表示校验成功，返回 false 表示失败。