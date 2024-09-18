use std::fs;

use tidbparser::TiDBParser;

fn main() {
    let filename = std::env::args().nth(1).expect(
        r#"
No arguments provided!

Usage:
$ cargo run --example cli FILENAME.sql
"#,
    );

    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Unable to read the file {}", &filename));

    let parse_result = TiDBParser::parse_sql(contents.as_str());
    match parse_result {
        Ok(statements) => {
            println!(
                "Round-trip:\n'{}'",
                statements
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            println!("Parse results:\n{statements:#?}");

            std::process::exit(0);
        }
        Err(e) => {
            println!("Error during parsing: {e:?}");
            std::process::exit(1);
        }
    }
}
