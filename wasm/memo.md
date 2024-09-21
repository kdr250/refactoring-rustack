## wasmプロジェクト作成
`cargo install cargo-generate`<br>
`cargo generate --git https://github.com/rustwasm/wasm-pack-template`

## wasm-packのビルド(Web)
`wasm-pack build --target web`

## ブラウザで確認
`python3 -m http.server 8080 -d pkg`
