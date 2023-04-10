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

//pub fn generateDeck<'a>(input_cards: &'a Vec<CardInput>,
pub fn generateDeck<'a>(input_cards: Vec<CardInput>,
                    colors: Vec<Colors>,
                    //colors: &'a Vec<Colors>,
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
                for deck_color in &colors {
                    // We found a color match.
                    if c == color_to_char(&deck_color) {
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

    // The deck.
    let mut cards: Vec<Card> = Vec::new();

    let mut artifact_count = 10;
    let mut multicolor_count = 10;

    // Rare (40%, uncommon 30%, common 30%) 

    let rng = thread_rng();

    let mut summons_ok = false;
    let mut tries = 0;

    // Create decsk as long as there are enough summon spells.
    while !summons_ok && tries < 100 {

        tries += 1;
        let mut summon_cards = 0;

        // Add artifacts.
        cards_map.entry(Colors::Colorless).or_insert(Vec::<CardInput>::new());

        let artifacts = cards_map.get(&Colors::Colorless).unwrap().clone();
        let mut artifacts_to_deck = generate_cards(&artifacts, 10, deck_size, [40,30,30]);

        cards_map.entry(Colors::Multicolor).or_insert(Vec::<CardInput>::new());


        let multicolors = cards_map.get(&Colors::Multicolor).unwrap().clone();
        let mut multicolors_to_deck = generate_cards(&multicolors, 10, deck_size, [40,30,30]);

        let cards_remaining = deck_size - artifacts_to_deck.0.len() as u32 - multicolors_to_deck.0.len() as u32;
        let cards_per_color = cards_remaining / colors.len() as u32;

        cards.append(&mut artifacts_to_deck.0);
        cards.append(&mut multicolors_to_deck.0);

        summon_cards += artifacts_to_deck.1;
        summon_cards += multicolors_to_deck.1;

        for c in &colors {
            cards_map.entry(*c).or_insert(Vec::<CardInput>::new());
            let color_cards = cards_map.get(&c).unwrap().clone();
            let mut random_color_cards = generate_cards(&color_cards, cards_per_color, deck_size, [40,30,30]);
            cards.append(&mut random_color_cards.0);
            summon_cards += random_color_cards.1;
        }

        // Check for summons cards.
        if summon_cards >= min_summon_spells {  
            summons_ok = true;
        }
        else {
            cards.clear();
        }
    }

    // artifacts_to_deck
    if tries == 100 { println!("Tried 100 times. Not enough creatures. Buy some more boosters!"); }
    else {
        println!("A random Deck created ({:?} cards).", cards.len());
    }
    cards
}

fn generate_cards(input_cards: &Vec<CardInput>, pref_count: u32, remaining_count: u32, propabilies: [u32; 3]) -> (Vec<Card<'static>>, u32)  {

    let mut result = Vec::<Card>::new();

    let mut rares = input_cards.clone().into_iter().filter(|c| c.rarity == "R").map(|c| c.clone()).collect::<Vec<_>>();
    let mut uncommons = input_cards.clone().into_iter().filter(|c| c.rarity == "U").map(|c| c.clone()).collect::<Vec<_>>();
    let mut commons = input_cards.clone().into_iter().filter(|c| c.rarity == "C").map(|c| c.clone()).collect::<Vec<_>>();

    let mut count = pref_count;
    let mut deck_card_count = pref_count;

    let mut summon_cards = 0;

    let mut rng = thread_rng();

    let prop_rare = propabilies[0];
    let prop_uncommon = prop_rare + propabilies[1];
    let prop_common = prop_uncommon + propabilies[2];

    while count > 0 && (rares.len() + uncommons.len() + commons.len() > 0 && deck_card_count > 0 ) {

         let propability = rng.gen_range(0..100);

         // Rare
         if propability < prop_rare && rares.len() > 0 { 
            // println!("Rare. {:?}", count);
            let index = rng.gen_range(0..rares.len());
            let the_card = rares.swap_remove(index);

            // Skip ante cards.
            if the_card.text.to_lowercase().contains("ante") { continue; }

            if the_card.card_type.to_lowercase().contains("creature") { summon_cards += 1; }

            result.push(
                    Card { name: Name {id: the_card.imagefile.clone().into(),
                    name: the_card.name.clone().into()},
                    set: Set { name: the_card.set.clone().into()},
                    });
            deck_card_count -= 1;
            count -= 1;
         }
         // Uncommon
         else if propability < prop_uncommon && uncommons.len() > 0 { 
            let index = rng.gen_range(0..uncommons.len());
            let the_card = uncommons.swap_remove(index);

            // Skip ante cards.
            if the_card.text.to_lowercase().contains("ante") { continue; }

            if the_card.card_type.to_lowercase().contains("creature") { summon_cards += 1; }

            result.push(
                    Card { name: Name {id: the_card.imagefile.clone().into(),
                    name: the_card.name.clone().into()},
                    set: Set { name: the_card.set.clone().into()},
                    });
            deck_card_count -= 1;
            count -= 1;
         }
         // Common
         else {
            if commons.len() == 0 { continue }
            // println!("Common. {:?}", count);
            let index = rng.gen_range(0..commons.len());
            let the_card = commons.swap_remove(index);

            // Skip ante cards.
            if the_card.text.to_lowercase().contains("ante") { continue; }

            if the_card.card_type.to_lowercase().contains("creature") { summon_cards += 1; }

            result.push(
                    Card { name: Name {id: the_card.imagefile.clone().into(),
                    name: the_card.name.clone().into()},
                    set: Set { name: the_card.set.clone().into()},
                    });
            deck_card_count -= 1;
            count -= 1;
         }
    }
    (result, summon_cards)
}
