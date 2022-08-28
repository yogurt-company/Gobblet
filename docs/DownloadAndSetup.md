# DownloadAndSetup

## Install

### Mac and Windows
如果想在 Mac 或是 Windos 作業系統上安裝 Rust <br>
可以使用[官網](https://www.rust-lang.org/tools/install)的直接安裝即可

### Linux

但如果想安裝到 Linux 上則建議使用 (https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)
中所提到的
```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
```
`官網中的 -h 為顯示幫助資訊`

### Docker

當然 `Rust` 也可藉由 Docker 來啟動

```bash
docker run rust rustc
```

## Hello World

接下來可以試著建立第一個 Hello world 檔案

```rust
// hello_world.rs
fn main() {
    println!("Hello World!");
}
```

再來則輸入
```bash
rustc hello_world.rs
```
來編譯

最後就執行編譯出來的 `hello_world` bin 檔
`Hello World!`

### Tips

編譯時有可能會碰到`error: linker 'cc' not found`的錯誤<br>
這個是因為 Rust 在安裝的時候不會去檢查是否有相應的 C 編譯工具<br>
在 Debain 生態系的系統中可以去安裝
```bash
sudo apt update && sudo apt install build-essential -y
```
當然你也可以只安裝 gcc 作為最小安裝
```bash
sudo apt update && sudo apt install gcc -y
```

## Cargo

Cargo 為Rust專案提供了依賴包管理<br>
同時也可以存放一些專案相關的 `Meatdata`<br>
將原本的Rust專案搖身一變成為 Local 的 Package<br>

### Create Cargo Package

再來讓我們利用 Cargo 來建立新專案吧
```bash
cargo new hello_world_2
```

可以看到 Cargo 已經幫我們將基本的檔案與Cargo.toml建好了
```
.
├── Cargo.toml
├── .gitignore
└── src
    └── main.rs
```

### Directory Structure

空的Cargo.toml結構為此
```toml
[package]
name = "hello_world_2"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```
基本上就是一些 meta data

再來可以利用 `cargo build` 來打包專案<br>
build 結束後可以在 `./target/debug/hello_world_2` 中找到測試用的 bin 檔<br>
也可以使用 `cargo run` 來執行<br>
<br>
另外cargo要加上依賴檔的方式也很單純<br>
就直接在Cargo.toml 中的 `[dependencies]` 底下放想安裝的依賴就好<br>

```toml
[package]
name = "hello_world_2"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.4"
```

再跑 `cargo build` cargo 就會自動的安裝相關依賴<br>
之後會產生一個 cargo.lock 來做 lock file<br>

```
.
├── Cargo.toml
├── Cargo.lock
├── target
|   └── debug
|       └── hello_world_2  <==== 編譯結果
├── .gitignore
└── src
    └── main.rs
```

最後只要執行 `./target/debug/{PACKAGE.NAME}` 的檔案即可測試編譯結果
