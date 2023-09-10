use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(NavbarDivider)]
pub fn navbar_divider(props: &Props) -> Html {
    html! {
        <div class={classes!("navbar-divider", props.class.clone())} style={props.style.clone()}>
            { for props.children.iter() }
        </div>
    }
}
