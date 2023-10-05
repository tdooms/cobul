use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul_props::{Model, Size};
use cobul_raw::components;
use cobul_raw::elements;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub value: Option<T>,

    #[prop_or_default]
    pub input: Callback<T>,

    #[prop_or_default]
    pub model: Option<Model<T>>,

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

    let handle = active.clone();
    let (input, value) = Model::combine(&props.input, &props.value, &props.model);

    let view_option = move |variant: T| {
        let active = &Some(variant) == &value;
        let (handle, input) = (handle.clone(), input.clone());

        let click = Callback::from(move |_| {
            handle.set(false);
            input.emit(variant);
        });

        html! {
            <components::DropdownItem class={classes!(props.size)} {click} {active}>
                {variant.to_string()}
            </components::DropdownItem>
        }
    };

    let focus = {
        let cloned = active.clone();
        Callback::from(move |focussed| cloned.set(focussed))
    };
    let click = {
        let cloned = active.clone();
        Callback::from(move |_| cloned.set(!*cloned))
    };

    let class = "is-flex is-justify-content-space-between";
    let trigger = html! {
        <elements::Button size={props.size} fullwidth=true {class} {click}>
            <span> {value.unwrap().to_string()} </span>
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
