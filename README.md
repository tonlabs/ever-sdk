# TON SDK

Core Client Library built on the TON OS GraphQL API for Free TON DApp development

**Get quick help in our telegram channel:**

[![Channel on Telegram](https://img.shields.io/badge/chat-on%20telegram-9cf.svg)](https://t.me/ton_sdk)

## Content Table

* [Content Table](./#content-table)
* [Useful links](./#useful-links)
* [What is Core Free TON Client Library](./#what-is-core-free-ton-client-library)
* [SDKs in other languages (bindings over TON-SDK)](./#sdks-in-other-languages-bindings-over-ton-sdk)
  * [Official Javascript(Typescript) SDK](./#official-javascripttypescript-sdk)
  * [Community bindings](./#community-bindings)
* [How to use library](./#how-to-use-library)
* [How to avoid Soft Breaking Problems](./#how-to-avoid-soft-breaking-problems)
* [Build client library](./#build-client-library)
* [Build artifacts](./#build-artifacts)
* [Run tests](./#run-tests)
* [Download precompiled binaries](./#download-precompiled-binaries)

## Useful links

[Quick Start (Javascript)](guides/quick_start.md)

[Error descriptions](reference/error_codes.md)

[JavaScript SDK Types and Methods (API Reference)](https://tonlabs.github.io/ton-client-js/)

[Core Types and Methods (API Reference)](reference/types-and-methods/modules.md)

[Guides](guides/installation/add_sdk_to_your_app.md)

## What is Core Free TON Client Library

Core Client Library is written in Rust that can be dynamically linked. It provides all heavy-computation components and functions, such as TON Virtual Machine, TON Transaction Executor, ABI-related functions, boc-related functions, crypto functions.

The decision to create the Rust library was made after a period of time using pure JavaScript to implement these use cases.

We ended up with very slow work of pure JavaScript and decided to move all this to Rust library and link it to Javascript as a compiled binary including a wasm module for browser applications.

Also this approach provided an opportunity to easily create bindings for any programming language and platform, thus, to make it possible to develop distributed applications (DApps) for any possible use-cases, such as: mobile DApps, web DApps, server-side DApps, enterprise DApp etc.

Client Library exposes all the functionality through a few of exported functions. All interaction with library is performed using JSON-RPC like protocol.

Library works over [GraphQL API](reference/ton_os_api/) of [TON OS DApp Server](https://github.com/tonlabs/TON-OS-DApp-Server). So, it can be used to interact directly with [TON OS Clouds](reference/ton_os_api/networks.md).

## SDKs in other languages (bindings over TON-SDK)

Binding is a thin client library written on the specific language that acts like a bridge between a client library and an application code written on that language.

### Official Javascript(Typescript) SDK

Supported platforms: Node.js, Web, React-Native for IOS/Android

Repository: [JavaScript SDK](https://github.com/tonlabs/ton-client-js)

### Community bindings

| Language   | Repository                                                                                                                                                                                                                                                                                                                                    |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Clojure    | [serge-medvedev/tonos-client-clojure](https://github.com/serge-medvedev/tonos-client-clojure)                                                                                                                                                                                                                                                 |
| Dart       | [freetonsurfer/ton_client_dart](https://github.com/freetonsurfer/ton_client_dart)                                                                                                                                                                                                                                                             |
| Golang     | <p><a href="https://github.com/radianceteam/ton-client-go">radianceteam/ton-client-go</a><br><a href="https://github.com/move-ton/ton-client-go">move-ton/ton-client-go</a></p>                                                                                                                                                               |
| Java       | [radianceteam/ton-client-java](https://github.com/radianceteam/ton-client-java)                                                                                                                                                                                                                                                               |
| Kotlin     | [mdorofeev/ton-client-kotlin](https://github.com/mdorofeev/ton-client-kotlin)                                                                                                                                                                                                                                                                 |
| Lua        | [serge-medvedev/tonos-client-lua](https://github.com/serge-medvedev/tonos-client-lua)                                                                                                                                                                                                                                                         |
| .NET       | <p><a href="https://github.com/radianceteam/ton-client-dotnet">radianceteam/ton-client-dotnet</a><br><a href="https://github.com/ton-actions/ton-client-dotnet">ton-actions/ton-client-dotnet</a><br><a href="https://github.com/vcvetkovs/TonSdk">vcvetkovs/TonSdk</a><br><a href="https://github.com/staszx/Ton.Sdk">staszx/Ton.Sdk</a></p> |
| PHP        | <p><a href="https://github.com/extraton/php-ton-client">extraton/php-ton-client</a><br><a href="https://github.com/radianceteam/ton-client-php">radianceteam/ton-client-php</a></p>                                                                                                                                                           |
| Python     | [move-ton/ton-client-py](https://github.com/move-ton/ton-client-py)                                                                                                                                                                                                                                                                           |
| Ruby       | <p><a href="https://github.com/radianceteam/ton-client-ruby">radianceteam/ton-client-ruby</a><br><a href="https://github.com/nerzh/ton-client-ruby">nerzh/ton-client-ruby</a></p>                                                                                                                                                             |
| Scala      | <p><a href="https://github.com/slavaschmidt/ton-sdk-client-scala-binding/">slavaschmidt/ton-sdk-client-scala-binding/</a><br><a href="https://github.com/radianceteam/ton-client-scala">radianceteam/ton-client-scala</a></p>                                                                                                                 |
| Swift      | [nerzh/ton-client-swift](https://github.com/nerzh/ton-client-swift)                                                                                                                                                                                                                                                                           |
| Typescript | [RSquad/ton-client-ts](https://github.com/RSquad/ton-client-ts)                                                                                                                                                                                                                                                                               |

## How to use library

The simplest way is to use library in then Rust applications because of the native Rust library interface. The Rust interface is clear and well documented.

But what if you are required to use library in languages others than Rust?

You have some options:

*   use library module `json_interface` which provides access to library functions through

    JSON-RPC interface. This interface exports several extern "C" functions. So you can build

    a dynamic or static link library and link it to your application as any other external

    libraries. The JSON Interface is fully "C" compliant. You can find description

    in section [JSON Interface](for-binding-developers/json_interface.md).
*   use bindings already written by TON Labs and community. Below you can find a list of known

    bindings.
* write your own binding to chosen language and share it with community.

If you choose using JSON Interface please read this document [JSON Interface](for-binding-developers/json_interface.md).\
Here you can find directions how to use `json_interface` and write your own binding.

## How to avoid Soft Breaking Problems

Soft Breaking is API changes that include only new optional fields in the existing structures. This changes are fully backward compatible for JSON Interface.

But in Rust such changes can produce some problems with an old client code.

Look at the example below:

1\) There is an API v1.0 function `foo` and the corresponding params structure:

```rust
#[derive(Default)]
struct ParamsOfFoo {
    pub foo: String,
}

pub fn foo(params: ParamsOfFoo)
```

2\) Application uses this function in this way:

```rust
foo(ParamsOfFoo {
    foo: "foo".into(),
});
```

3\) API v.1.1 introduces new field in `ParamsOfFoo`:

```rust
#[derive(Default)]
struct ParamsOfFoo {
    pub foo: String,
    pub bar: Option<String>,
}
```

From the perspective of JSON-interface it isn't breaking change because the new parameter is optional. But code snippet (2) will produce Rust compilation error.

4\) To avoid such problems we recommend to use default implementation inside structure initialisation:

```rust
foo(ParamsOfFoo {
    foo: "foo".into(),
    ..Default::default(),
});
```

For all Ton Client API structures `Default` trait is implemented.

## Build client library

The best way to build client libraries is to use build scripts from this repo.

**Note**: The scripts are written in JavaScript so you have to install Node.js (v.10 or newer) to run them. Also make sure you have the latest version of Rust installed.

To build a binary for a specific target (or binding), navigate to the relevant folder and run `node build.js`.

The resulting binaries are placed to `bin` folder in the gz-compressed format.

Note that the build script generates binaries compatible with the platform used to run the script. For example, if you run it on Mac OS, you get binaries targeted at Darwin (macOS) platform.

**Note**: You need latest version of rust. Upgrade it with `rustup update` command. Check version with `rustc --version`, it should be above or equal to `1.47.0`.

## Build artifacts

Rebuild `api.json`:

```
cd toncli
cargo run api -o ../tools
```

Rebuild `docs`:

```
cd tools
npm i
tsc
node index docs -o ../docs
```

Rebuild `modules.ts`:

```
cd tools
npm i
tsc
node index binding -l ts -o ../../ton-client-js/packages/core/src
```

## Run tests

To run test suite use standard Rust test command

```
cargo test
```

SDK tests need [TON OS API](https://docs.ton.dev/86757ecb2/p/793337-ton-os-api) endpoint to run on. Such an API is exposed by a [DApp Server](https://github.com/tonlabs/TON-OS-DApp-Server) which runs in real networks and by local blockchain [TON OS SE](https://github.com/tonlabs/tonos-se).

TON OS SE is used by default with address `http://localhost` and port 80. If you launch it on another port you need to specify it explicitly like this: `http://localhost:port`. If you have TON OS SE running on another address or you need to run tests on a real TON network use the following environment variables to override the default parameters

```
TON_USE_SE: true/false - flag defining if tests run against TON OS SE or a real network (DApp Server)
TON_NETWORK_ADDRESS - Dapp server or TON OS SE addresses separated by comma.
TON_GIVER_SECRET - Giver secret key. If not defined, default TON OS SE giver keys are used
TON_GIVER_ADDRESS - Address of the giver to use for prepaying accounts before deploying test contracts. If not defined, the address is calculated using `GiverV2.tvc` and configured public key
```

## Download precompiled binaries

Instead of building library yourself, you can download the **latest** precompiled binaries from TON Labs SDK Binaries Store.

| Platform | Major | Download links                                                                                                                                           |
| -------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Win32    | 0     | [`ton_client.lib`](https://binaries.tonlabs.io/tonclient\_0\_win32\_lib.gz), [`ton_client.dll`](https://binaries.tonlabs.io/tonclient\_0\_win32\_dll.gz) |
|          | 1     | [`ton_client.lib`](https://binaries.tonlabs.io/tonclient\_1\_win32\_lib.gz), [`ton_client.dll`](https://binaries.tonlabs.io/tonclient\_1\_win32\_dll.gz) |
| macOS    | 0     | [`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_0\_darwin.gz)                                                                             |
|          | 1     | (x86\_64)[`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_1\_darwin.gz)                                                                    |
|          | 1     | (aarch64)[`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_1\_darwin_arm64.gz)                                                              |
| Linux    | 0     | [`libton_client.so`](https://binaries.tonlabs.io/tonclient\_0\_linux.gz)                                                                                 |
|          | 1     | [`libton_client.so`](https://binaries.tonlabs.io/tonclient\_1\_linux.gz)                                                                                 |

If you want an older version of library (e.g. `0.25.0` for macOS), you need to choose a link to your platform from the list above and replace `0` with a version: [https://binaries.tonlabs.io/tonclient\_**0\_25\_0**\_darwin.gz](http://sdkbinaries.tonlabs.io/tonclient\_0\_25\_0\_darwin.gz)

_Downloaded archive is gzipped file_
