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

#[function_component(CardFooterItem)]
pub fn card(props: &Props) -> Html {
    html! {
        <nav style={props.style.clone()} class={classes!("card-footer-item", props.class.clone())} >
            { for props.children.iter() }
        </nav>
    }
}
