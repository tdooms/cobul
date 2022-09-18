use yew::prelude::*;

use crate::props::{Size, TextColor};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub icon: String,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let class = classes!("icon", props.class.clone(), props.size, props.color);
    let onclick = props.click.reform(|_| ());

    let size = match props.size {
        Some(Size::Small) => "fa-xs",
        Some(Size::Large) => "fa-2xl",
        _ => "",
    };

    let icon = format!("{} {}", props.icon, size);

    html! {
        <span style={props.style.clone()} {class} {onclick}>
            <i class={icon}> </i>
        </span>
    }
}
