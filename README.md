# refactoring-rustack
書籍『Rustで作るプログラミング言語 - コンパイラ／インタプリタの基礎からプログラミング言語の新潮流まで』第2章のスタックベース仮想マシンを自分なりにリファクタリングして学習するためのレポジトリ

↓GitHub Pages<br>
https://kdr250.github.io/refactoring-rustack/

## 動作確認
`cargo run scripts/if.txt`<br>
-> 10 を出力する

`cargo run scripts/function.txt`<br>
-> 20 100 5 を出力する

`cargo run scripts/recurse.txt`<br>
-> 3628800 を出力する

`cargo run scripts/fibonacci.txt`<br>
-> 55 を出力する

`cargo run scripts/while.txt`<br>
-> 100 を出力する

## ブラウザで動作確認
`cd wasm`<br>
`npm install`<br>
`npm start`

## 参考URL
- [GitHub - rustack](https://github.com/msakuta/rustack)
- [技術評論社 Rustで作るプログラミング言語 - コンパイラ／インタプリタの基礎からプログラミング言語の新潮流まで](https://gihyo.jp/book/2024/978-4-297-14192-9)
