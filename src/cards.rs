use crate::random_deck::{generateDeck, Colors};
use std::fs;
use rand::prelude::*;
use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};
use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str}; //, to_string};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TomlConfig {
    pub file_name: String,
    pub points: u32,
    pub boosters: Vec<Booster>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Booster {
    pub set: String,
    pub amount: u32, 
    pub price: f32,
}

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CardInput {
    pub name: String,
    pub set: String,
    pub imagefile: String,
    pub rarity: String,
    pub card_type: String,
    pub color: String,
    pub text: String,
}

// LACKEY output formats.

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "deck")]
pub struct Deck<'a> {
    #[xml(attr = "version")]
    version: Cow<'a, str>,
    #[xml(child = "meta")]
    meta: Meta<'a>,
    #[xml(child = "superzone")]
    super_zone: SuperZone<'a>,
    //super_zone: Vec<SuperZone<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "meta")]
pub struct Meta<'a> {
    #[xml(child = "game")]
    game: Game<'a>,
}

#[warn(clippy::needless_late_init)]
#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "game")]
pub struct Game<'a> {
    #[xml(text)]
    name: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "superzone")]
pub struct SuperZone<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
    #[xml(child = "cards")]
    cards: Vec<Card<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "card")]
#[allow(unused_must_use)]
pub struct Card<'a> {
    #[xml(child = "name")]
    pub name: Name<'a>,
    #[xml(child = "set")]
    pub set: Set<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "name")]
pub struct Name<'a> {
    #[xml(attr = "id")]
    pub id: Cow<'a, str>,
    #[xml(text)]
    pub name: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "set")]
pub struct Set<'a> {
    #[xml(text)]
    pub name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardLackey {
    name: CardName,
    set: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CardName {
    id: String,
    #[serde(rename = "$value")]
    name: String, 
}

pub fn cardinput_to_card(card_input: &CardInput) -> Card {

    Card { name: Name {id: card_input.imagefile.clone().into(),
                       name: card_input.name.clone().into()},
           set: Set { name: card_input.set.clone().into()},
    }
}
 
pub fn buy_boosters<'a>(boosters: &'a Vec<Booster>, sets: &'a mut HashMap<String, Vec<CardInput>>, random_deck: bool, colors: Vec<Colors>) -> Vec<Card<'a>> {

    println!("Create boosters.");
    let mut result = Vec::<Card>::new();
    let mut result_input_format = Vec::<CardInput>::new();

    for booster in boosters {
        
        let rare = String::from("R");
        let unc = String::from("U");
        let com = String::from("C");

        let mut rares = Vec::<CardInput>::new(); 
        let mut uncommons = Vec::<CardInput>::new(); 
        let mut commons = Vec::<CardInput>::new(); 

        println!("Set {:?}. Amount {:?}", booster.set, booster.amount);

        // Filter set using rarity.
        let the_set = sets.get(&booster.set).unwrap();

        for i in the_set.iter() {
            if i.rarity.eq(&rare) {
               rares.push(i.clone());
            }
            else if i.rarity.eq(&unc) {
               uncommons.push(i.clone());
            }
            else if i.rarity.eq(&com) {
               commons.push(i.clone());
            }
        }

        let rare_count = booster.amount; 
        let uncommon_count = booster.amount * 3; 
        let common_count = booster.amount * 11; 
        let mut rare_counter = 0;
        let mut uncommon_counter = 0;
        let mut common_counter = 0;

        let mut rng = thread_rng();

        while rare_counter < rare_count {
            let ind = rng.gen_range(0..rares.len()); 

            result.push(
                Card { name: Name {id: rares[ind].imagefile.clone().into(),
                                   name: rares[ind].name.clone().into()},
                       set: Set { name: rares[ind].set.clone().into()},
                });
            result_input_format.push(rares[ind].clone());

            rare_counter += 1;
        }

        while uncommon_counter < uncommon_count {
            let ind = rng.gen_range(0..uncommons.len()); 

            result.push(
                Card { name: Name {id: uncommons[ind].imagefile.clone().into(),
                                   name: uncommons[ind].name.clone().into()},
                       set: Set { name: uncommons[ind].set.clone().into()},
                });

            result_input_format.push(uncommons[ind].clone());
            uncommon_counter += 1;
        }
        
        // TODO: list
        let swamp = String::from("Basic Land - Swamp");
        let plains = String::from("Basic Land - Plains");
        let forest = String::from("Basic Land - Forest");
        let island = String::from("Basic Land - Island");
        let mountain = String::from("Basic Land - Mountain");

        while common_counter < common_count {
            let ind = rng.gen_range(0..commons.len()); 
            // println!("{:?}", commons[ind].card_type);

            if commons[ind].card_type.eq(&swamp) ||
               commons[ind].card_type.eq(&plains) ||
               commons[ind].card_type.eq(&forest) ||
               commons[ind].card_type.eq(&island) ||
               commons[ind].card_type.eq(&mountain) ||
               commons[ind].name.to_lowercase().contains("snow-covered") ||
               commons[ind].text.to_lowercase().contains("snow-covered") ||
               commons[ind].text.to_lowercase().contains(" ante ") {

                // println!("Omittin card: {:?}", commons[ind]);
                continue;
            } 

            result.push(
                Card { name: Name {id: commons[ind].imagefile.clone().into(),
                                   name: commons[ind].name.clone().into()},
                       set: Set { name: commons[ind].set.clone().into()},
                });

            result_input_format.push(commons[ind].clone());
            common_counter += 1;
        }
    }
    if random_deck {
        let mut concatenated = Vec::<CardInput>::new();  
        for x in result_input_format.iter() {
            concatenated.push(x.clone());
        }
        result = generateDeck(concatenated, colors, 20, 60);
    }
    //println!("Deck size {:?} cards.", result.len());
    result
}

pub fn to_lackey(cards: &[Card], to_deck: bool) -> String {

    let super_zone_name = if to_deck { "Deck" } else { "Sideboard" };
    let deck = Deck {
        version: "0.8".into(),
        meta: Meta { game: Game {name:std::borrow::Cow::Borrowed("magic")}},
        super_zone: 
            SuperZone {
            name: std::borrow::Cow::Borrowed(super_zone_name),
            //cards: vec![],
            cards: cards.to_owned(),
            },
        // super_zone: vec![
        //     SuperZone {
        //     name: std::borrow::Cow::Borrowed("Deck"),
        //     //cards: vec![],
        //     cards: cards.to_owned(),
        //     },
        //     SuperZone {
        //     name: std::borrow::Cow::Borrowed("Sideboard"),
        //     cards: cards.to_owned(),
        //     },
        // ],
    };
    deck.to_string().unwrap()
}

pub fn destroy_sideboard(filename: String) {

    println!("Load file {:?}", filename);
    let src = load_from_file(filename.to_string());
    //let binding = src.unwrap().replace("\r\n\t\t", "").replace("\r\n\t", "").replace("\r\n", "");
    let binding = src.unwrap();
    //println!("{:?}", binding);
    let deck = Deck::from_str(&binding).unwrap();
    println!("{:?}", deck);

    //++ let start_bytes = binding.find("<superzone name=\"Sideboard\">").unwrap_or(0);
    //++ if start_bytes != 0 { 
    //++     let end_bytes = binding.rfind("</superzone>").unwrap_or(binding.len());
    //++     let result_start = &binding[0..start_bytes];
    //++     let result_end = &binding[end_bytes+"</superzone>".len()..binding.len()];
    //++     let result = result_start.to_owned() + result_end; 
    //++     fs::write(filename, result).expect("Unable to write file.");
    //++ }
    // for i in deck.super_zone[1].cards.iter() {
    //     println!("{:?}", i);
    // }

    // println!("Destroying sideboard from deck {:?}", filename);
    // deck.super_zone.retain(|x| x.name != "Sideboard");

    // println!("Saving file {:?}", filename);
    // let deck_src = deck.to_string().unwrap();
    // fs::write(filename, deck_src).expect("Unable to write file.");
}

pub fn load_list_of_cards() -> Result<String, Box<dyn Error>>  {
    println!("Loading the card list.");
    load_from_file("ListOfCardDataFiles.txt".to_string())
}

pub fn load_from_file(filename: String) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn load_mtg() -> HashMap<String, Vec<CardInput>> {

    let card_lists: Listofcarddatafiles  = from_str(&load_list_of_cards().unwrap()).unwrap();

    // Create hashmap for sets.
    let mut set_hash_map = HashMap::<String, Vec<CardInput>>::new(); 

    for list in card_lists.files_to_include {

        let file_loc = String::from("sets/") + &list.file;
        if let Ok(lines) = read_lines(file_loc) {

            let mut counter = 0;
        	for line in lines.flatten() {
                counter += 1;
                if counter < 4 { continue };
                let ll = line.split('\t').collect::<Vec<&str>>();
                if ll.len() < 2 { continue; }

                set_hash_map.entry(ll[1]
                .to_string())
                .or_default()
                .push(CardInput {
                        name: ll[0].to_string(),
                        set: ll[1].to_string(),
                        imagefile: ll[2].to_string(),
                        rarity: ll[12].to_string(),
                        card_type: ll[8].to_string(),
                        color: ll[5].to_string(),
                        text: ll[16].to_string(),
                });
        	}
        }
    }

    set_hash_map
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
