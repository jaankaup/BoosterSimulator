use std::cell::Cell;
use dioxus::prelude::*;
use crate::cards::Booster;

#[derive(PartialEq, Props)]
pub struct BoosterProps {
    pub booster: Booster, 
}

pub fn booster_rsx(cx: Scope<BoosterProps>) -> Element {
    // let mut count = cx.use_hook(|| 0);
    let mut count: &Cell<i32> = cx.use_hook(|| Cell::new(0));
    //let mut count = cx.use_state(|| 0);
    cx.render(rsx!{
                  div {
                      //"{cx.props.booster.set} :: {cx.props.booster.amount}"
                      button { onclick: move |_| { count.set(count.get() + 1); cx.needs_update(); },  "+" }
                      button { onclick: move |_| { if count.get() != 0 { count.set(count.get() - 1); cx.needs_update(); } },  "-" }
                      "{cx.props.booster.set} :: {count.get()}"
                  }
              }
    )
}
