use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
#[function_component(Block)]
pub fn block(props: &Props) -> Html {
    let classes = classes!("block", props.class.clone());
    html! {
        <div style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </div>
    }
}
