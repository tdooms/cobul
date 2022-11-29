use yew::prelude::*;

use crate::props::SectionSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<SectionSize>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// A simple container to divide your page into sections, like the one you’re currently reading [reference](https://bulma.io/documentation/layout/section/)
///
/// Properties:
/// - `size: Option<SectionSize>`
#[function_component(Section)]
pub fn section(props: &Props) -> Html {
    let class = classes!("section", props.class.clone(), props.size);

    html! {
        <section style={props.style.clone()} {class}>
            { for props.children.iter() }
        </section>
    }
}
