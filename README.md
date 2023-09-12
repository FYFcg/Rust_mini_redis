# mini_redis
## 简介

一个简单的mini-redis，实现了命令:set, get, del, ping

警告:任何包含“fff”的命令都将被阻止。

## 使用方法

首先

```bash
cargo build
cargo run --bin server 
```

然后通过在另一个终端中尝试这四个操作

```bash
cargo run --bin client [cmd]
```
