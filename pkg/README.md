# hello_wasm
hello wasm

两点需要注意：

1. 安装 wasm-pack

```shell
cargo install wasm-pack --no-default-features # 忽略 OpenSSL
```

2. 构建时的 target

```shell
wasm-pack build --target web
```