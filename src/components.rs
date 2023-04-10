use dioxus::prelude::*;
use crate::cards::Booster;

pub struct Points(pub f32);
pub struct SharedBoosters(pub Vec<Booster>);

#[allow(non_snake_case)]
#[inline_props]
pub fn BoosterComponent<'a>(cx: Scope<'a>, booster: &'a Booster, index: usize) -> Element {
    let points_used = use_shared_state::<Points>(cx).unwrap();
    let shared_booster = use_shared_state::<SharedBoosters>(cx).unwrap();

    let name_style = r#"
        padding-left: 10px;
        width: 400px;
    "#;
    let price_style = r#"
        margin-right: 10px;
        width: 50px;
        color: white;
        background-color: rgb(125,75,15);
    "#;
    let plus_style = r#"
        margin-right: 10px;
        width: 50px;
        color: white;
        background-color: rgb(225,15,15);
    "#;
    let minus_style = r#"
        margin-right: 10px;
        width: 50px;
        color: white;
        vertical-align: middle;
        background-color: rgb(0,15,225);
    "#;
    let count_style = r#"
        margin-right: 10px;
        width: 50px;
        color: black;
        background-color: rgb(100,255,100);
    "#;
    let container_style = r#"
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

    let amount = ((*shared_booster.read().0)[*index]).amount;

    // How much to decrease booster count when pressing (-).
    let minus_val = if amount == 0 { 0 } else { 1 };
    //let minus_val = if *count.get() == 0 { 0 } else { 1 };

    // The new points_left value if (+) pressed.
    let new_points_plus =  points_used.read().0 + booster.price; 

    let new_points_minus =  points_used.read().0 - booster.price; 

    // The new points_left value if (-) pressed.
    let new_minus = if new_points_minus < 0.0 { points_used.read().0 } else { new_points_minus }; 


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
                onclick: move |_| { ((*shared_booster.write().0)[*index]).amount = amount + 1; points_used.write().0 = new_points_plus; }, "+", // inc_booster; points_used.write().0 = new_points_plus; }, "+",
            },
            button {
                style: "{minus_style}",
                onclick: move |_| { ((*shared_booster.write().0)[*index]).amount = amount - minus_val; points_used.write().0 = new_minus; }, "-", // points_used.write().0 = new_points_minus; }, "-",
            },
            p {
                style: "{count_style}",
                //"{count}"
                "{amount}"
            },
        }
    })
}
