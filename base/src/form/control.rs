use yew::prelude::*;

use crate::props::Expanded;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: Expanded,
    /// String for right icon type
    #[prop_or_default]
    pub right: Option<String>,
    /// String for left icon type
    #[prop_or_default]
    pub left: Option<String>,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Control)]
pub fn control(props: &Props) -> Html {
    let classes = classes!(
        "control",
        &props.extra,
        props.expanded,
        props.right.as_ref().map(|_| "has-icons-right"),
        props.left.as_ref().map(|_| "has-icons-left")
    );

    let map_icon = |icon: Option<&String>, right: bool| {
        let alignment = if right { "is-right" } else { "is-left" };
        let classes = classes!("icon", "is-small", alignment);

        match icon {
            None => html! {},
            Some(name) => html! {<span class={classes}><i class={name}> </i></span>},
        }
    };

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
            { map_icon(props.right.as_ref(), true) }
            { map_icon(props.left.as_ref(), false) }
        </@>
    }
}
