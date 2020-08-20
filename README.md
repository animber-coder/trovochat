# trovochat
[![Documentation][docs_badge]][docs]
[![Crates][crates_badge]][crates]
[![Actions][actions_badge]][actions]

This crate provides a way to interact with [Trovo]'s chat.

Along with parse messages as Rust types, it provides methods for sending messages.

It also provides an 'event' loop which you can use to make a bot.

### Runtime
This crate is runtime agonostic. To use..

| Read/Write provider                                        | Features                |
| ---                                                        | ---                     |
| [`async_io`](https://docs.rs/async-io/latest/async_io/)    |`async-io`               |
| [`smol`](https://docs.rs/smol/latest/smol/)                |`smol`                   |
| [`async_std`](https://docs.rs/async-std/latest/async_std/) |`async-std`              |
| [`tokio`](https://docs.rs/tokio/latest/tokio/)             |`tokio` and `tokio-util` |

### TLS

If you want TLS supports, enable the above runtime and also enable the cooresponding features:

| Read/Write provider                                        | Runtime     | Features                                          | TLS backend                |
| ---                                                        | ---         | ---                                               | ---                        |
| [`async_io`](https://docs.rs/async-io/latest/async_io/)    | `async_io`  | `"async-tls"`                                       | [`rustls`][rustls]         |
| [`smol`](https://docs.rs/smol/latest/smol/)                | `smol`      | `"async-tls"`                                       | [`rustls`][rustls]         |
| [`async_std`](https://docs.rs/async-std/latest/async_std/) | `async_std` | `"async-tls"`                                       | [`rustls`][rustls]         |
| [`tokio`](https://docs.rs/tokio/latest/tokio/)             | `tokio`     | `"tokio-util"`, `"tokio-rustls"`, `"webpki-roots"`   | [`rustls`][rustls]         |
| [`tokio`](https://docs.rs/tokio/latest/tokio/)             | `tokio`     | `"tokio-util"`, `"tokio-native-tls"`, `"native-tls"` | [`native-tls`][native-tls] |

[rustls]: https://docs.rs/rustls/0.18.1/rustls/
[native-tls]: https://docs.rs/native-tls/0.2.4/native_tls/


## Serde support
To enable serde support, simply enable the optional `serde` feature


## Examples
#### Using async_io to connect with.. 
* [async_io_demo.rs](./examples/async_io_demo.rs)

#### Using async_std to connect with..
* [async_std_demo.rs](./examples/async_std_demo.rs)


#### Using smol to connect with..
* [smol_demo.rs](./examples/smol_demo.rs)

#### Using tokio to connect with..
* [tokio_demo.rs](./examples/tokio_demo.rs)


#### How to use the crate as just a message parser(decoder)/encoder
* [message_parse.rs](./examples/message_parse.rs)

#### An a simple example of how one could built a bot with this
* [simple_bot.rs](./examples/simple_bot.rs)



## License
`trovochat` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.

[docs_badge]: https://docs.rs/trovochat/badge.svg
[docs]: https://docs.rs/trovochat
[crates_badge]: https://img.shields.io/crates/v/trovochat.svg
[crates]: https://crates.io/crates/trovochat
[actions_badge]: https://github.com/museun/trovochat/workflows/Rust/badge.svg
[actions]: https://github.com/museun/trovochat/actions

[Trovo]: https://dev.trovo.tv
