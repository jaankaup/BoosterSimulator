use std::cell::Cell;
use dioxus::prelude::*;
use crate::cards::Booster;

pub struct Points(pub f32);

// pub struct Boosters(Vec<Booster>);

// #[derive(PartialEq, Props)]
// pub struct BoosterProps {
//     pub set: String,
//     pub amount: u32,
// }

//pub fn BoosterComponent(cx: Scope<Booster>, booster: UseRef<Booster>) -> Element {
#[inline_props]
//pub fn BoosterComponent<'a>(cx: Scope<'a>, booster: &'a Booster) -> Element<'a> {
pub fn BoosterComponent(cx: Scope, booster: Booster) -> Element {
    let mut count = use_state(cx, || 0);
    let points_left = use_shared_state::<Points>(cx).unwrap();

    // let points_minus_val = minus_val * booster_cost + (*points_left.read()).0;
    // let points_plus_val = (*points_left.read()).0 + minus_val;


    let name_style = r#"
        width: 400px;
    "#;
    let price_style = r#"
        width: 50px;
        color: white;
        background-color: rgb(125,75,15);
    "#;
    let plus_style = r#"
        width: 50px;
        color: white;
        background-color: rgb(225,15,15);
    "#;
    let minus_style = r#"
        width: 50px;
        color: white;
        vertical-align: middle;
        background-color: rgb(0,15,225);
    "#;
    let count_style = r#"
        width: 50px;
        color: black;
        background-color: rgb(100,255,100);
    "#;
    let container_style = r#"
        display: inline-flex;
        color: red;
        background-color: rgb(0,200, 255);
        position: relative;
        width: fit-content;
        align-items: center;
        border: 1px solid red;
        width: 475px;
       "#;


    // How much to decrease booster count when pressing (-).
    let minus_val = if *count.get() == 0 { 0 } else { 1 };

    // How much to increase booster count whe pressing (+).
    let inc_booster = if (*points_left.read()).0 >= booster.price { 1 } else { 0 };

    // The new points_left value if (+) pressed.
    let new_points_plus =  (*points_left.read()).0 - inc_booster as f32 * booster.price; 

    // The new points_left value if (-) pressed.
    let new_points_minus =  (*points_left.read()).0 + minus_val as f32 * booster.price; 
    //let new_points_minus =  (*points_left.read()).0 + TryInto::<f32>::try_into(minus_val).unwrap() * booster.price; 

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
                onclick: move |_| { count += inc_booster; (*points_left.write()).0 = new_points_plus;  }, "+",
            },
            button {
                style: "{minus_style}",
                onclick: move |_| { count -= minus_val; (*points_left.write()).0 = new_points_minus; }, "-",
            },
            p {
                style: "{count_style}",
                "{count}"
            },
        }
    })
}

fn Yeah(cx: Scope) -> Element {
    let list = use_ref(cx, Vec::new);

    cx.render(rsx!(
        p { "Current list: {list.read():?}" }
        button {
            onclick: move |event| {
                list.with_mut(|list| list.push(event));
            },
            "Click me!"
        }
        button {
            onclick: move |event| {
                list.with_mut(|list| list.push(event));
            },
            "Click me2!"
        }
    ))
}
