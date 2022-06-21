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

/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuLabel)]
pub fn menu_label(props: &Props) -> Html {
    let classes = classes!("menu-label", props.class.clone());
    html! {
        <p style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </p>
    }
}
