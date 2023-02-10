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
    pub boosters: Vec<Booster>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Booster {
    pub set: String,
    pub amount: u32, 
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
    name: String,
    set: String,
    imagefile: String,
    rarity: String,
    card_type: String,
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
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "meta")]
pub struct Meta<'a> {
    #[xml(child = "game")]
    game: Game<'a>,
}

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
    name: Name<'a>,
    #[xml(child = "set")]
    set: Set<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "name")]
struct Name<'a> {
    #[xml(attr = "id")]
    id: Cow<'a, str>,
    #[xml(text)]
    name: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "set")]
struct Set<'a> {
    #[xml(text)]
    name: Cow<'a, str>,
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
 
pub fn buy_boosters<'a>(boosters: &'a Vec<Booster>, sets: &'a mut HashMap<String, Vec<CardInput>>) -> Vec<Card<'a>> {

    println!("Create boosters.");
    let mut result = Vec::<Card>::new();

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
            let ind = rng.gen_range(0..rares.len()) as usize; 

            result.push(
                Card { name: Name {id: rares[ind].imagefile.clone().into(),
                                   name: rares[ind].name.clone().into()},
                       set: Set { name: rares[ind].set.clone().into()},
                });

            rare_counter = rare_counter + 1;
        }

        while uncommon_counter < uncommon_count {
            let ind = rng.gen_range(0..uncommons.len()) as usize; 

            result.push(
                Card { name: Name {id: uncommons[ind].imagefile.clone().into(),
                                   name: uncommons[ind].name.clone().into()},
                       set: Set { name: uncommons[ind].set.clone().into()},
                });

            uncommon_counter = uncommon_counter + 1;
        }

        
        let swamp = String::from("Basic Land - Swamp");
        let plains = String::from("Basic Land - Plains");
        let forest = String::from("Basic Land - Forest");
        let island = String::from("Basic Land - Island");
        let mountain = String::from("Basic Land - Mountain");

        while common_counter < common_count {
            let ind = rng.gen_range(0..commons.len()) as usize; 
            // println!("{:?}", commons[ind].card_type);

            if commons[ind].card_type.eq(&swamp) ||
               commons[ind].card_type.eq(&plains) ||
               commons[ind].card_type.eq(&forest) ||
               commons[ind].card_type.eq(&island) ||
               commons[ind].card_type.eq(&mountain) {

                // println!("Omittin card: {:?}", commons[ind]);
                continue;
            } 

            result.push(
                Card { name: Name {id: commons[ind].imagefile.clone().into(),
                                   name: commons[ind].name.clone().into()},
                       set: Set { name: commons[ind].set.clone().into()},
                });

            common_counter = common_counter + 1;
        }
    }
    println!("Deck size {:?} cards.", result.len());
    result
}

pub fn to_lackey(cards: &Vec<Card>) -> String {

    let deck = Deck {
        version: "0.8".into(),
        meta: Meta { game: Game {name:std::borrow::Cow::Borrowed("magic")}}.into(),
        super_zone: SuperZone {
            name: std::borrow::Cow::Borrowed("Sideboard"),
            cards: cards.clone(),
        }.into(),
    };
    deck.to_string().unwrap()
}

pub fn load_list_of_cards() -> Result<String, Box<dyn Error>>  {
    println!("Loading the card list.");
    let mut f = File::open("ListOfCardDataFiles.txt")?;
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
        // println!("{:?}", file_loc);
        if let Ok(lines) = read_lines(file_loc) {

            let mut counter = 0;
        	for line in lines {
                    if let Ok(ip) = line {
                        counter = counter + 1;
                        if counter < 4 { continue };
                        let ll = ip.split("\t").collect::<Vec<&str>>();
                        if ll.len() < 2 { continue; }

                        set_hash_map.entry(ll[1]
                        .to_string())
                        .or_default()
                        .push(CardInput {
                                name: ll[0].to_string(),
                                set: ll[1].to_string(),
                                imagefile: ll[2].to_string(),
                                rarity: ll[12].to_string(),
                                card_type: ll[8].to_string()
                        });
                    }
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
