# AWS Lamdba

Rust 也可以使用 AWS Lamdba 來執行
而 Lamdba 無伺服器的屬性在 Rust 高效的運算速度下更顯優勢
以下就幾種將 Rust 程式碼部屬至 Lamdba 的方法進行介紹

*以下所有執行環境皆以 AWS Liunx 2 為範例*

## lambda-runtime

在介紹部屬之前
要先介紹的是專屬於 Rust 的 `lambda-runtime` 套件

[lamdba-runntime](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime)

`lambda-runtime`套件主要是為 Rust 開發的程式提供在 Lamdba 中執行的庫
可在 `Cargo.toml` 中載入 `dependencies` 來使用

```toml
[dependencies]
lambda_runtime = "0.6.1"
```

除此之外也需要提供一些額外的 `dependencies` 讓其相依的庫可以正常運作

```toml
[dependencies]
lambda_runtime = "0.6.1"

# serde 用於 serializing / deserializing
serde = "^1"
serde_json = "^1"
serde_derive = "^1"

# 用於處理 log
log = "^0.4"
simple_logger = "^1"
```

此外在 `Cargo.toml` 的頂部的 `package` 也需要加入以下內容

```toml
[package]
autobins = false
```

而`Cargo.toml`的底部則需要補上
```toml
[[bin]]
name = "bootstrap"
path = "src/main.rs"
```

至此完整的 `Cargo.toml` 如下

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"
autobins = false


[dependencies]
lambda_runtime = "0.6.1"
serde = "^1"
serde_json = "^1"
serde_derive = "^1"
log = "^0.4"
simple_logger = "^1"

[[bin]]
name = "bootstrap"
path = "src/main.rs"
```

## AWS Cli 部屬

再來我們就先來產生我們的 `Hello world` 專案
```rust
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
```

*範例中利用 `tokio` 來建立 async 的程式, 加入以下 `dependencies` 到 `Cargo.toml` 中*
```toml
tokio = "1.21.2"
```

接著先用 `cargo` 來打包 bin 檔
```bash
cargo build --release --bin
```

再將結果壓縮成 zip 檔
```bash
zip -j lambda.zip target/release/bootstrap
```

最後就可以使用 `aws` cli 來部屬了
```bash
aws lambda create-function --function-name rustTest \
  --handler bootstrap \
  --zip-file fileb://./lambda.zip \
  --runtime provided.al2 \
  --role arn:aws:iam::XXXXXXXXXXXXX:role/test-lamdba-rust-role \
  --environment Variables={RUST_BACKTRACE=1} \
  --tracing-config Mode=Active
```

最後 return sucess 就代表部屬成功了
再來我們就可以來測試一下 Lamdba 是否正常運作
```bash
/usr/local/bin/aws lambda invoke  --cli-binary-format raw-in-base64-out   --function-name rustTest   --payload '{"command": "Say Hi!"}'   output.json
```

最後檢查一下 output.json 的內容
```bash
cat output.json
```
這樣就完成佈署了!
