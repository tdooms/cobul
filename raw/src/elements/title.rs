use yew::prelude::*;

use cobul_props::{HeaderSize, Spaced};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub spaced: Spaced,

    #[prop_or_default]
    pub size: Option<HeaderSize>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// Simple headings to add depth to your page - [reference](https://bulma.io/documentation/elements/title/)
///
/// Properties:
/// - `spaced: Spaced`
/// - `size: Option<HeaderSize>`
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is3);
    let class = classes!("title", props.class.clone(), size, props.spaced);

    html! {
        <p style={props.style.clone()} {class}>
            { for props.children.iter() }
        </p>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub size: Option<HeaderSize>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// Simple headings to add depth to your page - [reference](https://bulma.io/documentation/elements/title/)
///
/// Properties:
/// - `size: Option<HeaderSize>`
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is5);
    let class = classes!("subtitle", props.class.clone(), size);

    html! {
        <p style={props.style.clone()} {class}>
            { for props.children.iter() }
        </p>
    }
}
