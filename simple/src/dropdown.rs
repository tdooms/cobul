use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::Size;
use cobul_model::{Model, use_model_eq};
use cobul_core as core;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ItemProps<T: ToString + Copy + PartialEq + 'static> {
    pub variant: T,
    pub model: Model<T>,
    pub active: Model<bool>,
    pub size: Option<Size>,
}

#[function_component(Item)]
fn item<T: ToString + Copy + PartialEq + 'static>(props: &ItemProps<T>) -> Html {
    let ItemProps { variant, model, active, size } = props.clone();

    let click = Callback::from(move |_| {
        active.emit(false);
        model.emit(variant);
    });

    let active = &props.variant == &*props.model;

    html! {
        <core::DropdownItem class={classes!(size)} {click} {active}>
            {variant.to_string()}
        </core::DropdownItem>
    }
}

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
    let active = use_model_eq(|| false);
    let onclick = active.toggle().reform(|_| ());

    let class = classes!("button", "is-flex", "is-justify-content-space-between", "is-fullwidth", props.size);
    let trigger = html! {
        <button {class} {onclick}>
            <span> {props.model.value().to_string()} </span>
            <core::Icon icon="fa-solid fa-angle-down"/>
        </button>
    };

    let style = props.style.clone();
    let class = classes!(props.class.clone(), props.size);
    let fullwidth = props.fullwidth;

    html! {
        <core::Dropdown {style} {trigger} {class} model={active.clone()} {fullwidth}>
            { for T::iter().map(|variant| html! {<Item<T> {variant} model={props.model.clone()} active={active.clone()} size={props.size} />}) }
        </core::Dropdown>
    }
}
