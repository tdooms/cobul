use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub text: Option<AttrValue>,
}

/// [https://wikiki.github.io/elements/pageloader/](https://wikiki.github.io/elements/pageloader/)
#[function_component(Loader)]
pub fn loader(props: &Props) -> Html {
    let direction = if props.vertical {
        "is-divider-vertical"
    } else {
        "is-divider"
    };
    let class = classes!(direction, props.class.clone());
    html! { <div style={props.style.clone()} data-content={props.text.clone()} {class}></div> }
}
