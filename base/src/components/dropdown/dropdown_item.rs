use crate::props::Active;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub active: Active,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(DropdownItem)]
pub fn dropdown_item(props: &Props) -> Html {
    let Props {
        class,
        children,
        onclick,
        active,
    } = &props;
    let classes = classes!("dropdown-item", *active, class.clone());

    html! {
        <a class={classes} onclick={onclick.reform(|_|())}>
            { for children.iter() }
        </a>
    }
}
