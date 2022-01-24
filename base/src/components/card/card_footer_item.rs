use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// [// https://bulma.io/documentation/components/card/](// https://bulma.io/documentation/components/card/)
#[function_component(CardFooterItem)]
pub fn card(props: &Props) -> Html {
    html! {
        <nav class={classes!("card-footer-item", props.class.clone())} >
            { for props.children.iter() }
        </nav>
    }
}
