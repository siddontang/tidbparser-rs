# tidbparser-rs

A Rust library for parsing TiDB dialect.

## Overview

tidbparser-rs is a Rust implementation of a parser for TiDB dialect.

## Usage

```rust
use tidbparser::TiDBParser;

let sql = "SELECT * FROM users WHERE id = 1";
let ast = TiDBParser::parse_sql(sql).unwrap();
println!("{:?}", ast);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
