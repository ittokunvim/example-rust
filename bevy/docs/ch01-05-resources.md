# リソース

> Ref: https://bevyengine.org/learn/book/getting-started/resources

エンティティやコンポーネントは、複雑で問い合わせ可能なデータ群を実現するのに適しています。
しかしほとんどのアプリケーションでは、ある種のグローバルにユニークなデータも必要になります。
Bevy ECSでは、グローバルにユニークなデータを、リソースを使って表現します。

Resourcesとしてエンコードされるデータは以下のとおりです。

- Elapsed Time（経過時間）
- Asset Collections (sounds, textures, meshes)
- Renderers

### リソースで時間を把握する

前節の`hello spam`問題を解決するために、"hello"を2秒に1回表示させるようにしましょう。
Timeリソースは、`add_plugins(DefaultPlugins)`で自動的に追加されます。

始める前に、`hello_world`システムを削除します。これで`greet_people`システムだけを適応させることができます。

リソースのアクセスはコンポーネントへのアクセスとほぼ同じです。
システム内のTimeリソースには、以下のようにアクセスできます。

```rust
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
```

`Res, ResMut`ポインタは、リソースへの読み取りおよび書き込みアクセスを提供します。

`Time`の`delta`フィールドは、最後の更新から経過した時間を教えてくれます。
しかし、2秒に1回システムを実行するためには、一連の更新の間に経過した時間を追跡する必要があります。
これを簡単にするために、`Timer`型を提供しています。`Timer`で経過時間を追跡するシステムのために、新しいResourceを作成しましょう

```rust
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
```

あとは`HelloPlugin`に`GreetTimer`リソースを追加して、`TimerMode::Repeating`を使用して、タイマーを繰り返すようにします。

```rust
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}
```

これでプログラムを実行すると、2秒ごとに生成されたPersonがターミナルに表示されます。
