extern crate regex;

use std::io;
use std::io::Read;
use regex::Regex;
use std::env;

fn url_clean(url: &str) -> String {
    let mut clean = String::from(url);
    let end = clean.find("&d=").unwrap();
    clean.truncate(end);

    clean = clean.replace("https://urldefense.proofpoint.com/v2/url?u=", "");
    clean = clean.replace("_", "/");
    clean = clean.replace("-21", "!");
    clean = clean.replace("-22", "\"");
    clean = clean.replace("-23", "#");
    clean = clean.replace("-24", "$");
    clean = clean.replace("-25", "%");
    clean = clean.replace("-26", "&");
    clean = clean.replace("-27", "'");
    clean = clean.replace("-28", "(");
    clean = clean.replace("-29", ")");
    clean = clean.replace("-2A", "*");
    clean = clean.replace("-2B", "+");
    clean = clean.replace("-2C", ",");

    clean = clean.replace("-2E", ".");
    clean = clean.replace("-2F", "/");
    clean = clean.replace("-31", "!");
    clean = clean.replace("-32", "\"");
    clean = clean.replace("-33", "#");
    clean = clean.replace("-34", "$");
    clean = clean.replace("-35", "%");
    clean = clean.replace("-36", "&");
    clean = clean.replace("-37", "'");
    clean = clean.replace("-38", "(");
    clean = clean.replace("-39", ")");
    clean = clean.replace("-3A", ":");
    clean = clean.replace("-3B", ";");
    clean = clean.replace("-3C", "<");
    clean = clean.replace("-3D", "=");
    clean = clean.replace("-3E", ">");
    clean = clean.replace("-3F", "?");
    clean = clean.replace("-40", "@");
    clean = clean.replace("-5B", "]");
    clean = clean.replace("-5C", "\\");
    clean = clean.replace("-5D", "]");
    clean = clean.replace("-5E", "^");
    clean = clean.replace("-5F", "_");
    clean = clean.replace("-2D", "-");

    return clean;
}

fn main() {
    let color = match env::args().nth(1) {
        Some(arg) => String::from("-c") == arg,
        _ => false,
    };
    let (set_col, unset_col) = if color {
        ("\x1b[1;32m", "\x1b[0m")
    } else {
        ("", "")
    };

    let re = Regex::new(r"https://urldefense.proofpoint.com[^ >]*").unwrap();

    let mut reader = io::stdin();
    let mut input = String::new();
    match reader.read_to_string(&mut input) {
        Ok(_) => (),
        Err(err) => panic!("{:}", err),
    }
    let mut prev: usize = 0;
    for pos in re.find_iter(&input) {
        match pos {
            (a, b) => {
                print!("{}", &input[prev..a]);
                print!("{}{}{}", set_col, url_clean(&input[a..b]), unset_col);
                prev = b;
            }
        }
    }
    print!("{}", &input[prev..]);
}
