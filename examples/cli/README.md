# BudouX-rs (CLI)

CLI for BudouX-rs.

## Usage

```console
$ cargo run -- -h
Usage: target\debug\examples\budoux-cli.exe [options]

Options:
        --in INPUT      input string
        --model MODEL   path of model file
    -h, --help          print this help menu
```

Split Japanese sentences with internal model.

```console
$ cargo run -- --in "日本語の文章をいい感じに分割します。"
日本語の
文章を
いい
感じに
分割します。
```

Split Japanese sentences using the specified model.

```console
$ cargo run -- --model ja-knbc.json --in "日本語の文章をいい感じに分割します。"
日本語の
文章を
いい
感じに
分割します。
```
