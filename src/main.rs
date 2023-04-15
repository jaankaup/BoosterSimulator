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
    CardImage,
};
use booster_simulator::random_deck::{Colors};

const card_images_style: &str = r#"
    margin: 5px;
    display: flex;
    color: red;
    background-color: rgb(235,235, 255);
    position: relative;
    align-items: center;
    border: 1px solid red;
   "#;
    // width: 1900px;
    // width: fit-content;


#[allow(non_snake_case)]
fn BoosterApp(cx: Scope<AppStateProps>) -> Element {

    // Top level properties than can be accessed from child elements.
    use_shared_state_provider(cx, || Points(0.0));
    use_shared_state_provider(cx, || SharedBoosters(cx.props.boosters.clone()));

    // Properties used are owned by this element.
    let boosters = use_state(cx, || cx.props.boosters.clone());
    let name = use_state(cx, || cx.props.deckname.clone());
    let points_used = use_shared_state::<Points>(cx).unwrap();
    let shared_boosters_main = use_shared_state::<SharedBoosters>(cx).unwrap();
    let shared_deck = use_state(cx, || Vec::<String>::new());
    let red_checked = use_state(cx, || false);
    let black_checked = use_state(cx, || false);
    let blue_checked = use_state(cx, || false);
    let green_checked = use_state(cx, || false);
    let white_checked = use_state(cx, || false);

    // Render.
    cx.render(rsx!(
                  p { "Points used {(*points_used.read()).0}" }
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
                              fs::write(path.clone(), lackey_filu).expect("Unable to write file.");
                              println!("Created file '{:?}'", path);
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
                              if !colors.is_empty() {
                                  let shared_b = shared_boosters_main.read().0.to_owned();
                                  let mut sets_b = cx.props.sets.to_owned();
                                  let deck_cards = buy_boosters(&(shared_b),
                                                                 &mut sets_b,
                                                                 true,
                                                                 colors);
                                  let lackey_filu = to_lackey(&deck_cards, true);
                                  let deck_images: Vec<_> = deck_cards.into_iter().map(|card| ("sets/setimages/".to_owned() +
                                                                                                &card.set.name.into_owned() + "/" + 
                                                                                                &card.name.id.into_owned() +
                                                                                                ".jpg"
                                                                                                )).collect();
                                  shared_deck.set(deck_images);
                                  // //let lackey_filu = to_lackey(&buy_boosters(&(shared_boosters_main.read().0),
                                  // //                                          &mut cx.props.sets.clone(),
                                  // //                                          true,
                                  // //                                          colors), true);
                                  // fs::write(path.clone(), lackey_filu).expect("Unable to write file.");
                                  // println!("Created file '{:?}'", path);
                              }
                              else { println!("Please choose at least one color."); }
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
                      div  {
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
                  p {
                    boosters.iter().enumerate().map(|(i,b)|
                        rsx!{BoosterComponent { booster: b, index: i, }}
                    )
                  }
                  p {
                      div {
                        style: "card_images_style",
                        
                        shared_deck.iter().map(|i_file|
                            rsx!{CardImage { image_file: i_file, }}
                            //CardImage { image_file: i_file}
                        )
                      }
                  }
        ) // !rxt
    ) // render
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
