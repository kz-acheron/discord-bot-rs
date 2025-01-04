use crate::{Data, Error};

mod examples;

pub fn register() -> Vec<poise::Command<Data, Error>> {
    vec![examples::button(),examples::select_menu()]
}
