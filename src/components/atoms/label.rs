use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub fr: String,
    pub text: String,
}

#[function_component(Label)]
pub fn text_input(props: &Props) -> Html {
    // <label for="exampleInputEmail1">{"Email address"}</label>
    html! {
        <label for={props.fr.clone()}>{props.text.clone()}</label>
    }
}
