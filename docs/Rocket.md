# Rocket

Rocket 是一個基於 Rust 的 Web 框架
這個框架非常的精簡, 主要任務就是監聽 Request 後派發給 商務邏輯
基本上只處理以下四件事情, 同時也是 Rust 的生命週期:
* Routing
  * 決定 Request 要導向哪個 marco function
* Validation
  * 驗證 Request 與身分認證
* Controller
  * 主要的商務邏輯區塊
* Response
  * 處理 Response

## install 

利用 `cargo` 快速的 setup Rocket

```bash
cargo new hello-rocket
cd hello-rocket
```
然後再其中的 `Cargo.toml` 中加入 Rocket 的 dependencies Package

```toml
[dependencies]
rocket = "0.5.0-rc.2"
```

最後再下
```bash
cargo install --path .
```
安裝依賴包
之後就可以開始使用 Rocket 了!

## Hello World

Rocket 使用 `macro_use` 來導入
```rust
#[macro_use] extern crate rocket;
```

接下來我們就來寫下第一個 `hello world` 的 routing 吧

```rust
#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}
```

不過只有 routing 是沒辦法啟動 rocket 的
所以最後要加上 `launch` 的 macro 來啟動她
```rust
#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![world])
}
```

接下來就可以利用 `cargo run` 來啟動 Rocket 了!

```bash
cargo run
```

通常 default stdout 應該是長這樣子

```
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: [..]
   >> keep-alive: 5s
   >> limits: [..]
   >> tls: disabled
   >> temp dir: /tmp
   >> log level: normal
   >> cli colors: true
🛰  Routes:
   >> (world) GET /hello/world
🚀 Rocket has launched from http://127.0.0.1:8000
```

這個時候就可以來驗證一下是否有成功
```bash
curl 127.0.0.1:8000/hello/world
```

### launch

`#[launch]` 可以簡易的啟動 Rocket
不過他沒辦法處理當啟動時發生的意外, 或是 `launch` 啟動後的 return
所以 Rocket 也可以利用 `#[rocket::main]` 來啟動 server

```rust
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await?;

    Ok(())
}
```

### Fairings

與 `launch` 關係密切的是 `fairings` trait
他類似於其他 web framework 中的 middleware
不過有些限制
* `fairings` 不能直接 return 或是 終止 Request
* `fairings` 不能在 Request 中加料
* `fairings` 可以阻止 `launch` 的啟動
* `fairings` 可以在啟動時修改 `config` 等配置

`fairings` trait 中一共有5種 function
分別在不同的時間點調用
* on_ignite
  * 在啟動時調用
* on_liftoff
  * 在啟動後調用
* on_request
  * 在 Request 進入時調用
* on_response
  * 在 Response 產生時調用
* on_shutdown
  * 在關閉時調用

而每個 `Fairings` 都必須有一個 `Info` 的實作
```rust
fn info(&self) -> Info {
    Info {
        name: "Example Fairing",
        kind: Kind::Ignite | Kind::Liftoff | Kind::Request | Kind::Response | Kind::Shutdown
    }
}
```
其中 `kind` 中表示 `Fairings` 希望接收的回調的集合

以下是一個空的 `Fairings` 實作
```rust
use rocket::{Rocket, Request, Data, Response, Build, Orbit};
use rocket::fairing::{self, Fairing, Info, Kind};

#[rocket::async_trait]
impl Fairing for MyType {
    fn info(&self) -> Info {
        /* ... */
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        /* ... */
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        /* ... */
    }

    async fn on_request(&self, req: &mut Request<'_>, data: &mut Data<'_>) {
        /* ... */
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        /* ... */
    }

    async fn on_shutdown(&self, rocket: &Rocket<Orbit>) {
        /* ... */
    }
}
```
