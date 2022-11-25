use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

#[function_component(CustomButton)]
pub fn cutom_button(props: &Props) -> Html {
    html! {
        <button class={"btn btn-success"}>{&props.text}</button>

    }
}
