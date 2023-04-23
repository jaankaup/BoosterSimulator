use std::collections::HashMap;
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
    SharedImageDimension,
    CardImage,
};
use booster_simulator::random_deck::{Colors};
use booster_simulator::cards::{CardInput};

const points_used_style: &str = r#"
    width: 150px;
    padding-right: 5px;
    padding-left: 5px;
    background-color: rgb(255,255, 255);
    border: 1px solid red;
   "#;

const card_images_style: &str = r#"
    display: flex;
    color: red;
    background-color: rgb(235,235, 255);
    position: relative;
    align-items: center;
    border: 1px solid red;
   "#;

const checkbox_style: &str = r#"
    background-color: rgb(255,235, 255);
    padding-left: 5px;
    padding-right: 5px;
    color: black;
   "#;

const header_style: &str = r#"
    margin: 5px;
    display: flex;
    color: red;
    background-color: rgb(120,120, 255);
    position: relative;
    align-items: center;
    border: 1px solid red;
   "#;

const boosters_deck_container_style: &str = r#"
    display: flex;
    flex_direction: row;
    flex_wrap: wrap;
    align_items: flex-start;
    width: 100%;
   "#;

const style_boosters: &str = r#"
    width: 400px;
   "#;

const style_deck_cards: &str = r#"
    width: auto;
   "#;


#[allow(non_snake_case)]
fn BoosterApp(cx: Scope<AppStateProps>) -> Element {

    // Top level properties than can be accessed from child elements.
    use_shared_state_provider(cx, || Points(0.0));
    use_shared_state_provider(cx, || SharedBoosters(cx.props.boosters.clone()));
    use_shared_state_provider(cx, || SharedImageDimension((150, 214)));

    // Properties used are owned by this element.
    let boosters = use_state(cx, || cx.props.boosters.clone());
    let name = use_state(cx, || cx.props.deckname.clone());
    let random_cards = use_state(cx, || Vec::<CardInput>::new());
    let points_used = use_shared_state::<Points>(cx).unwrap();
    let shared_boosters_main = use_shared_state::<SharedBoosters>(cx).unwrap();
    let shared_image_dimensions = use_shared_state::<SharedImageDimension>(cx).unwrap();
    let shared_deck = use_state(cx, || Vec::<String>::new());
    let red_checked = use_state(cx, || false);
    let black_checked = use_state(cx, || false);
    let blue_checked = use_state(cx, || false);
    let green_checked = use_state(cx, || false);
    let white_checked = use_state(cx, || false);
    let image_dimensions_checked = use_state(cx, || false);

    // Render.
    cx.render(rsx!(
                // main div
                div {
                header { 
                  style: "{header_style}",
                  p { style:"{points_used_style}",  "Points used {(*points_used.read()).0}" }
                  p { input { value: "{name}", oninput: move |evt| name.set(evt.value.clone()), } }
                  p {
                      button {
                          onclick: move |_| {
                                                                     
                            let mut path = "decks/".to_string();
                            path.push_str(name.get());
                            let lackey_filu = to_lackey(&convert_cardinput(random_cards.get()), true);
                            fs::write(path.clone(), lackey_filu).expect("Unable to write file.");
                            println!("Created file '{:?}'", path);
                          },
                          "Save deck"
                      }
                  }
                  p {
                      button {
                          onclick: move |_| {
                                                                     
                              let mut path = "decks/".to_string();
                              path.push_str(name.get());
                              let lackey_filu = to_lackey(&convert_cardinput(random_cards.get()), true);
                              fs::write(path.clone(), lackey_filu).expect("Unable to write file.");
                              println!("Created file '{:?}'", path);
                          },
                          "Buy boosters"
                      }
                  }
                  p {
                      button {
                          onclick: move |_| {
                                                                     
                              // let mut path = "decks/".to_string();
                              //path.push_str(name.get());
                              let mut colors = Vec::<Colors>::new();
                              if *red_checked.get() { colors.push(Colors::Red); }
                              if *black_checked.get() { colors.push(Colors::Black); }
                              if *blue_checked.get() { colors.push(Colors::Blue); }
                              if *green_checked.get() { colors.push(Colors::Green); }
                              if *white_checked.get() { colors.push(Colors::White); }
                              if !colors.is_empty() {
                                  let shared_b = shared_boosters_main.read().0.to_owned();
                                  let mut sets_b = cx.props.sets.to_owned();
                                  let mut deck_cards = buy_boosters(&shared_b,
                                                                &mut sets_b,
                                                                true,
                                                                colors);
                                  //let deck_cards2 = deck_cards.clone();
                                  //let mut input_cards = Vec::<CardInput>::new();
                                   // // TODO: avoid cloning
                                   // let mut sets_c: HashMap<String, Vec<CardInput>> = sets_b.clone().into();
                                   // for x in sets_c.values_mut() {
                                   //     //random_cards.modify(|random_c| random_c.append(x)); //.append(x);
                                   //     input_cards.append(x);
                                   // }
                                   // let mut joopajoo = Vec::<Card>::new();
                                   // for x in deck_cards.iter() {
                                   //     joopajoo.push(x.clone());
                                   // }
                                   //let copied_deck_cards = deck_cards.to_owned();
                                   // let new_deck_cards = deck_cards.iter().map(|x| x.to_owned()).collect::<Vec<_>>();
                                   //random_cards.with_mut(|x| *x = joopajoo);
                                   //random_cards.set(input_cards);
                                   deck_cards.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap()); 
                                   deck_cards.sort_by(|a, b| a.color.partial_cmp(&b.color).unwrap()); 
                                   let mut deck_images: Vec<_> = deck_cards.clone().into_iter().map(|card| ("sets/setimages/".to_owned() +
                                                                                                &card.set.to_owned() + "/" + 
                                                                                                &card.imagefile.to_owned() +
                                                                                                ".jpg"
                                                                                                )).collect();
                                  //deck_images.sort();
                                  shared_deck.set(deck_images);

                                  random_cards.set(deck_cards.clone());

                                  // let lackey_filu = to_lackey(&convert_cardinput(&deck_cards), true);
                                  // fs::write(path.clone(), lackey_filu).expect("Unable to write file.");
                                  // println!("Created file '{:?}'", path);
                              }
                              else { println!("Please choose at least one color."); }
                          },
                          "Generate deck"
                      }
                  }

                  // CHECK BOXES

                      p {
                          div {
                            style: "{checkbox_style}",
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
                            style: "{checkbox_style}",
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
                            style: "{checkbox_style}",
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
                            style: "{checkbox_style}",
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
                            style: "{checkbox_style}",
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
                          div  {
                            style: "{checkbox_style}",
                            input {
                                r#type: "checkbox",
                                id: "zoom",
                                value: "zoom",
                                name: "zoom",
                                onclick: move |_| { image_dimensions_checked.modify(|v| !v);
                                                    if *image_dimensions_checked.get() {
                                                       shared_image_dimensions.write().0 = (150, 214);
                                                    }
                                                    else {
                                                       shared_image_dimensions.write().0 = (312, 445); 
                                                    }

                                },
                            }
                            label {
                                r#for: "zoom",
                                "zoom"
                            }
                          }
                      }

                  // CHECKBOXES END

                } // main header

                  // Boosters and deck container
                  div {
                      style: "{boosters_deck_container_style}",
                      p {
                        style: "{style_boosters}",
                        boosters.iter().enumerate().map(|(i,b)|
                            rsx!{BoosterComponent { booster: b, index: i, }}
                        )
                      }

                      // Deck cards
                      p {
                        style: "{style_deck_cards}",
                          div {
                            style: "card_images_style",
                            shared_deck.iter().map(|i_file|
                                rsx!{CardImage { image_file: i_file, }}
                            )
                          }
                      }
                  }
               } // main div
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
