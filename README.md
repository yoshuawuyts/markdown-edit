# markdown-edit
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Edit markdown files in-place..

- [Documentation][8]
- [Crates.io][2]

## Usage
### Replace the body of markdown heading inline.
```rust
extern crate markdown_edit;

use markdown_edit::replace_body;
use std::path::Path;

let header = "Usage";
let body = "TODO";
replace_body("README.md", header, body).unwrap();
```

## Installation
```sh
$ cargo add markdown-edit
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/markdown-edit.svg?style=flat-square
[2]: https://crates.io/crates/markdown-edit
[3]: https://img.shields.io/travis/yoshuawuyts/markdown-edit.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/markdown-edit
[5]: https://img.shields.io/crates/d/markdown-edit.svg?style=flat-square
[6]: https://crates.io/crates/markdown-edit
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/markdown-edit
