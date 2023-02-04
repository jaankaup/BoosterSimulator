use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str, to_string};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Listofcarddatafiles {
    #[serde(rename = "$value")]
    files_to_include: Vec<Filetoinclude>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Filetoinclude  {
    #[serde(rename = "$value")]
    file: String
}

pub struct Card {

}

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct PlateAppearance {
//     #[serde(rename = "$value")]
//     events: Vec<Event>
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Content {
//     #[serde(rename = "$value")]
//     value: String
// }

pub struct Card {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

//pub fn load_list_of_cards( )-> Result<(), Box<dyn Error>>  {
pub fn load_list_of_cards() -> Result<String, Box<dyn Error>>  {
    let mut f = File::open("ListOfCardDataFiles.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn load_mtg() {

	if let Ok(lines) = read_lines("./ListOfCardDataFiles.txt") {
		// Consumes the iterator, returns an (Optional) String
		for line in lines {
			if let Ok(ip) = line {
				println!("{}", ip);
			}
		}
	}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
