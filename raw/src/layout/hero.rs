use yew::prelude::*;

use cobul_props::{Color, HeroSize};
use crate::util::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<HeroSize>,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

/// An imposing hero banner to showcase something - [reference](https://bulma.io/documentation/layout/hero/)
///
/// Properties:
/// - `color: Option<Color>`
/// - `size: Option<HeroSize>`
/// - `header: Option<Html>`
/// - `footer: Option<Html>`
#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    let class = classes!("hero", props.size, props.color, props.class.clone());

    html! {
        <section style={props.style.clone()} {class}>
            { enclose("hero-header", props.header.clone()) }
            <div class="hero-body"> { for props.children.iter() } </div>
            { enclose("hero-footer", props.footer.clone()) }
        </section>
    }
}
