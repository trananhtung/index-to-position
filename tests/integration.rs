//! Integration tests exercising the public API of `index-to-position`.

use index_to_position::{index_to_position, index_to_position_with, Options, PositionFinder};

const SOURCE: &str = "const x = 1;\nlet y = x + 2;\n\nfn main() {}\n";

#[test]
fn resolves_offsets() {
    // Start of line 2 ("let").
    let idx = SOURCE.find("let").unwrap();
    assert_eq!(index_to_position(SOURCE, idx), index_to_position(SOURCE, 13));
    let pos = index_to_position(SOURCE, idx);
    assert_eq!((pos.line, pos.column), (1, 0));
}

#[test]
fn finder_agrees_with_function_one_based() {
    let opts = Options::new().one_based(true);
    let finder = PositionFinder::with_options(SOURCE, opts);
    for idx in 0..=SOURCE.len() {
        assert_eq!(finder.position(idx), index_to_position_with(SOURCE, idx, opts), "idx {idx}");
    }
}

#[test]
fn blank_lines() {
    let idx = SOURCE.find("fn main").unwrap();
    let pos = index_to_position_with(SOURCE, idx, Options::new().one_based(true));
    assert_eq!((pos.line, pos.column), (4, 1));
}
