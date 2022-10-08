use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(CardHeaderTitle)]
pub fn card(props: &Props) -> Html {
    html! {
        <nav style={props.style.clone()} class={classes!("card-header-title", props.class.clone())} >
            { for props.children.iter() }
        </nav>
    }
}
