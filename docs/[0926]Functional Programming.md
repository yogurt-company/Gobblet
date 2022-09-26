# Functional Programming

寫在如何使用 functional programming(之後都簡稱FP)之前， 我們得先來看看為什麼我們要這樣使用.
我們會發現，FP 的特性， 其實就是一種解題的思維，而不是一種程式語言，甚至稱不上syntax.


# declarative vs imperative 
但是首先得要引入兩個大詞. declarative vs imperative

舉一個生活化的譬喻．

我們是一個早餐店老版的老闆，如何指導員工做出一個合格的總匯三明治該怎麼做呢？
有兩種解題思路.

一種是把所有步驟, 諸如刀子該用哪一把, 備料該怎麼放. 都一步一步的寫出來. 完全鉅細靡遺的把所有過程都寫出來. 這就是imperative的思維.
另外一種就是我們只管做出第一個合格的總匯三明治, 然後把時間, 成本的標準制訂出來, 至於如何實現則透過員工自行處理. 這是declarative的思維.

> 如果員工經驗老到你會採取什麼方針?

應該會果斷的選擇第二種. 這也是偏向functional programming的思維.

# Pure function
在FP中，我們會把一個function稱為pure function, 如果它滿足以下條件
- 一個function的輸入，一定會有一個輸出
- 一個function的輸入，不會因為外部的變數而改變輸出

這邊舉一個反例. 我們就想要寫一個function去 count 出現元素的次數.
如果是python的玩家大概一開始會寫出這樣的東西:

```rust

use std::collections::HashMap;


fn get_word_frequency(mut word_frequency:HashMap<& str,u32>, word: & str) -> () {
    let count = word_frequency.entry(word).or_insert(0);
    *count += 1;
}

fn main(){
    let mut word_map:HashMap<&str,u32> = HashMap::new();
    let ex = ["a","b","c","a","b","a"];
    for c in ex {
        get_word_frequency(word_map,c);
    }

}
```
這個會造成非常多不同的錯誤. 尤其是處理沒有copy trait的物件.需要非常細緻的處理life time.
在我們花更多時間之前，讓我們來看看FP的解法.

```rust
use std::collections::HashMap;
fn main(){
    let ex = ["a","b","c","a","b","a"];
    let counter = ex.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    println!("{:?}", counter);
}
```
還季前面說的嗎? 由於rust compiler/optimizer是個有經驗的店員, 這邊透過ＦＰ的方式去寫，他會自動幫我們處理好所有的life time問題.

