use yew::prelude::*;

use crate::props::SectionSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Option<SectionSize>,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[function_component(Section)]
pub fn section(props: &Props) -> Html {
    let classes = classes!("section", props.class.clone(), props.size);

    html! {
        <section style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </section>
    }
}
