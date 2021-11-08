use yew::prelude::*;

use crate::props::SectionSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub size: Option<SectionSize>,
}


/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[function_component(Section)]
pub fn section(props: &Props) -> Html {
    let classes = classes!("section", &props.extra, props.size);

    html! {
        <section class={classes}>
            { for props.children.iter() }
        </section>
    }
}
