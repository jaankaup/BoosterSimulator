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
