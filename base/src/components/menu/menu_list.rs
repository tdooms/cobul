use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuList)]
pub fn menu_list(props: &Props) -> Html {
    let classes = classes!("menu-list", props.class.clone());
    html! {
        <p class={classes}>
            { for props.children.iter() }
        </p>
    }
}
