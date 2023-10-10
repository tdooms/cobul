use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub label: Option<AttrValue>,

    #[prop_or_default]
    pub checked: Option<T>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// The mutually exclusive radio buttons in their native format - [reference](https://bulma.io/documentation/form/radio/)
#[function_component(Radio)]
pub fn radio<T>(props: &Props<T>) -> Html
    where
        T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let view_option = |variant: T| {
        let checked = Some(variant) == props.checked;
        html! { <label class="radio"> <input style={props.style.clone()} class={props.class.clone()} type="radio" name={props.label.clone()} {checked} /> {variant} </label> }
    };

    html! { { for T::iter().map(view_option) } }
}
