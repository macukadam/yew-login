use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub typ: String,
    pub class: String,
    pub id: String,
    pub aria: String,
    pub placeholder: String,
}

#[function_component(Input)]
pub fn text_input(props: &Props) -> Html {
    html! {
        <input type={props.typ.clone()} class={props.class.clone()} id={props.id.clone()} aria={props.aria.clone()} placeholder={props.placeholder.clone()}/>
    }
}
