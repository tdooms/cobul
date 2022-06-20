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

/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(LevelItem)]
pub fn level_item(props: &Props) -> Html {
    let classes = classes!("level-item", props.class.clone());
    html! {
        <div style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </div>
    }
}
