# ECS

> Ref: https://bevyengine.org/learn/book/getting-started/ecs

Bevyの全てのアプリのロジックは、Entity Component Systemパラダイムを使用しており、しばしばECSと略されます。
ECSは、プログラムを「Entity、Component、System」に分割するソフトウェアパターンです。
Entityはユニークな「もの」で、Componentのグループに割り当てられ、それをシステムで処理します。

例えば、あるEntityにはPositionとVelocityのコンポーネントがあり、別のEntityにはPositionとUIのコンポーネントがあります。
システムは、Componentタイプの特定のセットで実行されるロジックです。
PositionとVelocityコンポーネントを持つ全てのEntityで実行されるムーブメントシステムがあります。

ECSパターンは、アプリのデータやロジックをコアコンポーネントに分割することで、クリーンデデカップリングな設計を促します。
また、メモリアクセスのパターンを最適化し、並列処理を容易にすることで、コードを高速化することができます。

### Bevy ECS

Bevy ECSはBevyによるECSパターンの実装です。
他のRust ECSの実装では、`complex lifetime, traits, builder patterns, macros`が必要になることが多いですが、Bevy ECSでは、これらの概念に通常のRustデータ型を使用しています。

**Components** 

Component特性を実装したRust構造体

```rust
#[derive(Component)]
struct Position { x: f32, y: f32 }
```

**Systems**

通常のRust関数

```rust
fn print_position_system(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}
```

**Entities**

ユニークな整数を含むシンプルな型

```rust
struct Entity(u64);
```

では実際にどのような効果があるのか見ていきましょう！

### システム

以下のコードを`main.rs`に貼り付けます。

```rust
fn hello_world() {
    println!("hello world!");
}
```

これが最初のシステムになります。次はこれをアプリに追加します。

```rust
use bevy::app::App;

fn main() {
    App::new()
        .add_system(hello_world)
        .run()
}
```

`add_system`関数は、アプリの[Schedule](https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.Schedule.html)にシステムを追加しますが、後で説明します。

では`cargo run`を実行して見ましょう。ターミナルに`hello world!`と表示されるはずです。

### コンポーネント

全世界に挨拶するのもいいですが、特定の人物に挨拶する場合はどうすれば良いでしょうか。
ECSでは一般的に、人を定義する一連のコンポーネントを持つエンティティとしてモデル化することになります。
まずは`Persion`コンポーネントでシンプルに考えてみましょう。

以下の構造体を`main.rs`に追加します。

```rust
#[derive(Component)]
struct Persion;
```

しかし人物に名前をつける場合はどうすれば良いでしょうか。
伝統的な設計では、`Persion`に`name: String`でいいかもしれませんが、以下のようにすることで他のエンティティにも名前をつけることができます。
例えば犬にも名前があるはずです。コードの再利用を促進するために、データ型を細かく分割することは理にかなっている場合があります。
そこで`Name`を独自のコンポーネントにしてみましょう。

```rust
#[derive(Component)]
struct Name(String);
```

そして、"startup system"を使って、`People`を[`World`](https://docs.rs/bevy_ecs/latest/bevy_ecs/world/struct.World.html)に追加することができます。
"startup system"は通常のシステムと同じですが、アプリの起動時に一度だけ、他のシステムよりも先に実行されます。
それでは、[`Commands`](https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.Commands.html)を使って、`World`にエンティティを誕生させてみましょう。

```rust
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
```

そして以下のように"startup system"を登録します。

```rust
fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .run();
}
```

アプリを実行すると、`add_people`が実行された後に`hello_world`が実行されます。
次は新しく作成した人たちをきちんと迎えるシステムを作っていきます。

```rust
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
```

"system function"に渡すパラメータは、システムがどのようなデータに対して実行されるかを定義します。
`greet_people`は、`Person, Name`のコンポーネントを持つ全てのエンティティに対して実行されます。

上記のQueryは「`Person`コンポーネントを持つエンティティに対して、`Name`コンポーネントを全て繰り返し実行する」と解釈することができます。

最後のこのシステムをアプリに登録しましょう。

```rust
fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
```

プログラムを走らせると以下のような出力になります。素晴らしい！

```
hello world
hello Elaina Proctor!
hello Renzo Hume!
hello Zayna Nieves!
```

"hello world!"は、上記とは異なる順序で表示されるかもしれませんが、システムが並列で実行されるためです。
