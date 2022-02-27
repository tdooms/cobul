use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::props::{Color, Focused, Hovered, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    pub selected: Vec<T>,

    #[prop_or_default]
    pub onselect: Callback<Vec<T>>,
}

/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
#[function_component(MultipleSelect)]
pub fn multiple_select<T>(props: &Props<T>) -> Html
where
    T: IntoEnumIterator + ToString + Copy + PartialEq + 'static,
{
    let classes = classes!(
        "select",
        "is-multiple",
        props.class.clone(),
        props.color,
        props.size
    );

    let view_option = |variant: T| {
        let position = props
            .selected
            .iter()
            .position(|x| std::mem::discriminant(x) == std::mem::discriminant(&variant));

        // TODO: this is N^2 for a simple select, use Cow?
        let items = props.selected.clone();

        let (onclick, selected) = match position {
            Some(index) => (
                props.onselect.reform(move |_| {
                    let mut cloned = items.clone();
                    cloned.remove(index);
                    cloned
                }),
                true,
            ),
            None => (
                props.onselect.reform(move |_| {
                    let mut cloned = items.clone();
                    cloned.push(variant);
                    cloned
                }),
                false,
            ),
        };

        html! { <option selected={selected} onclick={onclick}> {variant} </option> }
    };

    html! {
        <div class={classes}>
            <select multiple={true} size={T::iter().count().to_string()} class={classes!(props.hovered, props.focussed)}>
                { for T::iter().map(view_option) }
            </select>
        </div>
    }
}
