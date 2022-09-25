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
```rust
let counter = HashMap::new();

```
