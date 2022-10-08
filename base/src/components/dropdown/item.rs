use yew::prelude::*;

use crate::props::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(DropdownItem)]
pub fn dropdown_item(props: &Props) -> Html {
    let Props {
        class,
        children,
        click,
        active,
        style,
    } = &props;
    let class = classes!("dropdown-item", *active, class.clone());

    html! {
        <a style={style.clone()} {class} onclick={click.reform(|_|())}>
            { for children.iter() }
        </a>
    }
}
