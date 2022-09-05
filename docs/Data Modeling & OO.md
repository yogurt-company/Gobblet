# Modeling & OO
嘿嘿嘿 終於要進入AI的領域了嗎? 不! 這裡的建模並不是Machine learning 的建模.
而是更單純的將外在世界透過一個合理的方式做[抽象化](https://zh.wikipedia.org/wiki/%E6%8A%BD%E8%B1%A1%E5%8C%96) 和 繼承.

抽象化概念:
> 主要是為了只保存和一特定目的有關的資訊。例如，將一個皮製的足球抽象化成一個球，只保留一般球的屬性和行為等資訊。相似地，亦可以將快樂抽象化成一種情緒，以減少其在情緒中所含的資訊量。

Object oriented development 經常會跟建模牽扯在一起. 而class又幾乎和OOD一起出現. 這裡就來介紹一下class的概念.
class希望把行為和status作為一個概念的罐頭封裝起來. 將來再使用的情境中, 只要把罐頭拿出來就可以使用裡面的行為和status.

透過合理的**抽象化** 和 **繼承** 可以讓我們的程式碼更加的可讀, 可維護, 可擴充.

-----
我知道本篇的讀者絕對不是希望來這裡複習class的概念. 但是對於來自於js 和 python的旅人, 如果談到OO卻跟他們說
> 不 本店沒有提供任何class 和繼承.

[官方cook book](https://doc.rust-lang.org/book/ch17-00-oop.html)也可以找到
>Many competing definitions describe what OOP is, and by some of these definitions Rust is object-oriented, but by others it is not.

就連官方都不確定Rust是不是OO. 但是我們可以確定的是, Rust以及其他現代程式語言都必須具備封裝概念/ 繼承/ 可擴抽的特性.
# Struct / traits
最基礎的

# Enum


# impl

# Generic


# mod

# 後面碎碎念
來自於Python, JVM 世界的朋友感覺會對於這樣的OO design語法非常不適應. Go的解題思路跟這個也很像. 不希望你從 Is A 關係做直接使用.
Rust 提供了另外一個方便的Macro feature 去提升程式的復用度和抽象化.
