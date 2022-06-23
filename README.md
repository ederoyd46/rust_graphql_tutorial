## Rust GraphQL Tutorial

Learning how to create a GraphQL server in Rust.

Creates 2 binaries, one to run locally for debugging and another to deploy to AWS as a lambda.

To start the local server, run...

```shell
make run_local_graph
```

To deploy the graph as a lambda, run...

```shell
make release.package.deploy
```

..this will cross compile to linux first.

You will need to install the `musl` libc library and the `rust-std-x86_64-unknown-linux-musl` rustup component.
