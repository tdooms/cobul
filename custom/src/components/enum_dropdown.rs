use strum::IntoEnumIterator;
use yew::prelude::*;

use base::components::{Dropdown, DropdownItem};
use base::elements::{Button, Icon};
use base::props::Color;

// TODO: deselect when not in focus
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    pub value: T,

    pub onchange: Callback<T>,
}

#[function_component(EnumDropdown)]
pub fn enum_dropdown<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let active = use_state(|| false);

    let Props {
        class,
        value,
        onchange,
    } = &props;

    let trigger = {
        let active = active.clone();
        let onclick = Callback::from(move |_| active.set(!(*active)));

        html! {
        <Button {onclick}>
            <span> {value.to_string()} </span>
            <Icon icon="fas fa-angle-down"/>
        </Button>}
    };

    let dropdown_map = |variant: T| {
        let active = &variant == value;
        let onclick = onchange.reform(move |_| variant);

        html! {<DropdownItem {onclick} {active}> {variant.to_string()} </DropdownItem>}
    };

    html! {
        <Dropdown {trigger} active={*active}>
            { for T::iter().map(dropdown_map) }
        </Dropdown>
    }
}
