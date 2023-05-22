# プラグイン

> Ref: https://bevyengine.org/learn/book/getting-started/plugins

Bevyの基本原則の1つはモジュール化です。
Bevyのエンジンの機能は、全てプラグインとして実装されています。
これにはレンダラーのような内部機能も含まれますが、ゲーム自体もプラグインとして実装されています。
このため開発者は必要な機能を選び取ることができます。
UIが必要なければUIPluginは登録しなくていいし、ヘッドレスサーバーを作るだけならRenderPluginは必要ありません。

これは気に入らないコンポーネントを自由に置き換えることができることを意味します。
自分自身の[UiPlugin](https://docs.rs/bevy_ui/latest/bevy_ui/struct.UiPlugin.html)を作ることはいいですが、もしそれが有用であるなら、それを[Bevyに還元する](https://bevyengine.org/learn/book/contributing)ことを検討してください！

しかしほとんどの開発者はカスタムエクスペリエンスを必要とせず、手間をかけずに「フルエンジン」エクスペリエンスを得たいだけです。
そのためにBevyでは「デフォルトプラグイン」のセットを提供しています。

### デフォルトプラグイン

`add_plugins(DefaultPlugins)`は、2D/3Dレンダラー、アセットのロード、UIシステム、ウィンドウ、入力など、多くの人がエンジンに期待する機能を追加します。

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
```

プログラムを実行すると2つのことに気がつくはずです。

**ウィンドウがポップアップする**

これはウィンドウインターフェイスを定義するWindowPluginと、OSのネイティブウィンドウAPIを使ってウィンドウを作成するwinitライブラリを使用するWinitPluginがあるためです。

**helloメッセージがいっぱいになる**

これはDefaultPluginsがアプリケーションに、イベントループを追加したためです。
アプリのECSスケジュールが「フレーム」ごとに1度ループで実行されるようになりました。

なお、以下のコードは上記のコードと等価です。

```rust
fn main() {
    App::new()
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        /* more plugins omitted for brevity */
        .run();
}
```

### プラグインを作成する

helloロジックを全てプラグインに移動し、整理してみましょう。
プラグインを作成するには、Pluginインターフェイスを実装する必要があります。

```rust
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
```

あとはHelloPluginにシステムを移すだけです。

```rust
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
```

もう一度アプリを実行してみてください。以前と全く同じように動作するはずです。
次のセクションでは、Resourcesを使用してhelloスパムを修正します。
