use dioxus::prelude::*;
use crate::cards::Booster;

pub struct Points(pub f32);
pub struct SharedBoosters(pub Vec<Booster>);
// pub struct SharedDeckCards(pub Vec<Card>);

const image_style: &str = r#"
    padding-left: 10px;
    width: 200px;
    display: inline-block;
"#;

const name_style: &str = r#"
    padding-left: 10px;
    width: 400px;
"#;

const price_style: &str = r#"
    margin-right: 10px;
    width: 50px;
    color: white;
    background-color: rgb(125,75,15);
"#;
const plus_style: &str = r#"
    margin-right: 10px;
    width: 50px;
    color: white;
    background-color: rgb(225,15,15);
"#;
const minus_style: &str = r#"
    margin-right: 10px;
    width: 50px;
    color: white;
    vertical-align: middle;
    background-color: rgb(0,15,225);
"#;
const count_style: &str = r#"
    margin-right: 10px;
    width: 50px;
    color: black;
    background-color: rgb(100,255,100);
"#;
const container_style: &str = r#"
    margin: 5px;
    display: inline-flex;
    color: red;
    background-color: rgb(0,200, 255);
    position: relative;
    width: fit-content;
    align-items: center;
    border: 1px solid red;
    width: 475px;
   "#;

#[allow(non_snake_case)]
#[inline_props]
pub fn BoosterComponent<'a>(cx: Scope<'a>, booster: &'a Booster, index: usize) -> Element {
    let points_used = use_shared_state::<Points>(cx).unwrap();
    let shared_booster = use_shared_state::<SharedBoosters>(cx).unwrap();


    let amount = ((*shared_booster.read().0)[*index]).amount;

    // How much to decrease booster count when pressing (-).
    let minus_val = if amount == 0 { 0 } else { 1 };
    let new_points_plus =  points_used.read().0 + booster.price; 
    let new_points_minus =  points_used.read().0 - booster.price * minus_val as f32; 

    cx.render(rsx!{
        p {
            style: "{container_style}",
            p {
                style: "{name_style}",
                "{booster.set}"
            }
            p {
                style: "{price_style}",
                "{booster.price}"
            }
            button {
                style: "{plus_style}",
                onclick: move |_| { ((*shared_booster.write().0)[*index]).amount = amount + 1; points_used.write().0 = new_points_plus; },
                "+",
            },
            button {
                style: "{minus_style}",
                onclick: move |_| { ((*shared_booster.write().0)[*index]).amount = amount - minus_val; points_used.write().0 = new_points_minus; },
                "-",
            },
            p {
                style: "{count_style}",
                "{amount}"
            },
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn CardImage<'a>(cx: Scope<'a>, image_file: &'a String) -> Element {

    // let deck_cards = use_shared_state::<SharedDeckCards>(cx).unwrap();

    cx.render(rsx!{
        p {
            style: "{image_style}",
            img {
                src: "{image_file}",
                width: "200px",
                // width: "312px",
                // height: "445px",
            }
        }
    })
}
