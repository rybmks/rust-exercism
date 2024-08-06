use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.graphemes(true).rev() {
        result += c;
    }
    result
}
