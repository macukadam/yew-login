use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::main_title::MainTitle;
use crate::components::molecules::labeled_input::LabeledInput;
use yew::prelude::*;

#[function_component(UserLoginForm)]
pub fn custom_form() -> Html {
    html! {
        <form>
            <MainTitle title={"Login form"}/>
            <LabeledInput class="form-control" id="email_input" label_text="Email address" input_type="email" aria="emailHelp" placeholder="Enter your email"  />
            <LabeledInput class="form-control" id="password" label_text="Password" input_type="password" aria="passwordHelp" placeholder="Enter your password"  />

            <div class="form-group">
                <CustomButton text="Submit"/>
            </div>
        </form>
    }
}
