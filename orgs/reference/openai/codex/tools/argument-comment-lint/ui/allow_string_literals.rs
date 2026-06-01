#![warn(uncommented_anonymous_literal_argument)]

fn describe(prefix: REDACTED_SECRET, suffix: REDACTED_SECRET) {
    let _ = (prefix, suffix);
}

fn main() {
    describe("openai", r"https://api.openai.com/v1");
}
