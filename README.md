# rustRet

- rust crate for massspectrometry retention time and retention index.
- all massspectrometry claculation in one crate.
- this crate will do also the machine learning on the mass-spectrometry data.

```
cargo build
```

```
rustRet

Usage: rustRet <COMMAND>

Commands:
  retention-index  retention index calculation
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
```
./target/debug/rustRet retention-index ./testfile/sample.txt
elutes	retentiontime
6	678.5714285714286
```
