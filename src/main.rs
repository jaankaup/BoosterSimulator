//use crate::BoosterSimulator::cards::*;
use boosterSimulator::cards::*;
use dioxus::prelude::*;
use serde_xml_rs::{from_str, to_string};

fn main() {
//    pub struct Listofcarddatafiles {
//    pub struct Filetoinclude  {
    //let card = Point {x:1, y:2}; 
    let card_lists: Listofcarddatafiles  = from_str(&load_list_of_cards().unwrap()).unwrap();
   to_lackey_test();
    //println!("{:?}", card_lists);
    //println!("{:?}", load_list_of_cards().unwrap());
    load_mtg();

    //dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}
