# if-else 與 loop

## if else

if else 與一般的程式語言差異不大
比較特別的點是 rust 中可以將 if else 的中的 block 直接 return block 的運算結果並賦值
```rust
fn main() {
    let n = 5;
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
}
```
只是要注意 return 的 type 必須相同

## loop

Rust 的 loop 相較一般語言的 `for` 差異也不大
只是在 loop 中並不需要額外的條件式
預設就是進行無限循環, 相當於 `while true`
而因此所有的 loop block 中都必須有一個 `break` 來中止循環

另外在多層的 loop 中也可以利用 `'label` 來直接 `break` 對應的 loop
```rust
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
```
這點對於有時要從深層的 for loop 中退出的程式很友善

與 if else 相同的
rust 的 loop 也可以直接對變數賦值
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
有點類似於其他 script langaunge 中的 early return
只是是利用 `break` 來達成

## while

毫無反應, 就只是個 `while`

## for loop

毫無反應, 就只是個 `for`....?

Rust 在對物件進行迭代時
可以呼叫 `.iter()` 和 `.into_iter()`
`.iter()` 與其他語言的 `foreach` 類似
而 `.into_iter()` 則類似於 pop, 被使用過的 item 會從原物件中移除
於最後的 `.iter_mut()` 則可以回去修改原物件中的 item

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
```

```rust
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```

### range

Rust 中也直接的提供 range 功能
```rust
let range = 1..10
```
這樣即可產生 1 到 10 的物件