# Benchmarks

These benchmarks were created using [criterion](https://github.com/bheisler/criterion.rs), [cargo-criterion](https://github.com/bheisler/cargo-criterion) and [criterion-table](https://github.com/nu11ptr/criterion-table).

You can run them yourself by running this command

```sh
cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
```

You can view the results in [BENCHMARKS.md](BENCHMARKS.md)
