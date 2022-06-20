use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Label)]
pub fn label(props: &Props) -> Html {
    let classes = classes!("label", props.class.clone());

    html! {
        <div style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </div>
    }
}
