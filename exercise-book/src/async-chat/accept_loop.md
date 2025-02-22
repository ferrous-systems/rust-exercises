## Writing an Accept Loop

Let's implement the scaffold of the server: a loop that binds a TCP socket to an address and starts accepting connections.

First of all, let's add required import boilerplate:

```rust,ignore
# extern crate tokio;
use std::future::Future; // 1
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, // 2
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream, ToSocketAddrs}, // 3
    sync::{mpsc, oneshot},
    task, // 4
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // 5
```

1. Import traits required to work with futures.
2. Import traits required to work with streams.
3. For the socket type, we use `TcpListener` from `tokio`, which is similar to the sync `std::net::TcpListener`, but is non-blocking and uses `async` API.
4. The `task` module roughly corresponds to the `std::thread` module, but tasks are much lighter weight.
   A single thread can run many tasks.
5. We will skip implementing detailed error handling in this example.
   To propagate the errors, we will use a boxed error trait object.
   Do you know that there's `From<&'_ str> for Box<dyn Error>` implementation in stdlib, which allows you to use strings with `?` operator?

Now we can write the server's accept loop:

```rust,ignore
# extern crate tokio;
# use tokio::net::{TcpListener, ToSocketAddrs};
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> { // 1
    let listener = TcpListener::bind(addr).await?; // 2

    loop { // 3
        let (stream, _) = listener.accept().await?;
        // TODO
    }

    Ok(())
}
```

1. We mark the `accept_loop` function as `async`, which allows us to use `.await` syntax inside.
2. `TcpListener::bind` call returns a future, which we `.await` to extract the `Result`, and then `?` to get a `TcpListener`.
   Note how `.await` and `?` work nicely together.
   This is exactly how `std::net::TcpListener` works, but with `.await` added.
3. We generally use `loop` and `break` for looping in Futures, that makes things easier down the line.

Finally, let's add main:

```rust,ignore
# extern crate tokio;
# use tokio::net::{ToSocketAddrs};
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
# Ok(())
# }
#
#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}
```

The crucial thing to realise is that in Rust, unlike in other languages, calling an async function does **not** run any code.
Async functions only construct futures, which are inert state machines.
To start stepping through the future state-machine in an async function, you should use `.await`.
In a non-async function, a way to execute a future is to hand it to the executor.
