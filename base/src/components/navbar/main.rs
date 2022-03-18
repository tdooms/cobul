use yew::prelude::*;

use crate::props::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub brand: Option<Html>,

    #[prop_or_default]
    pub start: Option<Html>,

    #[prop_or_default]
    pub end: Option<Html>,

    #[prop_or_default]
    pub active: Active,

    #[prop_or(true)]
    pub burger: bool,

    pub onclick: Callback<()>,
}

/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let classes = classes!("navbar-burger", props.active);
    let onclick = props.onclick.reform(|_| ());

    let burger = match props.burger {
        true => html! {
            <a role="button" class={classes} aria-label="menu" aria-expanded="false" data-target="navbar" onclick={onclick}>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </a>
        },
        false => html! {},
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                { props.brand.clone().unwrap_or_default() }
                { burger }
            </div>

            <div id="navbar" class="navbar-menu">
                <div class="navbar-start">
                    { props.start.clone().unwrap_or_default() }
                </div>

                <div class="navbar-end">
                    { props.end.clone().unwrap_or_default() }
                </div>
            </div>
        </nav>
    }
}
