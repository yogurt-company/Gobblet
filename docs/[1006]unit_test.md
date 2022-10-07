# Unit Test 超簡略入門
該說近代的程式語言已經把unit test作為標準庫的一環. rust 當然不意外的有相應的unit test的機制. 

rstest是我們
``` rust
[dependencies]
rstest = "0.15.0"
```
相信應該大多數的工程師都或多或少聽聞過unit test. 但在那之前先想想這幾兩個特點:

> Unit test 可以幫助我們抓出bug

> Unit test 可以幫助我們測試各式各樣edge case

上述兩個問題其實沒有真正的答案, 甚至會在公司看到不同的作法. 但筆者私心覺得這兩個問題都是 **No**. 才會是一個好的unit test.
unit test 需要的只是測試看看程式是不是照著我們的預期走, 而不是作為窮舉各式各樣狀態的測驗function.

# Triple A 原則
Arrange, Act, Assert.
一個單元測試必須包含的東西

- Arrange, 對於測試的環境進行設定, 例如: 建立一個空的vector, 或是建立一個空的hashmap
- Act, 對於測試的環境進行操作, 例如: 對vector進行push, 或是對hashmap進行insert
- Assert, 對於測試的環境進行驗證, 例如: 檢查vector的長度, 或是檢查hashmap的key是否存在. assert 中文是斷言, _我認定這就是對的啦_



```rust
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let a = 1;
        let b = 2;
        let expected_result = 3;//Arrange
        let result = bad_add(a, b);//Act
        assert_eq!(result,expected_result);//assert
    }
}
```

# 實務上的unit test

用三個工程師都可坐到位置講講 unit test

- 程式主創
- 輔助/維護程式的工程師
- 讀者


## 程式主創
最高效寫程式, 其中一個希望達到境界就是可以加快try & error. 如果build code時間要達到數分鐘/數小時. 就代表一個開發的週期能做的嘗試很有限.
如果在開發的過程中會摸到database, 非stateless就有可能要考慮和同事之前的互相妨礙(?!). 這也是Unit test.

如果寫python, ds 出身的工程師, 會有習慣在jupyter上測試部分function的習慣. 轉移到靜態語言, 像是rust只有單一入口並沒有提供很好repl的程式語言, 就很適合透過unit test把開發過切割成可信任的小單元.


## 輔助/維護程式的工程師
[PLACE HOLDER]


## 讀者
就算有再好的程式套件維護系統, 如果不能完全弭平硬體差異, 還是會有git clone 完沒法完全試run的情況. 這時候unit test就是一個測試環境安裝的好方法. 
而且不得不說在machine learning領域, 會有部分的作者其實有手工的部分，不能復現結果.

以實際經驗來說，如果看到一個repository裏頭有測試相關的程式碼, 可以算是保證了基本的工程能力．
另外一個觀點來說, unit test 應該是try run 足夠小的邏輯. 自然也是一個絕佳的閱讀/改code的切入點



# Test Doubles
Why we need test doubles?

何謂unit 就是希望可以不仰賴外部任何額外的資源或是造成side effect. 這是一個保證，你必須確定你現在run test不會因為某一個人把db關掉就不能跑test了.
所以在unit test裡面，你會看到很多的test double. 這些test double的目的就是要模擬一個外部的資源. 

反正一搜尋一大把doubles, 例如 Dummy Object, Test stub, Test Spy, Mock object, Fake object.. etc 
感覺一上來就會被各式各樣的名詞轟炸. 我們先暸解三種就好. 



## Mock
[PLACE HOLDER]
## Stub
[PLACE HOLDER]
## Fake
[PLACE HOLDER]
# rstest
[PLACE HOLDER]
## fixture
[PLACE HOLDER]


