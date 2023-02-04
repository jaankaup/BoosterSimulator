//use crate::BoosterSimulator::cards::*;
use boosterSimulator::cards::*;
use dioxus::prelude::*;
use serde_xml_rs::{from_str, to_string};
use core::mem::replace;
use std::fs;


fn main() {
//    pub struct Listofcarddatafiles {
//    pub struct Filetoinclude  {
    //let card = Point {x:1, y:2}; 
    // let card_lists: Listofcarddatafiles  = from_str(&load_list_of_cards().unwrap()).unwrap();
    // to_lackey_test();
    //println!("{:?}", card_lists);
    //println!("{:?}", load_list_of_cards().unwrap());
    //load_mtg();

    //dioxus_desktop::launch(app);
    let boosters = vec![
	    Booster { set: "4e".to_string(), amount: 10, }, 
    ];
    let mut sets = load_mtg();
    let cards =  buy_boosters(&boosters, &mut sets);
    let lackey_filu = to_lackey(&cards);
    fs::write("eikujoo.dek", lackey_filu).expect("Unable to write file");
    //let lackey_filu_siisitty = lackey_filu.replace(&"\\\"", &"\"");
    //println!("{:?}", lackey_filu_siisitty);
    //println!("{:?}", r#"heko="just joo"#);
    
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}
