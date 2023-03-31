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
    SharedBoosters,
};


fn BoosterApp(cx: Scope<AppStateProps>) -> Element {
    use_shared_state_provider(cx, || Points(80.0));

    // New!
    use_shared_state_provider(cx, || SharedBoosters(cx.props.boosters.clone()));

    let boosters = use_state(&cx, || cx.props.boosters.clone());
    let name = use_state(cx, || "pahapakka.dek".to_string());
    let points_left = use_shared_state::<Points>(cx).unwrap();

    // New!
    let shared_boosters_main = use_shared_state::<SharedBoosters>(cx).unwrap();
    cx.render(rsx!(
                  p { "Points left {(*points_left.read()).0}" }
                  p {
                      input {
                          // we tell the component what to render
                          value: "{name}",
                          // and what to do when the value changes
                          oninput: move |evt| name.set(evt.value.clone()),
                      }
                  }
                  p {
                      button {
                          onclick: move |event| {
                                                                     
                          //let cards = buy_boosters(&boosters, &mut cx.props.sets.clone());
                          //let lackey_filu = to_lackey(&buy_boosters(&boosters.get(), &mut cx.props.sets.clone()));
                          let mut path = "decks/".to_string();
                          path.push_str(name.get());
                          let lackey_filu = to_lackey(&buy_boosters(&(shared_boosters_main.read().0), &mut cx.props.sets.clone()));
                          fs::write(path, lackey_filu).expect("Unable to write file.");

                          },
                          "Buy boosters"
                      }
                  }
                  boosters.iter().enumerate().map(|(i,b)|
                      rsx!{BoosterComponent { booster: b, index: i, }}
                  ))
    )
}

fn main() {

    let mut f = File::open("booster_config.toml").expect("Couldn't find 'booster_config_toml'.");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut booster_conf: TomlConfig = toml::from_str(&buffer).unwrap(); 

    let mut sets = load_mtg();
    // println!("Creating boosters.");

    // let cards = buy_boosters(&booster_conf.boosters, &mut sets);
    // let lackey_filu = to_lackey(&cards);

    // println!("Saving deck to {:?}.", booster_conf.file_name);
    // fs::write(booster_conf.file_name, lackey_filu).expect("Unable to write file.");
    // println!("Done!");

    let app_props = AppStateProps { 
        sets: sets,
        boosters: booster_conf.boosters,
    };
    dioxus_desktop::launch_with_props(BoosterApp, app_props, Config::new());

}

