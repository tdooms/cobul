use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<()>,
}

/// [// https://bulma.io/documentation/components/dropdown/](// https://bulma.io/documentation/components/dropdown/)
#[function_component(DropdownItem)]
pub fn dropdown_item(props: &Props) -> Html {
    let classes = classes!("dropdown-item", props.class.clone());

    html! {
        <a class={classes} onclick={props.onclick.reform(|_|())}>
            { for props.children.iter() }
        </a>
    }
}
