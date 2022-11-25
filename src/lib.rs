// use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;
use components::organisms::user_login_form::UserLoginForm;

#[derive(Serialize, Deserialize)]
struct DummyStruct<'a> {
    name: &'a str,
    posture: &'a str,
}

// const STYLE_FILE: &str = include_str!("main.css");

#[function_component(App)]
pub fn app() -> Html {
    // let my_vec = vec!["1", "2", "3", "4"]
    //     .iter()
    //     .map(|x| html!(<li>{x}</li>))
    //     .collect::<Vec<Html>>();

    // let name = DummyStruct {
    //     name: "hello",
    //     posture: "world",
    // };

    // let stylesheet = stylist::Style::new(STYLE_FILE).unwrap();
    html! {
        <UserLoginForm/>
    }
}
