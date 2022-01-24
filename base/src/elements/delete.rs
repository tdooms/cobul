use crate::props::Size;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_else(|| "button".into())]
    pub tag: String,

    pub onclick: Callback<()>,
}

/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
#[function_component(Delete)]
pub fn delete(props: &Props) -> Html {
    let classes = classes!("delete", props.class.clone(), props.size);
    let onclick = props.onclick.reform(|_| ());

    html! {
        <@{props.tag.clone()} class={classes} onclick={onclick}>
            { for props.children.iter() }
        </@>
    }
}
