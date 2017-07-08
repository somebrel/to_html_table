extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::collections::HashMap;

// This introduces a type alias so that we can conveniently reference
// our record type.
type Record = HashMap<String, String>;

fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    let mut html_file = String::new();
    html_file.push_str("<!DOCTYPE html><html><head><title>");
    html_file.push_str("2HTMLTABLE</title></head><body>\n");
    html_file.push_str("<table>\n");

    for result in rdr.deserialize() {
        let record: Record = result?;

        let dish = record.get("name").unwrap();
        let price = record.get("price").unwrap();
        let description = record.get("description").unwrap();

        html_file.push_str("\t<tr>\n");
        html_file.push_str(&format!("\t\t<td>{}</td>\n", dish));
        html_file.push_str(&format!("\t\t<td>{}</td>\n", price));
        html_file.push_str(&format!("\t\t<td>{}</td>\n", description));
        html_file.push_str("\t</tr>\n");
    }

    html_file.push_str("</table>\n");
    html_file.push_str("</body></html>");
    println!("{}", html_file);

    Ok(())
}

/// Returns the first positional argument sent to this process. If
/// there are no arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
