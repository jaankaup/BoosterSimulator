use booster_simulator::booster_list::AppStateProps;
use crate::dioxus_elements::map;
use std::rc::Rc;
use dioxus_desktop::Config;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use booster_simulator::cards::*;
use dioxus::prelude::*;
use std::fs;
use booster_simulator::components::{
    BoosterComponent,
    Points,
    SharedBoosters,
};

fn BoosterApp(cx: Scope<AppStateProps>) -> Element {

    // Top level properties than can be accessed from child elements.
    use_shared_state_provider(cx, || Points(cx.props.points));
    use_shared_state_provider(cx, || SharedBoosters(cx.props.boosters.clone()));

    // Properties used are owned by this element.
    let boosters = use_state(&cx, || cx.props.boosters.clone());
    let name = use_state(cx, || cx.props.deckname.clone());
    let points_left = use_shared_state::<Points>(cx).unwrap();
    let shared_boosters_main = use_shared_state::<SharedBoosters>(cx).unwrap();

    // Render.
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

    // Load deck informations and application settings.
    let mut f = File::open("booster_config.toml").expect("Couldn't find 'booster_config_toml'.");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut booster_conf: TomlConfig = toml::from_str(&buffer).unwrap(); 

    let mut sets = load_mtg();

    // These properties are used in BoosterApp.
    let app_props = AppStateProps { 
        sets: sets,
        boosters: booster_conf.boosters,
        points: booster_conf.points as f32,
        deckname: booster_conf.file_name,
    };
    dioxus_desktop::launch_with_props(BoosterApp, app_props, Config::new());
}
