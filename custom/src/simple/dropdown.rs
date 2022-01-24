use yew::prelude::*;

use base::props::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    pub text: String,

    #[prop_or_default]
    pub help: Option<String>,

    #[prop_or_default]
    pub help_color: Option<Color>,

    #[prop_or_default]
    pub icon_right: Option<String>,

    #[prop_or_default]
    pub icon_left: Option<String>,
}

#[function_component(SimpleDropdown)]
pub fn simple_dropdown(_: &Props) -> Html {
    html! {}
}
