## Cargo
Rustのビルドシステム、パッケージマネージャ

```bash
$ cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)
```

プロジェクトの作成
```bash
$ cargo new hello_cargo --bin
     Created binary (application) `hello_cargo` package
$ cd hello_cargo
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

ビルド
```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (/Users/dev/lessons/lesson-rust/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89s
```

ビルドと実行
```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

クリーン
```bash
$ cargo clean
```

ドキュメント生成
```bash
$ cargo doc --open
    Checking libc v0.2.105
    Checking rand v0.4.6
    Checking rand v0.3.23
 Documenting guessing_game v0.1.0 (/Users/dev/lessons/lesson-rust/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.21s
     Opening /Users/dev/lessons/lesson-rust/guessing_game/target/doc/guessing_game/index.html
```
