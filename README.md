# index-to-position

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![crates.io](https://img.shields.io/crates/v/index-to-position.svg)](https://crates.io/crates/index-to-position)
[![docs.rs](https://docs.rs/index-to-position/badge.svg)](https://docs.rs/index-to-position)
[![CI](https://github.com/trananhtung/index-to-position/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/index-to-position/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/index-to-position.svg)](#license)

**Convert a string index to a line and column position.**

Turn a byte offset into a string — such as a parser or lexer error offset — into a
human-readable `line:column`. A Rust port of the
[`index-to-position`](https://www.npmjs.com/package/index-to-position) npm package.

- **Zero dependencies**, **`#![no_std]`**
- Zero- or one-based line/column (independently configurable)
- [`PositionFinder`] for `O(log lines)` lookups when resolving many indices into one text
- Differential-tested against the reference `index-to-position` implementation

## Install

```toml
[dependencies]
index-to-position = "0.1"
```

## Usage

```rust
use index_to_position::{index_to_position, index_to_position_with, Options, PositionFinder};

let text = "foo\nbar\nbaz";

// Zero-based by default:
let pos = index_to_position(text, 5); // the 'a' in "bar"
assert_eq!((pos.line, pos.column), (1, 1));

// One-based, for display:
let pos = index_to_position_with(text, 5, Options::new().one_based(true));
assert_eq!((pos.line, pos.column), (2, 2));

// Resolve many indices efficiently:
let finder = PositionFinder::new(text);
assert_eq!(finder.position(9).line, 2);
```

## Indices are byte offsets

Unlike the JavaScript original (which uses UTF-16 code unit offsets), this crate works with
**byte offsets** — the idiomatic Rust string index, as produced by `str` slicing and most
parsers. Lines and columns are counted in bytes. For ASCII text the two are identical; only
`\n` is treated as a line break (a `\r` is an ordinary column character, matching the
reference).

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
