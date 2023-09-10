use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    pub value: T,

    #[prop_or_default]
    pub click: Callback<T>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A simple component for rendering a navbar link.
///
/// Properties:
/// - `value: T` &npbs; The value of the tab.
/// - `click: Callback<T>` &npbs; A callback that is invoked when the tab is clicked.
#[function_component(PanelTabs)]
pub fn panel_tabs<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let button_map = |variant: T| {
        let active = props.value == variant;
        let onclick = props.click.reform(move |_| variant);

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
