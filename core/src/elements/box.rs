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

/// A white box to contain other elements - [reference](https://bulma.io/documentation/elements/box/)
///
/// Properties:
#[function_component(Box)]
pub fn r#box(props: &Props) -> Html {
    let class = classes!("box", props.class.clone());
    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
