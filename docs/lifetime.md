## Lifetime

上一篇有提到值的**存活時間**，因為Rust在一個作用域結束後，會自動Drop所有在內的變數，所以在編寫程式碼時有使用到**引用**就需要非常注意變數的存活時間。

```
fn main() {
    let x: &Vec<i32>;
    {
        let y = vec![111];

        x = &y;// y doesn't live long enough
    }// y drop here

    println!("{:?}", x);// borrow later used here
}
```

### 為什麼需要標注生命週期

在其他語言當中，stack或是heap中數據的存活時間是不固定的，因此會需要開發者手動維護(C)或是程式執行時額外做處理(Garbage collection)。在Rust當中，heap中數據的存活時間預設跟stack中的數據是一樣的，因此在同個作用域底下編譯器可以清楚知道**被引用**的存活時間有沒有大於**引用**。

當一個函式有使用到引用的參數時，編譯器為了確保**被引用的值**的存活時間大於等於**引用的值**，會要求開發者手動標注參數生命週期。


```
fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");

    let res = max(&s1, &s2);

    println!("{}", res);
}

fn max(s1: &str, s2: &str) -> &str { // missing lifetime specifier
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
```

上述程式碼`fn max`出現編譯器的提示需要標注生命週期，那為什麼會這樣呢？因為編譯器不知道`res`這個變數引用的到底是`s1`還是`s2`，那編譯器就無從去檢查`s1`或`s2`是不是活得比`res`還要久。

```
fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");

    let res = max(&s1, &s2);

    println!("{}", res);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
```

我們將`max`補上`<'a>`代表這個函式會用到標示為`a`的生命週期，`s1`與`s2`的`&'a str`則代表這兩的變數活的時間都是`a`，回傳的`&'a str`則代表的是`res`會引用到的變數的生命週期。

做個小結論，只有參數為**引用**的才需要標示生命週期。回傳的值若標示為`'a`那麼傳入的參數的存活時間就必須大於`'a`，反過來說就是`s1`與`s2`變數的存活時間必須比`res`還要長。


```
fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");

    let res = max(&s1, &s2);

    println!("{}", res);
}

fn max<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2 // lifetime may not live long enough, consider adding the following bound: `'b:a`
    }
}

```
當然我們也可以兩個傳入的參數分別用不同兩個記號標記，但這時會看到編譯器建議我們在標注`'b`後面加上a，這個意思就是標示`b`的存活時間與`a`一樣長，那就符合剛剛說的**參數的生命週期必須大於等於回傳的生命週期** `b`必須大於等於`a`


### 省略標記生命週期

```
fn main() {
    let s1 = String::from("first");
    print_string(&s1)
}

fn print_string(s1: &str) {
    println!("{}", s1);
}
```
但從上述的例子可以發現，有些函式即使有用到引用的參數但我們也不必去標記生命週期，原因是因為Rust在最初版本中是需要每個引用都標記的，但Rust團隊發現，在某些情況下生命週期的標記其實是有規律的，因此只要符合照些條件在編譯的過程中Rust都會幫我們自動標上

1. 每個引用都有自己的生命週期('a, 'b)
2. 只有一個引用的參數
3. 如果有多個引用參數但其中一個是**&self**或是**mut self**

### 靜態生命週期
若我們需要一個變數的值貫穿整個程式(process)，常見的全域變數就是中之一，我們稱為**靜態生命週期**。

`let s1: &'static str = "hello";`

只需要在將標記的文字改成`static`即可。
