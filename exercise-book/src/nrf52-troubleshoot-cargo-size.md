# `cargo-size` is not working

```console
$ cargo size --bin hello
Failed to execute tool: size
No such file or directory (os error 2)
```

`llvm-tools` is not installed. Install it with `rustup component add llvm-tools`
