use strum::IntoEnumIterator;
use yew::prelude::*;

use base::components;
use base::elements;
use base::props::Size;
use ywt::callback;

// TODO: deselect when not in focus
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    pub value: T,

    pub onchange: Callback<T>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub style: Option<String>,
}

#[function_component(Dropdown)]
pub fn dropdown<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let trigger = html! {
        <elements::Button size={props.size} fullwidth=true class="is-flex is-justify-content-space-between">
            <span> {props.value.to_string()} </span>
            <elements::Icon icon={fa::Solid::AngleDown}/>
        </elements::Button>
    };

    let view_option = |variant: T| {
        let active = &variant == &props.value;
        let onclick = props.onchange.reform(move |_| variant);

        html! {
            <components::DropdownItem class={classes!(props.size)} {onclick} {active}>
                {variant.to_string()}
            </components::DropdownItem>
        }
    };

    let active = use_state(|| false);
    let onfocus = callback!(active; move |focussed| active.set(focussed));

    let style = props.style.clone();
    let class = classes!(props.class.clone(), props.size);
    let fullwidth = props.fullwidth;

    html! {
        <components::Dropdown {style} {trigger} active={*active} {class} {onfocus} {fullwidth}>
            { for T::iter().map(view_option) }
        </components::Dropdown>
    }
}
