use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(|| "div".into())]
    pub tag: String,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/content/](https://bulma.io/documentation/elements/content/)
#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    let classes = classes!("content", props.class.clone());
    html! {
        <@{props.tag.clone()} style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </@>
    }
}
