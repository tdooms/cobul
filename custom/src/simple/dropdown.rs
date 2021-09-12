use yew::prelude::*;

use base::elements::Icon;
use base::form::{Help, Label};
use base::props::Color;

pub enum DropdownElement {
    Item(String),
    Divider
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

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

#[function_component(SimpleField)]
pub fn simple_dropdown(props: &Props) -> Html {
}
