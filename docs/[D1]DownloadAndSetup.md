這是一份 Rust的入門指南. 

我們從第一開始就是以 "你是個其他程式語言國度來的旅人" 作為前提展開這些課程. 實際上更可以說筆者是來自於JS/python大陸來的.
在這邊我們會假設
- 你已經有一定的程式基礎. 在我們談到某個rust feature 我們盡量會往其他語言的寫法去比較.
- 對於超級基本的電腦架構認知. 例如: stack, heap, memory allocation, pointer, etc.

但不論你來自何方, 都要從能跑第一個hello world 開始. 

# Install

## Mac and Windows
如果想在 Mac 或是 Windos 作業系統上安裝 Rust <br>
可以使用[官網](https://www.rust-lang.org/tools/install)的直接安裝即可

## Linux

但如果想安裝到 Linux 上則建議使用 (https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)
中所提到的
```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
```
`官網中的 -h 為顯示幫助資訊`

## Docker

當然 `Rust` 也可藉由 Docker 來啟動

```bash
docker run rust rustc
```

# Hello World

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

## Tips

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

# Cargo

Cargo 為Rust專案提供了依賴包管理, 同時也可以存放一些專案相關的 `Meatdata`, 將原本的Rust專案搖身一變成為 Local 的 Package

## Create Cargo Package

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

## toml 
toml檔案可以類推到python poetry幾乎一至的使用體驗. 幾個筆者所使用過的語言scala, python都可以找到類似的風格. 將版本依賴的library與實際資源該從哪裡入手的`.lock檔案`分開

```toml
[package]
name = "hello_world_2"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.4"
```

Cargo.toml 可以透過 ``cargo update`` 產生 Cargo.lock 來鎖定依賴包的版本. 最常見的流程是這樣子的:
1. 在[crate.io](https://crates.io/) 找到確實可以滿足需求的 3rd party library
2. 在 Cargo.toml 中的[dependencies]區塊中加入該library. 如上述範例的 rand
3. 執行 ``cargo update`` 來產生 Cargo.lock, 這點與python poetry的使用體驗也相似.
-----

再來可以利用 `cargo build` 來打包專案<br>
build 結束後可以在 `./target/debug/hello_world_2` 中找到測試用的 bin 檔, 也可以使用 `cargo run` 來執行

直接跑 `cargo build` cargo 就會自動的安裝相關依賴<br>
之後會產生一個 cargo.lock. 特別是在不同環境下開發時, 會希望能夠確保每個環境下的依賴都是一致的<br>

最後只要執行 `./target/debug/{PACKAGE.NAME}` 的檔案即可測試編譯結果
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
但其實 `cargo run` 會包含 `cargo build` , `cargo update` 的功能.

