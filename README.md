# BudouX-rs

BudouX-rs is a rust port of [BudouX](https://github.com/google/budoux) (machine learning powered line break organizer tool).

Note:
This project contains the deliverables of the [BudouX](https://github.com/google/budoux) project.

Note:
BudouX-rs supported plain text only, not supports html inputs.

## Demo

TBD

## Documentation

TBD

## Usage

Split sentences with internal model.

```rust
let model = budoux::models::default_japanese_model();
let words = budoux::parse(model, "これはテストです。");

assert_eq!(words, vec!["これは", "テストです。"])
```

Load model from json file and split sentences using the loaded model.

```rust
let file = File::open(path_to_json).unwrap();
let reader = BufReader::new(file);
let model: budoux::Model = serde_json::from_reader(reader).unwrap();
let words = budoux::parse(&model, "これはテストです。");

assert_eq!(words, vec!["これは", "テストです。"])
```

## Test

```console
cargo test
```

## Generate model from original BudouX

```console
go generate ./...
```

Note:
Generate model is require Go 1.13 or later.
