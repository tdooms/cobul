use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Bulma’s most basic spacer block - [reference](https://bulma.io/documentation/elements/block/)
///
/// Properties:
#[function_component(Block)]
pub fn block(props: &Props) -> Html {
    let class = classes!("block", props.class.clone());
    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
