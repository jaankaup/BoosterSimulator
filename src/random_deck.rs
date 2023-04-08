use crate::cards::Name;
use crate::cards::Set;
use std::cmp;
use rand::prelude::*;
use std::mem::transmute;
use std::collections::HashMap;
use crate::cards::{CardInput, Card, cardinput_to_card};

// const Colors = ["W", "U", "B", "G", "R"];

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Colors {
    Red,
    Black,
    Blue,
    Green,
    White,
    Colorless,
    Multicolor,
}

fn color_to_char(color: &Colors) -> char {
    match color {
        Colors::Red => 'R',
        Colors::Black => 'B',
        Colors::Blue => 'U',
        Colors::Green => 'G',
        Colors::White => 'W',
        Colors::Colorless => panic!("Colorless has no char value"),
        Colors::Multicolor => panic!("Multicolor has no char value"),
    }
}

// const ColorChar: [char; 5] = ['R', 'B', 'U', 'W', 'G'];  

pub fn generateDeck<'a>(input_cards: &'a Vec<CardInput>,
                    colors: &'a Vec<Colors>,
                    min_summon_spells: u32,
                    deck_size: u32) -> Vec<Card<'a>> {

    // Hash.
    let mut cards_map: HashMap<Colors, Vec<CardInput>> = HashMap::new();

    // Find multicolor cards.
    for i in input_cards {

        // Multicolor.
        if i.color.len() > 1 {
            let mut contains = false;

            // For each color in card.
            for c in i.color.chars() {

                let mut found_color = false;

                // Card color belongs to the deck color.
                for deck_color in colors {
                    // We found a color match.
                    if c == color_to_char(deck_color) {
                        found_color = true;
                        break;
                    }
                }
                if !found_color { contains = false; break; }
                else { contains = true; }
            }

            // We found a multicolor card that matches our deck colors.
            if contains {
                if cards_map.get(&Colors::Multicolor).is_none() {
                    cards_map.insert(Colors::Multicolor, Vec::new());
                }
                cards_map.get_mut(&Colors::Multicolor).unwrap().push(i.clone());
            }
        }

        // It's not a multicolor card.
        else {
            if i.color.len() == 1 {

                // Does the color match deck color.
                for deck_color in colors.iter() {
                    // We found a card with right color!
                    if color_to_char(deck_color) == i.color.chars().nth(0).unwrap() {
                        if cards_map.get(&deck_color).is_none() {
                            cards_map.insert(*deck_color, Vec::new());
                        }
                        cards_map.get_mut(&deck_color).unwrap().push(i.clone());
                        break;
                    }
                }
            }
            // Artifact or land.
            else {
                // Get or insert!!!
                if cards_map.get(&Colors::Colorless).is_none() {
                    cards_map.insert(Colors::Colorless, Vec::new());
                }
                cards_map.get_mut(&Colors::Colorless).unwrap().push(i.clone());
                
            }
        }
    }

    //println!("{:?}", cards_map);

    // The deck.
    let mut cards: Vec<Card> = Vec::new();

    let mut card_counter = 0;

    let mut artifact_count = 10;
    let cards_per_color: u32 = (deck_size - artifact_count) / colors.len() as u32;

    // Rare (40%, uncommon 30%, common 30%) 

    let rare_prob = 40;
    let uncommon_prob = 30;
    let common_prob = 30;

    let mut rng = thread_rng();

    // Add artifacts.
    cards_map.entry(Colors::Colorless).or_insert(Vec::<CardInput>::new());
    let artifacts = cards_map.get(&Colors::Colorless).unwrap().clone();
    let mut artifacts_rares = artifacts.clone().into_iter().filter(|c| c.rarity == "R").map(|c| c.clone()).collect::<Vec<_>>();
    let mut artifacts_uncommons = artifacts.clone().into_iter().filter(|c| c.rarity == "U").map(|c| c.clone()).collect::<Vec<_>>();
    let mut artifacts_commons = artifacts.clone().into_iter().filter(|c| c.rarity == "C").map(|c| c.clone()).collect::<Vec<_>>();

    let mut rares = 0;
    let mut uncommons = 0;
    let mut commons = 0;

    while artifact_count > 0 && (artifacts_rares.len() + artifacts_uncommons.len() + artifacts_commons.len() > 0) {

         let propability = rng.gen_range(0..100);
         println!("propability == {:?}", propability);
         println!("artifacts_rares.len() == {:?}", artifacts_rares.len());
         println!("artifacts_uncommons.len() == {:?}", artifacts_uncommons.len());
         println!("artifacts_commons.len() == {:?}", artifacts_commons.len());

         // Rare
         if propability < 40 && artifacts_rares.len() > 0 { 
            println!("Rare. {:?}", artifact_count);
            let index = rng.gen_range(0..artifacts_rares.len());
            let the_card = artifacts_rares.swap_remove(index);
            cards.push(
                    Card { name: Name {id: the_card.imagefile.clone().into(),
                    name: the_card.name.clone().into()},
                    set: Set { name: the_card.set.clone().into()},
                    });

            artifact_count -= 1;
         }
         // Uncommon
         else if propability < 70 && artifacts_uncommons.len() > 0 { 
            println!("Uncommon. {:?}", artifact_count);
                let index = rng.gen_range(0..artifacts_uncommons.len());
                let the_card = artifacts_uncommons.swap_remove(index);
                cards.push(
                        Card { name: Name {id: the_card.imagefile.clone().into(),
                        name: the_card.name.clone().into()},
                        set: Set { name: the_card.set.clone().into()},
                        });
                artifact_count -= 1;
         }
         // Common
         else {
            if artifacts_commons.len() == 0 { continue }
            println!("Common. {:?}", artifact_count);
            let index = rng.gen_range(0..artifacts_commons.len());
            let the_card = artifacts_commons.swap_remove(index);
            cards.push(
                    Card { name: Name {id: the_card.imagefile.clone().into(),
                    name: the_card.name.clone().into()},
                    set: Set { name: the_card.set.clone().into()},
                    });
            artifact_count -= 1;
         }
         println!("pah");
    }

    // for i in 0..cmp::min(artifacts.len() as u32, artifact_count as u32) {
    //     let ind = rng.gen_range(0..cards_map.get.len()); 
    // }

    //    get(&Colors::Colorless).unwrap_or_else(|| &Vec::<CardInput>::new()); 

    // Get the multicolor cards that match colors. TODO if colors > 1.
    // let actual_multicolor = Vec<_> = input_cards.iter().filter(|c| c.color.len() > 1).collect(); 

    println!("{:?}", cards);
    
    cards
}
