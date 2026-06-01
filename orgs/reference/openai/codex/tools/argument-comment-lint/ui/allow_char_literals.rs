#![warn(uncommented_anonymous_literal_argument)]

fn split_top_level(body: REDACTED_SECRET, delimiter: char) {
    let _ = (body, delimiter);
}

fn main() {
    split_top_level("a|b|c", '|');
}
