## wasmプロジェクト作成
`cargo install cargo-generate`<br>
`cargo generate --git https://github.com/rustwasm/wasm-pack-template`

## wasm-packのビルド(Web)
`wasm-pack build --target web`<br>
=> wasm_bg.wasm、wasm_bg.wasm.d.ts、wams.d.ts、wasm.jsなどを生成してくれる
=> index.html、index.js、main.jsは自分で書いた

## ブラウザで確認
`python3 -m http.server 8080 -d pkg`
