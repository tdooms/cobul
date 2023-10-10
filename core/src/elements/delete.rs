use yew::prelude::*;

use cobul_props::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_else(|| "button".into())]
    pub tag: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A versatile delete cross - [reference](https://bulma.io/documentation/elements/delete/)
///
/// Properties:
/// - `click: Callback<()>` &npbs; default: None
/// - `size: Option<Size>`
/// - `tag: AttrValue` the tag of the element - default: button
#[function_component(Delete)]
pub fn delete(props: &Props) -> Html {
    let class = classes!("delete", props.class.clone(), props.size);
    let onclick = props.click.reform(|_| ());

    html! {
        <@{props.tag.to_string()} style={props.style.clone()} {class} {onclick}>
            { for props.children.iter() }
        </@>
    }
}
