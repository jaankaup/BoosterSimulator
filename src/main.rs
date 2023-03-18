use booster_simulator::booster_list::AppStateProps;
use crate::dioxus_elements::map;
use std::rc::Rc;
use dioxus_desktop::Config;
use std::io::Read;
use std::fs::File;
// use toml::*;
use std::collections::HashMap;
use booster_simulator::cards::*;
use dioxus::prelude::*;
use std::fs;
// use booster_simulator::booster_list::*;
use booster_simulator::components::{
    //Boosters;
    BoosterComponent,
    Points,
};

// #[derive(Props)]
// struct AppStateProps<'a> {
//     sets: &'a HashMap<String, Vec<CardInput>>,
//     boosters: &'a Vec<Booster>,
// }


// #[inline_props]
// fn App(cx: Scope, boosters: Vec<Booster>) -> Element {
// 
//     let state = use_state(cx, || boosters);
//     //let state = use_state(cx, || vec![Booster { set: "5e".to_string(), amount: 2, }, Booster { set: "6e".to_string(), amount: 1, }]);
// 
//     cx.render(rsx!(
//                   state.iter().map(|b|
//                       rsx!{BoosterComponent { booster: b.clone() }}
//                   ))
//     )
// }


fn BoosterApp(cx: Scope<AppStateProps>) -> Element {
    use_shared_state_provider(cx, || Points(45));
    let boosters = use_state(&cx, || cx.props.boosters.clone());
    let points_left = use_shared_state::<Points>(cx).unwrap();
    cx.render(rsx!(
                  p { "Points left {(*points_left.read()).0}" }
                  boosters.iter().map(|b|
                      rsx!{BoosterComponent { booster: b.clone() }}
                  ))
    )
}

fn main() {

    let mut f = File::open("booster_config.toml").expect("Couldn't find 'booster_config_toml'.");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut booster_conf: TomlConfig = toml::from_str(&buffer).unwrap(); 

    let mut sets = load_mtg();
    println!("Creating boosters.");

    let cards = buy_boosters(&booster_conf.boosters, &mut sets);
    let lackey_filu = to_lackey(&cards);
    // let app_state = AppState { sets: &sets, boosters: &boosters };

    println!("Saving deck to {:?}.", booster_conf.file_name);
    fs::write(booster_conf.file_name, lackey_filu).expect("Unable to write file.");
    println!("Done!");

    let app_props = AppStateProps { 
        sets: sets,
        boosters: booster_conf.boosters,
    };
    dioxus_desktop::launch_with_props(BoosterApp, app_props, Config::new());

    //dioxus_desktop::launch_with_props(App, AppProps { boosters: booster_conf.boosters });

    //let to_toml = TomlConfig { file_name: "mun_pakka.dek".to_string(), boosters: boosters, };

    // fs::write("booster_config.toml", toml::to_string(&to_toml).unwrap()).expect("Unable to write file");
    // let app_props = AppStateProps { 
    //     sets: sets,
    //     boosters: booster_conf.boosters,
    // };
    // dioxus_desktop::launch_with_props(BoosterApp, app_props, Config::new());
}

// #[inline_props]
// fn Boo(cx: Scope, booster: Booster) -> Element {
//     let set = use_ref(&cx, || booster.set.clone()); 
//     let amount = use_ref(&cx, || booster.amount); 
//     cx.render(rsx!{
//         h1 { "Erki" }
//     })
// }
// 
// //fn BoosterApp<'a>(cx: Scope<'a, AppStateProps>) -> Element<'a> {
// fn BoosterApp(cx: Scope<AppStateProps>) -> Element {
//     //let sets = use_ref(&cx, || cx.props.sets.clone());
//     //let boosters = use_ref(&cx, || cx.props.boosters.iter().map(|b| 1).collect::<Vec<_>>());
//     let boosters = use_ref(&cx, || cx.props.boosters.clone());
//     //let boosters = use_ref(&cx, || cx.props.boosters.iter().map(|b| use_ref(&cx, || b.clone())).collect::<Vec<_>>());
//     //let boosters = use_state(&cx, || cx.props.boosters.iter().map(|b| use_ref(&cx, || b.clone())).collect::<Vec<_>>());
//     //let boosters = use_ref(&cx, || cx.props.boosters.iter().map(|b| Rc::<Booster>::new(b.clone())).collect::<Vec<_>>());
//     //let boosters = use_ref(&cx, || cx.props.boosters.clone());
//     //let boosters = use_ref(&cx, || cx.props.boosters.clone());
// 
//     cx.render(rsx! {
//                  div {
//                      ul {
//                         boosters.read().iter().map(|b|
//                            rsx! {
//                               li {
//                                   div {
//                                       //booster_rsx(cx, b)
//                                       booster_rsx { cx, booster: b.clone() }
//                                       //button { onclick: move |_| { let mut bb = b.clone(); *Rc::make_mut(&mut bb) = Booster { set: "erkki".to_string(), amount: 5, }; } } // cx.needs_update(); },  "+" }
//                                       //button { onclick: move |_| { *Rc::get_mut(&mut b).unwrap() = Booster { set: "erkki".to_string(), amount: 5, }; } } // cx.needs_update(); },  "+" }
//                                       //button { onclick: move |_| { b.make_mut().push(Booster { set: "erkki".to_string(), amount: 5, }); } } // cx.needs_update(); },  "+" }
//                                       //"{b.set} :: {b.amount}"
//                                       //Boo(b.clone())
//                                       "Erkki"
//                                       //button { onclick: move |_| { let x = use_ref(&cx, || b.clone(); }, "Oolrait" } // cx.needs_update(); },  "+" }
//                                   }
//                               }
//                            })
//                      }
//                  }
//     })
// }
