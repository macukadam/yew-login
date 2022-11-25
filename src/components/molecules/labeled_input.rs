use crate::components::atoms::input::Input;
use crate::components::atoms::label::Label;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub label_text: String,
    pub input_type: String,
    pub aria: String,
    pub placeholder: String,
    pub class: String,
}

#[function_component(LabeledInput)]
pub fn custom_form(props: &Props) -> Html {
    html! {
    <div class="form-group">
        <Label fr={props.id.clone()} text={props.label_text.clone()}/>
        <Input typ={props.input_type.clone()} class={props.class.clone()} id={props.id.clone()} aria={props.aria.clone()} placeholder={props.placeholder.clone()}/>
    </div>
    }
}
