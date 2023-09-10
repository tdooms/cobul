use yew::prelude::*;

use cobul_props::{Color};
use cobul_props::general::Light;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub delete: Callback<()>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Bold notification blocks, to alert your users of something - [reference](https://bulma.io/documentation/elements/notification/)
///
/// Properties:
/// - `delete: Callback<()>` Callback for when the delete button is clicked
/// - `color: Option<Color>`
/// - `light: Light`
#[function_component(Notification)]
pub fn notification(props: &Props) -> Html {
    let class = classes!(
        "notification",
        props.class.clone(),
        props.color,
        props.light
    );

    let button = match props.delete == Callback::noop() {
        false => html! {<button class="delete" onclick={props.delete.reform(|_| ())}></button>},
        true => html! {},
    };

    html! {
        <div style={props.style.clone()} {class}>
            { button }
            { for props.children.iter() }
        </div>
    }
}
