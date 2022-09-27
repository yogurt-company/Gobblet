## config

Rocket 使用 `Rocket.toml` 來進行config 配置
`Rocket.toml` 可以 `ROCKET_CONFIG` 來指定 path
或是讓 Rocket 由工作目錄一路往上層查找

官方的 default `Rocket.toml` 配置
```toml
[default]
address = "127.0.0.1"
port = 8000
workers = 16
max_blocking = 512
keep_alive = 5
ident = "Rocket"
log_level = "normal"
temp_dir = "/tmp"
cli_colors = true
## NOTE: Don't (!) use this key! Generate your own!
secret_key = "hPRYyVRiMyxpw5sBB1XeCMN1kFsDCqKvBi2QJxBVHQk="

[default.limits]
form = "64 kB"
json = "1 MiB"
msgpack = "2 MiB"
"file/jpg" = "5 MiB"

[default.tls]
certs = "path/to/cert-chain.pem"
key = "path/to/key.pem"

[default.shutdown]
ctrlc = true
signals = ["term", "hup"]
grace = 5
mercy = 5
```

不過需要注意的是 Rocket 同時支援以環境變數作為 config 讀取
且 env 的優先級高於 `Rocket.toml` 中的配置

以下介紹幾個筆者覺得比較重要且特殊的 config 配置

### Secret Key

```toml
[default]
## NOTE: Don't (!) use this key! Generate your own!
secret_key = "hPRYyVRiMyxpw5sBB1XeCMN1kFsDCqKvBi2QJxBVHQk="
```

`Secret Key` 為加密 cookie 的金鑰
僅在啟用 `secrets` 這個 (crate feature)[https://crates.io/crates/secrets] 會生效
要注意如果是 `release` mode 中安裝了這個 feature 就必須事先配置好 Secret Key, 否則將無法啟動 Rocket server

如需產生 `Secret Key` 的話可以利用 `openssl`
```bash
openssl rand -base64 32
```

當然, 這把金鑰需妥善保管

### Limits

```toml
[default.limits]
form = "64 kB"
json = "1 MiB"
msgpack = "2 MiB"
"file/jpg" = "5 MiB"
```

與一般 web service 的不同
Rocket 的 limit 除了限制整個 form data / json 的上限外
也可以針對不同的 type 進行限制
有趣的是 from data 與 json 是分開來限制的
`Limit` 限制的值除了可以用 `unit based strings(KiB, MiB, GiB)` 表示外
也可以用位元的 `integers` 來表示

### 雙向 TLS

TLS 一般來說不太會直接掛到 web service 上
所以就跳過基本的TLS

不過 Rocket 中有個很有趣的 `mtls` 功能
再進行 TLS 握手時也會要求對方也出示證書
利用此一特點可以輕鬆的實作微服務之間的基本 `zero trust` 配置

首先需要啟用 `mtls` 功能
`Cargo.toml`
```toml
rocket = { version = "0.5.0-rc.2", features = ["mtls"] }
```

再來於`Rocket.toml`中配置
```toml
[default.tls.mutual]
ca_certs = "path/to/ca_certs.pem" # Path or bytes to DER-encoded X.509 TLS cert chain.
mandatory = true                  # when absent, defaults to false
```

配置完畢後就可以利用 `mtls` 來進一步的保護 Routing
```rust
use rocket::mtls::Certificate;

// 被保護的 Routing
#[get("/auth")]
fn auth(cert: Certificate<'_>) {
    // This handler only runs when a valid certificate was presented.
}
```
