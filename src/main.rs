use std::io::Read;
use std::fs::File;
use toml::*;
use std::collections::HashMap;
use booster_simulator::cards::*;
use dioxus::prelude::*;
use std::fs;

#[derive(Props)]
struct AppStateProps<'a> {
    sets: &'a HashMap<String, Vec<CardInput>>,
    boosters: &'a Vec<Booster>,
    //initialized: bool,
}

fn AppState<'a>(cx: Scope<'a, AppStateProps<'a>>) -> Element {
    let mut sets = load_mtg();
    let boosters = vec![
            Booster { set: "alpha".to_string(), amount: 5, },
            Booster { set: "mirage".to_string(), amount: 5, },
            Booster { set: "mirage".to_string(), amount: 5, },
            Booster { set: "visions".to_string(), amount: 5, },
            Booster { set: "ia".to_string(), amount: 10, },
            Booster { set: "5e".to_string(), amount: 10, },
            Booster { set: "homelands".to_string(), amount: 5, },
    ];
    let cards =  buy_boosters(&boosters, &mut sets);
    let lackey_filu = to_lackey(&cards);
    let mut app_state = AppStateProps { sets: &sets, boosters: &boosters };

    let mut count = use_state(&cx, || 0);

    // cx.render(rsx!(
    //     ul {
    //         booster_list
    //     }
    // ))

    cx.render(rsx!(
        div {
            p {
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    ))
}

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

    let mut f = File::open("booster_config.toml").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let booster_conf: TomlConfig = toml::from_str(&buffer).unwrap(); 

    // let boosters = vec![
    //         Booster { set: "alpha".to_string(), amount: 1, },
    //         Booster { set: "beta".to_string(), amount: 1, },
    //         Booster { set: "mirage".to_string(), amount: 1, },
    //         Booster { set: "mirage".to_string(), amount: 1, },
    //         Booster { set: "visions".to_string(), amount: 1, },
    //         Booster { set: "ia".to_string(), amount: 1, },
    //         Booster { set: "5e".to_string(), amount: 1, },
    //         Booster { set: "homelands".to_string(), amount: 1, },
    //         Booster { set: "an".to_string(), amount: 1, },
    //         Booster { set: "dark".to_string(), amount: 1, },
    //         Booster { set: "unlimited".to_string(), amount: 1, },
    //         Booster { set: "legends".to_string(), amount: 1, },
    // ];

    let mut sets = load_mtg();
    let cards = buy_boosters(&booster_conf.boosters, &mut sets);
    let lackey_filu = to_lackey(&cards);
    // let app_state = AppState { sets: &sets, boosters: &boosters };

    fs::write(booster_conf.file_name, lackey_filu).expect("Unable to write file");
    //let to_toml = TomlConfig { file_name: "mun_pakka.dek".to_string(), boosters: boosters, };

    // fs::write("booster_config.toml", toml::to_string(&to_toml).unwrap()).expect("Unable to write file");
    // dioxus_desktop::launch(app);
}


fn app(cx: Scope) -> Element {
    let mut initialized = use_state(&cx, || true);
    // let boosters = vec![
    //         Booster { set: "alpha".to_string(), amount: 0, },
    //         Booster { set: "mirage".to_string(), amount: 0, },
    //         Booster { set: "mirage".to_string(), amount: 0, },
    //         Booster { set: "visions".to_string(), amount: 0, },
    //         Booster { set: "ia".to_string(), amount: 0, },
    //         Booster { set: "5e".to_string(), amount: 0, },
    //         Booster { set: "homelands".to_string(), amount: 0, },
    // ];
    // let booster_list = boosters.iter().map(|booster| rsx!(
    //     li {
    //         div {
    //             p { "{booster.set}" }
    //             p { "{booster.amount}"}
    //         }
    //     }
    // ));

    let mut count = use_state(&cx, || 0);

    // cx.render(rsx!(
    //     ul {
    //         booster_list
    //     }
    // ))

    cx.render(rsx!(
        div {
            p {
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    ))
}

// fn app(cx: Scope) -> Element {
//     cx.render(rsx! (
//         div { "Hello, world!" }
//     ))
// }
