use yew::prelude::*;

use crate::props::{Light, Rounded, Size};

/// The turn into delete button is intentionally skipped,
/// use the delete element for that functionality
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_else(|| "span".into())]
    pub tag: String,

    #[prop_or_default]
    pub onclick: Option<Callback<()>>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let classes = classes!(
        "tag",
        props.class.clone(),
        props.size,
        props.rounded,
        props.light,
    );

    html! {
        <@{ props.tag.clone() } style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </@>
    }
}
