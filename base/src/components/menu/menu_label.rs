use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
}

/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuLabel)]
pub fn menu_label(props: &Props) -> Html {
    let classes = classes!("menu-label", &props.extra);
    html! {
        <p class={classes}>
            { for props.children.iter() }
        </p>
    }
}