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

#[function_component(MenuList)]
pub fn menu_list(props: &Props) -> Html {
    let class = classes!("menu-list", props.class.clone());
    html! {
        <p style={props.style.clone()} {class}>
            { for props.children.iter() }
        </p>
    }
}
