use crate::random_deck::{generateDeck, Colors, color_to_char};

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

const deleted_names: [&str; 6] =
    ["Basic Land - Swamp",
     "Basic Land - Plains", 
     "Basic Land - Swamp",
     "Basic Land - Forest",
     "Basic Land - Island",
     "Basic Land - Mountain"
];

const deleted_names_contains: [&str; 1] =
    ["snow-covered"
];

const deleted_text_contains: [&str; 2] =
    ["snow-covered",
     " ante.",
];

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExcludeByName {
    pub exact_name: Vec<String>,
    pub name_contains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExcludeByText {
    pub text_contains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExcludeByColor {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RequireColor {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExcludeToml {
    pub by_name: ExcludeByName,
    pub by_text: ExcludeByText,
    pub exclude_color_contains: Vec<ExcludeByColor>,
    pub exclude_color_required: Vec<RequireColor>,
}

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

// pub fn example_exclude_toml() -> ExcludeToml {
//     ExcludeToml {
//         by_name: ExcludeByName { exact_name: vec!["Kortti blaah, Paha kortti".to_string()], name_contains: vec!["Kortti joka on".to_string(), "Joop".to_string()] },
//         by_text: ExcludeByText { text_contains: vec!["Kielleettya tekstia 1".to_string(), "Kiellettya tekstia 2".to_string()] },
//         exclude_color_contains: vec![ExcludeByColor { name: "Nimmi".to_string(), color: "GR".to_string()}, ExcludeByColor { name: "Nimmi2".to_string(), color: "BU".to_string()} ],
//         exclude_color_required: vec![RequireColor { name: "Hekotus".to_string(), color: "B".to_string(), }, RequireColor { name: "Hekotus2".to_string(), color: "R".to_string(), }],
//     }
// }

pub fn cardinput_to_card(card_input: &CardInput) -> Card {

    Card { name: Name {id: card_input.imagefile.clone().into(),
                       name: card_input.name.clone().into()},
           set: Set { name: card_input.set.clone().into()},
    }
}

/// A filter function. Decided if the cards should be ignored. TODO: do to_lowercase only once.
fn drop_card(card: &CardInput, exclude_list: &ExcludeToml, deck_colors: &Vec<Colors>) -> bool {

    let mut drop = false;
    let card_name = card.name.to_lowercase().to_owned();

    // Drop exact card names.
    for name in &exclude_list.by_name.exact_name {
        if card.name == *name { drop = true; break; }
    }

    // If the card name contains these words. 
    if !drop {
        for text in &exclude_list.by_name.name_contains {
            if card.name.to_lowercase().contains(&text.to_lowercase()) { drop = true; break; }
        }

    }
    // If the card text contains these words. 
    if !drop {
        for text in &exclude_list.by_text.text_contains {
            if card.text.to_lowercase().contains(&text.to_lowercase()) { drop = true; break; }
        }
    }

    if deck_colors.len() > 0 {
        // If deck contains a specific color. 
        if !drop {

            // Do the name exists on the exlude list.
            if let Some(name_found) = exclude_list.exclude_color_contains.iter().find(|&x| x.name.to_lowercase() == card_name) {
                for i in name_found.color.chars() {
                    for j in deck_colors.iter() {
                        if i == color_to_char(j) {
                            println!("{:?} == {:?}", i, color_to_char(j));
                            drop = true;
                            break;
                        }
                    }
                    if drop { break; }
                    //if drop { println!("{:?} dropped", card); break; }
                    
                }
            };
        }

        // The deck must contain same colors than the this rule.
        if let Some(name_found) = exclude_list.exclude_color_required.iter().find(|&x| x.name.to_lowercase() == card_name) {
            let deck_colors_char = deck_colors.iter().map(|x| color_to_char(x)).collect::<Vec<_>>();
            for i in name_found.color.chars() {
                if !deck_colors_char.contains(&i) {
                    drop = true;
                    // println!("{:?} dropped", card);
                    break;
                }
            }
        };
    }
    
    drop

}
 
pub fn buy_boosters<'a>(boosters: &'a Vec<Booster>, sets: &'a mut HashMap<String, Vec<CardInput>>, random_deck: bool, colors: Vec<Colors>) -> Vec<Card<'a>> {
    // println!("{:?}", toml::to_string(&example_exclude_toml()));
    println!("\n");
    println!("Create boosters.");

    // Load card exclude list.
    let mut f = File::open("exclude.toml").expect("Couldn't find 'exclude.toml'.");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let exclude_list: ExcludeToml = toml::from_str(&buffer).unwrap(); 

    let mut result = Vec::<Card>::new();
    let mut result_input_format = Vec::<CardInput>::new();

    for booster in boosters {
        
        let rare = String::from("R");
        let unc = String::from("U");
        let com = String::from("C");

        let mut rares = Vec::<CardInput>::new(); 
        let mut uncommons = Vec::<CardInput>::new(); 
        let mut commons = Vec::<CardInput>::new(); 

        if booster.amount > 0 {
            println!("Set {:?}. Amount {:?}", booster.set, booster.amount);
        }

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

            if drop_card(&rares[ind], &exclude_list, &colors) { continue; }

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

            if drop_card(&uncommons[ind], &exclude_list, &colors) { continue; }

            result.push(
                Card { name: Name {id: uncommons[ind].imagefile.clone().into(),
                                   name: uncommons[ind].name.clone().into()},
                       set: Set { name: uncommons[ind].set.clone().into()},
                });

            result_input_format.push(uncommons[ind].clone());
            uncommon_counter += 1;
        }
        
        while common_counter < common_count {
            let ind = rng.gen_range(0..commons.len()); 

            if drop_card(&commons[ind], &exclude_list, &colors) { continue; }

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
    result
}

/// save cards to xml form that can be loaded from Lackey.
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
    let src = load_from_file(filename);
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

// Load card database files.
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

// Load mtg cards and return hashmap, where key is set and value is the cards that belongs to the set.
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
