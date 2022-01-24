use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let classes = classes!("footer", props.class.clone());
    html! {
        <footer class={classes}>
            { for props.children.iter() }
        </footer>
    }
}
