use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A single class to handle WYSIWYG generated content, where only HTML tags are available - [reference](https://bulma.io/documentation/elements/content/)
#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    let class = classes!("content", props.class.clone());
    html! {
        <@{props.tag.to_string()} style={props.style.clone()} {class}>
            { for props.children.iter() }
        </@>
    }
}
