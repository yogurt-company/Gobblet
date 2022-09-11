## borrow mutable case

在Rust中能調整變數是否可以被更改，只要在變數前面加個`mut`即可。

`let mut x = String::from("hello");`

但因為有引用的關係，會變得讓人難以理解，這邊整理了各種情況，來幫大家一一釐清。

| case | description |
| ----------- | ----------- |
| a: &T | 不能修改a的指向，也不能修改a所指向的數據 |
| mut a: &T | 可以修改a指向到別的數據，但不能修改原本指向的數據 |
| a: &mut T | a不能修改指向到別的數據，但可以修改a原本指向的數據 |
| mut a: &mutT | 可以修改a的指向，也可以修改a原本指向的數據 |

### 第一種狀況`a: &T`
這個狀況即是**指向**或**原始指向的資料**都不能修改
```
fn main() {
    let mut x1 = String::from("hello");
    let x2 = String::from("hello2");

    let y = &x1;

    y = &x2;
    // 當我們想要改變y指向x2時會出現錯誤
    // cannot assign twice to immutable variable

    *y = String::from("hello3");
    // 或是想要改變x1的值也會出現錯誤
    // cannot assign to `*y`, which is behind a `&` reference `y` is a `&` reference, so the data it refers to cannot be written

    println!("{}", y);
}
```

### 第二種狀況 `mut a: &T`
這個是可以改變a的指向，但原始資料不能被改變。
以第一個例子來說，只要在`y`前面加讓`mut`就可以通過了。

```
fn main() {
    let mut x1 = String::from("hello");
    let x2 = String::from("hello2");

    let mut y = &x1;

    y = &x2; // 錯誤消失

    *y = String::from("hello3");
    // 或是想要改變x1的值也會出現錯誤
    // cannot assign to `*y`, which is behind a `&` reference `y` is a `&` reference, so the data it refers to cannot be written

    println!("{}", y); // hello2
}
```

### 第三種狀況 `a: &mut T`
與第二個例子相反，只能改變原始的數據，不能改變`y`的指向
```
fn main() {
    let mut x1 = String::from("hello");
    let x2 = String::from("hello2");

    let y = &mut x1;

    y = &x2; // mismatched types expected mutable reference `&mut String` found reference `&String`
    *y = String::from("hello3");

    println!("{}", x1); // hello3
    // 這邊注意是印出x1的資料而不是y，因為我們要確認是不是有修改到x1的資料
}
```

### 第四種狀況 `mut a: &mut T`
既可以修改a的指向，也可以修改a指向的原始資料。
```
fn main() {
    let mut x1 = String::from("hello");
    let mut x2 = String::from("hello2");

    let mut y = &mut x1;

    *y = String::from("hello3");
    y = &mut x2;

    println!("{}", x1); // hello3
    println!("{}", y); // hello2
}
```

但這邊要注意，是先**修改原始資料再改指向**還是**先改指向再修改原始資料**，結果會不一樣喔。
 
