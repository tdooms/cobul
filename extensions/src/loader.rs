use base::props::Color;
use yew::prelude::*;

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
    pub style: Option<String>,

    #[prop_or_default]
    pub direction: Direction,

    #[prop_or_default]
    pub color: Option<Color>,
}

/// [https://wikiki.github.io/elements/pageloader/](https://wikiki.github.io/elements/pageloader/)
#[function_component(Loader)]
pub fn loader(props: &Props) -> Html {
    let classes = classes!(
        "pageloader",
        "is-active",
        props.class.clone(),
        props.direction.to_string(),
        props.color
    );
    html! { <div style={props.style.clone()} class={classes}></div> }
}
