use std::cell::Cell;
use dioxus::prelude::*;
use crate::cards::Booster;

// #[derive(PartialEq, Props)]
// pub struct BoosterProps {
//     pub set: String,
//     pub amount: u32,
// }

//pub fn BoosterComponent(cx: Scope<Booster>, booster: UseRef<Booster>) -> Element {
#[inline_props]
pub fn BoosterComponent(cx: Scope, booster: Booster) -> Element {
    let mut count = use_state(cx, || 0);
    let minus_val = if *count.get() == 0 { 0 } else { 1 };

    let name_style = r#"
        width: 400px;
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
    cx.render(rsx!(
        p {
            style: "{container_style}",
            p {
                style: "{name_style}",
                "{booster.set}"
            }
            button {
                style: "{plus_style}",
                onclick: move |_| { count += 1 }, "+",
            },
            button {
                style: "{minus_style}",
                onclick: move |_| { count -= minus_val }, "-",
            },
            p {
                style: "{count_style}",
                "{count}"
            },
        }
    ))
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
