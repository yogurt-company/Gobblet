# 並發

在 Rocket 中處理 Request 時
是使用 Rust `Future` 來實現並發處理的
可以在 `Routing` 使用 `async` 或是 `Future` 來處理 Request

以下就來簡單介紹 Rust 中並發的概念

## 概念

Rust 中的並發同時支援 `multi-threaded` 與 `single-threaded`(async)
* multi-threaded
    * 透過建立新的 `thread` 來處理並發可以簡單而且輕鬆的快速使用, 且不須對原有程式進行大幅的修改
    * 但是建立 `thread` 會有一些開銷, 且在處理大量的並發時會有一些問題
* single-threaded (async)
    * 透過 `async` 來實現並發在大量的請求時會有更佳的性能表現, 尤其當並發的內容是在等待其他的資源時(例如: 等待資料庫回應)
    * 但是在當發生錯誤時可能會難以處理, 且須大量的修改原有的程式
在一般情況中, 會建議使用 `thread` 來處理並發
但需要考量性能的場景下, 則可以考慮使用 `async` 來處理並發

以下提供官網上的兩個範例來比較 `thread` 與 `async` 的在 coding 上的差異

同步情況下的下載檔案
```rust
fn get_two_sites() {
    // Spawn two threads to do work.
    let thread_one = download("https://www.foo.com");
    let thread_two = download("https://www.bar.com");
}

```

改成建立新的 `thread` 來處理, 只需把 `download` 放進去 `thread` 中即可
```rust
fn get_two_sites_thread() {
    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
```

使用 `async` 來處理, 則需要修改使用的 function
```rust
async fn get_two_sites_async() {
    // Create two different "futures" which, when run to completion,
    // will asynchronously download the webpages.
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // Run both futures to completion at the same time.
    join!(future_one, future_two);
}
```

## Future

在 Rust 中已經具備了 `async` 與 `await`
而當使用 `async fn` 時, 會回傳一個 `Future` 物件

而 `Future` 不是預載在 Rust 中, 需要在 `Cargo.toml` 中加入 `futures` 來使用
```toml
[dependencies]
futures = "0.3"
```

當 `Future` 被建立時, 其實並不會立即執行, 直到他被 `polling`
這意味著在 Rust 中使用 `async` 時並不一定真的會占用該 `thread` 去執行其中的程式
直到我們有需要他的時候, 一般來說是使用 `await` 來等待 `Future` 的執行結果
此時才真正的會開始消耗資源

但必須注意的是當我們呼叫 `await` 時, 會將目前的 `thread` 交出去, 並等待 `Future` 的執行結果
而使用的 `thread` 則不一定是當時建立 `Future` 的 `thread`

```rust
async fn get_book_and_music() -> (Book, Music) {
    let book = get_book().await;
    let music = get_music().await;
    (book, music)
}
```

而 `Future` 的特性有時也會造成一些麻煩
例如以上的程式碼片段
實際上在 `get_music().await` 完成前
並不會一起執行 `get_book().await`
除非我們使用 `join!`

```rust
use futures::join;

async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}
```

這樣子 `get_book().await` 與 `get_music().await` 就會一起執行了
