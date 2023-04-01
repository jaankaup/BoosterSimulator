use std::cell::Cell;
use dioxus::prelude::*;
use crate::cards::Booster;
use std::collections::HashMap;
use crate::cards::CardInput;

#[derive(PartialEq, Props)]
pub struct AppStateProps {
    pub sets: HashMap<String, Vec<CardInput>>,
    pub boosters: Vec<Booster>,
    pub points: f32,
    pub deckname: String,
}

#[derive(PartialEq, Props)]
pub struct BoosterProps {
    pub set: String,
    pub amount: u32,
}

//pub fn booster_rsx(cx: Scope<BoosterProps>, booster: UseRef<Booster>) -> Element {
pub fn booster_rsx(cx: Scope<AppStateProps>, booster: UseRef<Booster>) -> Element {

    cx.render(rsx!{
        "hellou"
    })
}
// 
//     let mut set: &Cell<String> = cx.use_state(|| booster. );
//     let mut amount = cx.use_state(|| 0);
// }
//     // let mut count = cx.use_hook(|| 0);
//     // let mut count: &Cell<i32> = cx.use_ref(&cx, Cell::new(0));
//     //let mut count: &Cell<i32> = cx.use_hook(|| Cell::new(0));
//     //let mut count = cx.use_state(|| 0);
//     cx.render(rsx!{
//                   div {
//                       //"{cx.props.booster.set} :: {cx.props.booster.amount}"
//                       button { onclick: move |_| { cx.write(cx.read() + 1); cx.needs_update(); },  "+" }
//                       // button { onclick: move |_| { if count.get() != 0 { count.set(count.get() - 1); cx.needs_update(); } },  "-" }
//                       //"{cx.props.booster.set} :: {count.get()}"
//                       "Hekotus"
//                   }
//               }
//     )
// }
