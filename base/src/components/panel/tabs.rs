use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    pub value: T,

    pub onclick: Callback<T>,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelTabs)]
pub fn panel_tabs<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let button_map = |variant: T| {
        let active = props.value == variant;
        let onclick = props.onclick.reform(move |_| variant);

        html! {
            <a onclick={onclick} class={active.then(|| "is-active")}>
                {variant.to_string()}
            </a>
        }
    };

    html! {
        <div style={props.style.clone()} class={classes!("panel-tabs", props.class.clone())}>
            { for T::iter().map(button_map) }
        </div>
    }
}
