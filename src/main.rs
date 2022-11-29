use std::cmp::Ordering;
use std::env;
use std::fs;
use std::process;

use argparse::*;
use chrono::*;

struct Entry {
    date: NaiveDate,
    text: String,
}

const JOURNAL_PATH: &str = "/notebook/diarios/2022";

// Parsed and whole entry from an text fragment
fn parse_entries(e_vector: &mut Vec<Entry>, file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("ERROR: No se encuentra fichero indicado");
    let entries_text = contents.split("## ");
    for e in entries_text {
        let entry_title = e.split("\n").nth(0).unwrap();
        let edate = parse_entry_date(entry_title.to_string(), &file_path);
        match edate {
            Ok(date) => {
                let entry = Entry {
                    date: date,
                    text: e.to_string(),
                };
                e_vector.push(entry);
            }
            _ => {}
        }
    }
}

// Parsed an entry date from the entry title. It's first line
fn parse_entry_date(entry_title: String, file_path: &String) -> ParseResult<NaiveDate> {
    let day: String = entry_title
        .chars()
        .filter(|entry_title| entry_title.is_digit(10))
        .collect();

    let file_path_cur = file_path.replace(".md", "");
    let fields = file_path_cur.split("/");
    let mut filename = fields.last().unwrap().split("-");
    let year = filename.next().unwrap();
    let month = filename.next().unwrap();

    let sdate = format!("{}/{}/{}", day, month, year);
    return NaiveDate::parse_from_str(&sdate, "%d/%m/%Y");
}

// Parsed an string date from user
fn parse_date(sdate: String) -> NaiveDate {
    let edate = NaiveDate::parse_from_str(&sdate, "%d/%m/%y");
    return match edate {
        Ok(date) => date,
        Err(_err) => Utc::now().naive_utc().date(),
    };
}

// Get a entry by a date
fn get_entry_by_date(e_vector: &Vec<Entry>, date: NaiveDate) -> Option<&Entry> {
    for e in e_vector {
        if e.date.cmp(&date) == Ordering::Equal {
            return Some(e);
        }
    }
    return None;
}

// Print an entry for a date
fn print_entry_by_date(e_vector: &Vec<Entry>, date: NaiveDate) {
    let entry = get_entry_by_date(e_vector, date);
    match entry {
        Some(ent) => {
            println!("{}", date.format("\x1b[1m == [%d/%m/%y] ==\x1b[0m"));
            let lines = textwrap::wrap(&ent.text, 80);
            for l in lines {
                println!("{}", l);
            }
        }
        _ => {}
    }
}

fn main() {
    let mut date = "".to_string();
    let mut next = 0;

    {
        // argparse block code
        let mut ap = ArgumentParser::new();
        ap.set_description("Aplicación para la gestión de diarios");
        ap.refer(&mut date)
            .add_option(&["-d", "--date"], Store, "Mostrar entradas de una fecha");

        ap.refer(&mut next).add_option(
            &["-n", "--next"],
            Store,
            "Mostrar las siguientes entradas sobre la fecha",
        );

        let args: Vec<_> = env::args().collect();
        if args.len() == 1 {
            println!("Se necesitan opciones. Use -h para obtener ayuda");
            process::exit(0);
        } else {
            ap.parse_args_or_exit();
        }
    }

    let mut path = env::var("HOME").expect("$HOME is not set");
    path.push_str(JOURNAL_PATH);

    let mut entries_vector: Vec<Entry> = Vec::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        parse_entries(
            &mut entries_vector,
            &path.unwrap().path().display().to_string(),
        );
    }

    entries_vector.sort_by(|a, b| b.date.cmp(&a.date));

    if date != "" {
        let real_date = parse_date(date);
        print_entry_by_date(&entries_vector, real_date);

        if next > 0 {
            for i in 0..next {
                let next_date = real_date.checked_add_days(Days::new(i));
                match next_date {
                    Some(nxt) => print_entry_by_date(&entries_vector, nxt),
                    _ => {}
                }
            }
        }
    }
}
