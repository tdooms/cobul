use yew::prelude::*;

use cobul_base::props::Color;

#[derive(Clone, Copy, Debug, PartialEq, Default, derive_more::Display)]
pub enum Direction {
    #[default]
    #[display(fmt = "is-top-to-bottom")]
    TopToBottom,
    #[display(fmt = "is-bottom-to-top")]
    BottomToTop,
    #[display(fmt = "is-left-to-right")]
    LeftToRight,
    #[display(fmt = "is-right-to-left")]
    RightToLeft,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub direction: Direction,

    #[prop_or_default]
    pub color: Option<Color>,
}

/// Display a page-loader to inform users that content is loading, in different colors - [reference](https://wikiki.github.io/elements/pageloader/)
#[function_component(Loader)]
pub fn loader(props: &Props) -> Html {
    let class = classes!(
        "pageloader",
        "is-active",
        props.class.clone(),
        props.direction.to_string(),
        props.color
    );
    html! { <div style={props.style.clone()} {class}></div> }
}
