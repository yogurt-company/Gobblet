#　泛型

泛型是一種可以將不同的型別的變數一併處理的寫作方式
在特定的應用場景下能夠有效的減少重複的程式碼
當然在強型別語言中使用這類特性需特別小心

## 基本用法

以下實作將 `i32` 與 `char` type 的 vec 取最大值的 function 時
其實兩段 function 中的內容幾乎雷同

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("最大數字為 {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("最大字元為 {}", result);
    assert_eq!(result, 'y');
}
```

此時使用泛型的型別即可將二者合而為一

```rust
// PartialOrd, Copy 是做為限制可以傳入泛型中的型別, 必須要滿足同時具有 PartialOrd 和 Copy 實作的型別
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("最大字元為 {}", result);
}
```

而 Rust 的編譯器實際上在處理泛型時, 稱之為 `單態化（monomorphization`
會將有使用到泛型 function 的型別進行枚舉, 並將結果填入實作中
所以也不用擔心使用泛型會導致拖慢了執行速度

## 多類型

作為泛型代替型別用的 `T` 在編譯時會被單態化為某個特定型別
所以理所當然的, `T` 不能在同一次呼叫中被轉化為不同的型別
例如:
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // 此行將導致編譯錯誤
    let wont_work = Point { x: 5, y: 4.0 };
}
```

所以我們也必須用另外一個泛型型別來表示
當然, 如果兩個泛型型別同為一個型別是允許的
```rust 
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```