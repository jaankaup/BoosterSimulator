use booster_simulator::cards::*;
use dioxus::prelude::*;
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


    // visions
    // mirage
    // ia
    // home
    // fe

    // let boosters = vec![
    //         Booster { set: "alpha".to_string(), amount: 5, },
    //         Booster { set: "mirage".to_string(), amount: 5, },
    //         Booster { set: "mirage".to_string(), amount: 5, },
    //         Booster { set: "visions".to_string(), amount: 5, },
    //         Booster { set: "ia".to_string(), amount: 10, },
    //         Booster { set: "5e".to_string(), amount: 10, },
    //         Booster { set: "homelands".to_string(), amount: 5, },
    // ];
    // let mut sets = load_mtg();
    // let cards =  buy_boosters(&boosters, &mut sets);
    // let lackey_filu = to_lackey(&cards);
    // fs::write("janne.dek", lackey_filu).expect("Unable to write file");

    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}
