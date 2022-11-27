
use chrono::*;
use std::fs;
use std::env;


struct Entry{
    time:NaiveDate,
    text:String,
}


const JOURNAL_PATH: &str = "/notebook/diarios/2022";



fn parse_entries(e_vector: &mut Vec<Entry>, file_path: &String){

    let contents = fs::read_to_string(file_path)
        .expect("ERROR: No se encuentra fichero indicado");
    let entries_text = contents.split("## ");
    for e in entries_text{
        let entry_title = e.split("\n").nth(0).unwrap();
        let edate=parse_date(entry_title.to_string(), &file_path);
        match edate {
            Ok(date) => {
                let entry = Entry{time: date, text:e.to_string()};
                e_vector.push(entry);
            },
            _ => {},
        }
        
    }
}


fn parse_date(entry_title:String, file_path: &String)  -> ParseResult<NaiveDate>{
    
    let day: String = entry_title.chars()
        .filter(|entry_title| entry_title.is_digit(10))
        .collect();

    let file_path_cur = file_path.replace(".md","");
    let fields = file_path_cur.split("/");
    let mut filename = fields.last().unwrap().split("-");
    let year = filename.next().unwrap();
    let month = filename.next().unwrap();

    let sdate = format!("{}/{}/{}",day,month,year);
    return NaiveDate::parse_from_str(&sdate,"%d/%m/%Y");
}


fn main(){

    let mut path = env::var("HOME").expect("$HOME is not set");
    path.push_str(JOURNAL_PATH);

    let mut entries_vector : Vec<Entry> = Vec::new();

    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        parse_entries(&mut entries_vector, &path.unwrap().path().display().to_string());
    }

    println!("Se añadieron {} entradas a e_vector",entries_vector.len());

    println!("{}",entries_vector[1].text);

/*
    let fecha = create_date(22,11,2022);
    match fecha{
        Some(f) => println!("{}",f.format("%d/%m/%Y")),
        None => println!("Fecha mal formada"),
    }
*/

}