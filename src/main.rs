use booster_simulator::booster_list::AppStateProps;
use dioxus_desktop::Config;
use std::io::Read;
use std::fs::File;
use booster_simulator::cards::*;
use dioxus::prelude::*;
use std::fs;
use booster_simulator::components::{
    BoosterComponent,
    Points,
    SharedBoosters,
};
use booster_simulator::random_deck::{Colors};


#[allow(non_snake_case)]
fn BoosterApp(cx: Scope<AppStateProps>) -> Element {

    // Top level properties than can be accessed from child elements.
    use_shared_state_provider(cx, || Points(cx.props.points));
    use_shared_state_provider(cx, || SharedBoosters(cx.props.boosters.clone()));

    // Properties used are owned by this element.
    let boosters = use_state(cx, || cx.props.boosters.clone());
    let name = use_state(cx, || cx.props.deckname.clone());
    let points_left = use_shared_state::<Points>(cx).unwrap();
    let shared_boosters_main = use_shared_state::<SharedBoosters>(cx).unwrap();
    let red_checked = use_state(cx, || false);
    let black_checked = use_state(cx, || false);
    let blue_checked = use_state(cx, || false);
    let green_checked = use_state(cx, || false);
    let white_checked = use_state(cx, || false);

    let new_red = !red_checked;

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
                          onclick: move |_| {
                                                                     
                          let mut path = "decks/".to_string();
                          path.push_str(name.get());
                          let lackey_filu = to_lackey(&buy_boosters(&(shared_boosters_main.read().0),
                                                                    &mut cx.props.sets.clone(),
                                                                    false,
                                                                    vec![Colors::Black, Colors::Red]), false);
                          fs::write(path, lackey_filu).expect("Unable to write file.");

                          },
                          "Buy boosters"
                      }
                  }
                  p {
                      button {
                          onclick: move |_| {
                                                                     
                          let mut path = "decks/".to_string();
                          path.push_str(name.get());
                          let mut colors = Vec::<Colors>::new(); 
                          if *red_checked.get() { colors.push(Colors::Red); }
                          if *black_checked.get() { colors.push(Colors::Black); }
                          if *blue_checked.get() { colors.push(Colors::Blue); }
                          if *green_checked.get() { colors.push(Colors::Green); }
                          if *white_checked.get() { colors.push(Colors::White); }
                          if (colors.len() != 0) {
                          let lackey_filu = to_lackey(&buy_boosters(&(shared_boosters_main.read().0),
                                                                    &mut cx.props.sets.clone(),
                                                                    true,
                                                                    colors), true);
                            fs::write(path, lackey_filu).expect("Unable to write file.");
                            }
                          },
                          "Generate deck"
                      }
                  }
                  p {
                      div {
                        input {
                            r#type: "checkbox",
                            onclick: move |_| { red_checked.modify(|v| !v); },
                            id: "red",
                            value: "red",
                            name: "red",
                        }
                        label {
                            r#for: "red",
                            "red"

                        }
                      }
                  }
                  p {
                      div {
                        input {
                            r#type: "checkbox",
                            id: "black",
                            value: "black",
                            name: "black",
                            onclick: move |_| { black_checked.modify(|v| !v); },
                        }
                        label {
                            r#for: "black",
                            "black"

                        }
                      }
                  }
                  p {
                      div {
                        input {
                            r#type: "checkbox",
                            onclick: move |_| { blue_checked.modify(|v| !v); },
                            id: "blue",
                            value: "blue",
                            name: "blue",
                        }
                        label {
                            r#for: "blue",
                            "blue"

                        }
                      }
                  }
                  p {
                      div {
                        input {
                            r#type: "checkbox",
                            id: "green",
                            value: "green",
                            name: "green",
                            onclick: move |_| { green_checked.modify(|v| !v); },
                        }
                        label {
                            r#for: "green",
                            "green"

                        }
                      }
                  }
                  p {
                      div {
                        input {
                            r#type: "checkbox",
                            id: "white",
                            value: "white",
                            name: "white",
                            onclick: move |_| { white_checked.modify(|v| !v); },
                        }
                        label {
                            r#for: "white",
                            "white"
                        }
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

    let booster_conf: TomlConfig = toml::from_str(&buffer).unwrap(); 

    let sets = load_mtg();

    // These properties are used in BoosterApp.
    let app_props = AppStateProps { 
        sets,
        boosters: booster_conf.boosters,
        points: booster_conf.points as f32,
        deckname: booster_conf.file_name,
    };
    dioxus_desktop::launch_with_props(BoosterApp, app_props, Config::new());
}
