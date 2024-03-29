# Rust Axum library

## new project

```bash
mkdir rust-library
cd rust-library
git config --global init.defaultBranch main
git init
touch Cargo.toml
```

Cargo.toml

```toml
[workspace]

members = [
    "rs_library",
]
```

use **cargo --list** to find more command

```bash
cargo new rs_library --bin
```

[It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

refer [new project](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)

```toml
[workspace]
resolver = "2"

members = [
    "rs_library",
    "rs_system",
]

```

```bash
cargo new rs_system --lib
```

library/Cargo.toml

```toml
[dependencies]
system = {path="../rs_system"}
```

library/src/main.rs

```rust
use system;

fn main() {
    let num = rs_system::add(1, 2);
    println!("Hello, world!{num}");
}
```

build and run

```bash
cargo build
cargo run -p rs_library
```

### auto-reload

[auto-reload](https://github.com/tokio-rs/axum/blob/main/examples/auto-reload/README.md)

### log

```rust
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {user_set, tx});
    
    let app = Router::new()
        .route("/", get(handler))
        .route("/listenfd", get(listenfd_handdler))
        .route("/websocket", get(websocket_handler))
        .route("/index", get(index))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)));
    
    ...

}
```

在Rust中，互斥锁（Mutex）是一种用于在多个线程之间共享数据的并发原语。互斥器提供了一种安全的方式，允许多个线程访问共享数据，但每次只允许一个线程进行写操作。
在并发编程中，数据竞争（Data Race）是一种常见的并发问题，可能导致不可预测的结果和不稳定的程序行为。互斥器的作用就是避免数据竞争，确保共享数据的安全访问。

[Rrc Mutex](https://zhuanlan.zhihu.com/p/523959791)

## WebSocket

传统的请求-响应模式的Web开发在处理此类业务场景时，通产采用实时通讯方案，常见的是：

- 轮询 polling：
  原理简单易懂，就是客户端通过一定的时间间隔以频繁请求的方式向服务器发送请求，来保持客户端和服务器的数据同步。问题很明显，当客户端以固定频率向服务器端发送请求时，服务器端的数据可能并没有更新，带来很多无畏请求，浪费带宽，效率低下。
- 基于Flash：AdobeFlash通过自己的Socket实现完成数据交换，再利用Flash暴露出相应的接口为JavaScript调用，从而达到实时传输目的。此方式比轮询要高效，且因为Flash安装率高，应用场景比较广泛，但在移动互联网终端上Flash的支持并不好。IOS系统中没有Flash的存在，在Android中虽然有Flash的支持，但实际的使用效果差强人意，且对移动设备的硬件配置要求较高。2012年Adobe官方宣布不再支持4.1+系统，宣告了Flash在移动端上的死亡。

我们需要一种高效节能的双向通信机制来保证数据的实时传输。在此背景下，基于HTML5规范的、有Web TCP之称的WebSocket应用而生。

WebSocket是HTML5一种新的协议。它实现了浏览器与服务器全双工通信，能更好的节省服务器资源和带宽并达到实时通讯，它建立在TCP之上，同HTTP一样通过TCP来传输数据。

## Profiles

Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.

refer [Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
