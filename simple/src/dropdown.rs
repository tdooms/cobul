use strum::IntoEnumIterator;
use yew::prelude::*;
use ywt::callback;

use base::components;
use base::elements;
use base::props::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    pub value: T,

    pub input: Callback<T>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Dropdown)]
pub fn dropdown<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let active = use_state_eq(|| false);

    let input = props.input.clone();
    let handle = active.clone();

    let view_option = move |variant: T| {
        let active = &variant == &props.value;

        let click = callback!(handle, input; move |_| {
            handle.set(false);
            input.emit(variant);
            log::info!("here");
        });

        html! {
            <components::DropdownItem class={classes!(props.size)} {click} {active}>
                {variant.to_string()}
            </components::DropdownItem>
        }
    };

    let focus = callback!(active; move |focussed| active.set(focussed));
    let click = callback!(active; move |_| active.set(!*active));

    let class = "is-flex is-justify-content-space-between";
    let trigger = html! {
        <elements::Button size={props.size} fullwidth=true {class} {click}>
            <span> {props.value.to_string()} </span>
            <elements::Icon icon="fa-solid fa-angle-down"/>
        </elements::Button>
    };

    let style = props.style.clone();
    let class = classes!(props.class.clone(), props.size);
    let fullwidth = props.fullwidth;

    html! {
        <components::Dropdown {style} {trigger} active={*active} {class} {focus} {fullwidth}>
            { for T::iter().map(view_option) }
        </components::Dropdown>
    }
}
