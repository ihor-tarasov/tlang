
fn main() {
    if let Some(v) = tlang::run_str_unwrap("2 + 2 - 1 + 8 - 9 + 4") {
        println!("{v:?}");
    }
}
