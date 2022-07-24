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
    pub style: Option<String>,
}

#[function_component(Dropdown)]
pub fn dropdown<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let Props {
        class,
        value,
        onchange,
        size,
        ..
    } = &props;

    let trigger = html! {
        <elements::Button size={size.clone()}>
            <span> {value.to_string()} </span>
            <elements::Icon icon={fa::Solid::AngleDown}/>
        </elements::Button>
    };

    let view_option = |variant: T| {
        let active = &variant == value;
        let onclick = onchange.reform(move |_| variant);

        html! {
            <components::DropdownItem class={classes!(*size)} {onclick} {active}>
                {variant.to_string()}
            </components::DropdownItem>
        }
    };

    let active = use_state(|| false);
    let onfocus = callback!(active; move |focussed| active.set(focussed));

    let style = props.style.clone();
    html! {
        <components::Dropdown {style} {trigger} active={*active} class={class.clone()} {onfocus}>
            { for T::iter().map(view_option) }
        </components::Dropdown>
    }
}
