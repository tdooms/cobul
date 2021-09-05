use yew::prelude::*;

// https://bulma.io/documentation/components/card/
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    pub extra: String,
}

#[function_component(CardHeaderTitle)]
pub fn card(props: &Props) -> Html {
    html! {
        <nav class={classes!("card-header-title", &props.extra)} >
            { for props.children.iter() }
        </nav>
    }
}
