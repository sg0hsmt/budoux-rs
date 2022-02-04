#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn bench_parse(b: &mut Bencher) {
    let model = budoux::models::default_japanese_model();
    b.iter(|| budoux::parse(model, "日本語の文章をいい感じに分割します。"))
}
