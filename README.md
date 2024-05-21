# wf-score

Small hobby project to calculate scores for weapon builds in Warframe.

## Usage

Two binary crates are available, `single-score` and `bruteforce`. The first one
calculates the score for a single build, while the second one calculates the
score for all possible builds.

To run the binaries, use either of the following commands:
```bash
cargo run --release --package single-score
```

```bash
cargo run --release --package bruteforce
```

The `--release` flag is optional, but recommended for performance reasons
(especially for the `bruteforce` binary).

## Build examples

Here are some examples of builds I made using the `bruteforce` binary:
- [Ceti Lacera](https://overframe.gg/build/666758/ceti-lacera/)
- [Ceti Lacera w/ Riven](https://overframe.gg/build/668455/ceti-lacera/)
- [Tenet Cycron w/ Riven](https://overframe.gg/build/669679/tenet-cycron/)

## License

Licensed under the MIT license. See the [LICENSE](LICENSE) file for more
details.
