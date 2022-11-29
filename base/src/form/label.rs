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

/// All generic form controls, designed for consistency - [reference](https://bulma.io/documentation/form/general/)
///
/// Properties:
#[function_component(Label)]
pub fn label(props: &Props) -> Html {
    let class = classes!("label", props.class.clone());

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
