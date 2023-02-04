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

#[derive(Debug)]
pub struct CardInput {
    name: String,
    set: String,
    imagefile: String,
}

#[allow(dead_code)]
pub struct Card {
    name: String,
    set: String,
    imagefile: String,
    actualset: String,
    color: String,
    colorid: String,
    cost: String,
    manavalue: String,
    card_type: String,
    power: String,
    toughness: String,
    loyalty: String,
    rarity: String,
    draftqualities: String,
    sound: String,
    script: String,
    text: String,
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

pub fn load_cards() -> Vec<Card> {
    let card_lists: Listofcarddatafiles  = from_str(&load_list_of_cards().unwrap()).unwrap();
    Vec::<Card>::new()
}

pub fn load_mtg() {

    let card_lists: Listofcarddatafiles  = from_str(&load_list_of_cards().unwrap()).unwrap();

    let mut cards = Vec::<CardInput>::new();

    for list in card_lists.files_to_include {

        //if let Ok(lines) = read_lines("./premodern-expansions.txt") {
        if let Ok(lines) = read_lines(list.file) {
        	// Consumes the iterator, returns an (Optional) String
            let mut counter = 0;
        	for line in lines {
                    if let Ok(ip) = line {
                        counter = counter + 1;
                        if counter < 4 { continue };
                        let ll = ip.split("\t").collect::<Vec<&str>>();
                        if ll.len() < 2 { continue; }
                        cards.push(CardInput { name: ll[0].to_string(), set: ll[1].to_string(), imagefile: ll[2].to_string() });
                    }
        	}
        }
    }
    println!("{:?}", cards);
    println!("The length == {:?}", cards.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
