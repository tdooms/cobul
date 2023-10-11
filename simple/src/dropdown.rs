use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Model, Size};
use cobul_core as core;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    pub model: Model<T>,

    #[prop_or_default]
    pub class: Classes,

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
    let Model { value, input } = props.model.clone();

    let handle = active.clone();
    let view_option = move |variant: T| {
        let active = &variant == &value;
        let (handle, input) = (handle.clone(), input.clone());

        let click = Callback::from(move |_| {
            handle.set(false);
            input.emit(variant);
        });

        html! {
            <core::DropdownItem class={classes!(props.size)} {click} {active}>
                {variant.to_string()}
            </core::DropdownItem>
        }
    };

    let focus = {
        let cloned = active.clone();
        Callback::from(move |focussed| cloned.set(focussed))
    };
    let onclick = {
        let cloned = active.clone();
        Callback::from(move |_| cloned.set(!*cloned))
    };

    let class = classes!("button", "is-flex", "is-justify-content-space-between", "is-fullwidth", props.size);
    let trigger = html! {
        <button {class} {onclick}>
            <span> {value.to_string()} </span>
            <core::Icon icon="fa-solid fa-angle-down"/>
        </button>
    };

    let style = props.style.clone();
    let class = classes!(props.class.clone(), props.size);
    let fullwidth = props.fullwidth;

    html! {
        <core::Dropdown {style} {trigger} active={*active} {class} {focus} {fullwidth}>
            { for T::iter().map(view_option) }
        </core::Dropdown>
    }
}
