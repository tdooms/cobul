use yew::prelude::*;

use cobul_props::{Size, TextColor};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub click: Callback<()>,

    pub icon: AttrValue,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Bulma is compatible with all icon font libraries - [reference](https://bulma.io/documentation/elements/icon/)
///
/// - `click: Callback<()>` &npbs; Callback fro icon click
/// - `size: Option<Size>`
/// - `color: Option<Color>`
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let class = classes!("icon", props.class.clone(), props.size, props.color);
    let onclick = props.click.reform(|_| ());

    let size = match props.size {
        Some(Size::Small) => "fa-xs",
        _ => "",
    };

    let icon = format!("{} {}", props.icon, size);

    html! {
        <span style={props.style.clone()} {class} {onclick}>
            <i class={icon}> </i>
        </span>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconTextProps {
    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// You can combine an icon with text - [reference](https://bulma.io/documentation/elements/icon/#icon-text)
///
/// Properties:
/// - `color: Option<TextColor>`
#[function_component(IconText)]
pub fn icon_text(props: &IconTextProps) -> Html {
    let class = classes!("icon-text", props.class.clone(), props.color);
    html! { <span style={props.style.clone()} {class}> { for props.children.iter() } </span> }
}
