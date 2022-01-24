use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
#[function_component(Box)]
pub fn r#box(props: &Props) -> Html {
    let classes = classes!("box", props.class.clone());
    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
