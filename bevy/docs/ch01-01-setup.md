# セットアップ

> Ref: https://bevyengine.org/learn/book/getting-started/setup

早くゲームを作りたい気持ちはわかりますが、まずはちょっとしたセットアップをする必要があります。

### Rustのセットアップ

Bevyのアプリとエンジンのコードは全てRustで書かれています。
つまり始める前に、Rustの開発環境を整える必要があります。

### Rustをインストール

BevyはRust言語とコンパイラの改良に大きく依存しています。
そのため、MSRV(Minimum Supported Rust Version)はRustの「最新の安定したリリース」となっています。

[Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started)に従って、Rustをインストールしましょう。

うまくいけば、`rustc`コンパイラと`cargo`ビルドシステムがパスにインストールされているはずです。

### OSの依存関係をインストール

**Linux**

[Linux Dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)の説明に従ってください。

**Windows**

- [Visual Studio 2019 build tools installer](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio)を走らせます。（リンクに飛ぶと自動で`Visual Studio`がダウンロードされます）
- インストーラで「`C++`によるデスクトップ開発」を選択します。
- そして以下の手順に従います。
   1. インストーラで、「個々のコンポーネント」に移動する。
   2. 現在使用しているパソコンのアーキテクチャと、Windowsのバージョンに対応した最新の`MSVC`を選択する。
   3. Windowsのバージョンに対応した最新のWindows SDKを選択する。
   4. `C++ CMake tools for Windows`コンポーネントを選択する。
   5. 最後にコンポーネントをインストールする。

**MacOS**

`Xcode`のコマンドラインツールを`xcode-select --install`でインストールするか、`Xcode`アプリをインストールします。

### コードエディタとIDE

コードエディターはなんでも構いませんが、[rust-analyzer](https://github.com/rust-lang/rust-analyzer)プラグインを搭載したものをお勧めします。
まだ開発中ですが、すでにトップクラスのオートコンプリートとコードインテリジェンスを提供しています。

### Rustの教材

本書の目的はBevyを学ぶことなので、Rustについては紹介しません。
Rust言語について学びたい場合は、以下の教材をご覧ください。

- [The Rust Book](https://doc.rust-lang.org/book)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example)

### Bevyプロジェクトを作成する

これでBevyプロジェクトをセットアップする準備ができました！
Bevyは通常のRustの依存関係に過ぎません。
既存のRustプロジェクトに追加することも、新しく作成することもできます。
念の為、ゼロから始めると仮定します。

#### Rustを動かすプロジェクトを作成する

まずは新しいプロジェクトを作成するためのフォルダに移動し、以下のコマンドを実行してプロジェクトを作成します。

```bash
cargo new my_bevy_game
cd my_bevy_game
```

正しくプロジェクトが作成されているか、`cargo run`を実行して確認してみましょう。

#### Bevyを依存関係に追加する

以下のコマンドを実行して`bevy`を依存関係に追加します。

```bash
cargo add bevy
```

しっかり追加されているか、`Cargo.toml`ファイルをみてみましょう。

#### Cargoワークスペース

Cargoワークスペースを使用している場合は、`Cargo.toml`にリゾルバを追加します。

```toml
[workspace]
resolver = "2" # Important! wgpu/Bevy needs this!
```

#### パフォーマンス最適化でコンパイルする

単純なプロジェクトでは問題にならないかもしれませんが、Rustのデバッグビルドは非常に時間がかかります。
特にBevyを使ったゲームなど。

デフォルトの設定を使ったデバッグビルドでは、大きな3Dモデルの読み込みに数分かかったり、単純なシーンのフレームレートが再生不可能に近いレベルまで落ちたりすることも珍しくありません。

幸いなことに、Cargo.tomlに以下の記述をするだけで簡単に修正することができます。

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

リリースモードで開発することも考えられますが、リコンパイルが遅くなったり、デバッグに役立つシンボルやアサーションが使えなくなったりして、開発環境が悪化する可能性があるため、お勧めしません。

#### 高速コンパイルを有効にする

Bevyは安定したRust上でデフォルトの設定を使用しても問題なくビルドすることができます。
しかし反復コンパイルを最大限に高速化するには、以下の構成を推奨します。

**Bevyのダイナミックリンク機能を有効にする**

これは最もインパクトのあるコンパイル時間の短縮です！
bevyが依存関係にある場合、"dynamic_linking"機能フラグでバイナリをコンパイルすることができます。

```bash
cargo run --features bevy/dynamic_linking
```

実行のたびに`--features bevy/dynamic_linking`を追加したくない場合は、Cargo.tomlで恒久的に設定することができます。

```toml
[dependencies]
bevy = { version = "0.10.1", features = ["dynamic_linking"] }
```

注意として、ゲームをリリースする前に元に戻す必要があります。
そうしないと、ゲームを実行したい場合、`libbevy_dylib`をゲームと一緒にインクルードしていまします。

**LLD Linker**

Rustコンパイラは「リンク」に多くの時間を費やします。
LLDは、Rustのデフォルトのリンカーよりも高速です。
LLDをインストールするには、以下のOSに対応するコマンドを実行します。

- Ubuntu: `sudo apt-get install lld`
- Arch: `sudo pacman -S lld`
- Windows: `cargo install -f cargo-binutils && rustup component add llvm-tools-preview`
- MacOS: `brew install llvm`

**Alternative - mold linker**

moldはLLDより最大5倍高速ですが、プラットフォームのサポートに制限があり、時々安定性に問題があるなどの注意点があります。
`mold`をインストールするには、以下のOSに対応するコマンドを実行します。

- Ubuntu: `sudo apt-get install mold`
- Arch: `sudo pacman -S mold`
- Windows: わからない
- MacOS: `brew install mold`

インストールできたら、`.cargo/config.toml`にてCargoに以下の設定を追加する必要があります。

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

また`bevy/dynamic`を無効にすると、性能が向上する場合があります。

**Nightly Rust Compiler**

このコンパイラは、最新のパフォーマンス改善と、不安定な最適化を利用することができます。

追加するには`Cargo.toml`に以下の記述を行います。

```toml
[toolchain]
channel = "nightly"
```

詳細は、[The rustup book: Overrides](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)をご覧ください。

**Generic Sharing** 

クレートがジェネリックコードを複製する代わりに、単型化されたジェネリックコードを共有できるようにします。
場合によってはジェネリックコードをプリコンパイルして、反復コンパイルに影響を与えないようにすることもできます。
この機能は**Nightly Rust**でのみ利用可能です。

高速コンパイルを有効にするには、`nightly rust compiler`と`LLD`をインストールします。
そして`.cargo/config.toml`ファイルに上記で紹介したコードを記述します。
このガイドのプロジェクトでは、`my_bevy_game/.cargo/config.toml`に記述します。

何か問題が発生した場合は、[troubleshooting section](https://bevyengine.org/learn/book/troubleshooting/)か、[Discord](https://discord.com/invite/gMUk5Ph)で助けを求めましょう。

### Bevyをビルド

ここでもう一度`cargo run`を実行してください。Bevyの依存関係が構築を開始するはずです。
これは基本的にエンジンをゼロから構築することになるので時間がかかります。
完全な再構築を行うのは一度だけなので、移行のビルドは高速になります！

これでBevyプロジェクトのセットアップが完了したので、次からはBevyアプリを作っていきます。
