use cobul_props::general::Expanded;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,

    #[prop_or_default]
    pub expanded: Expanded,

    #[prop_or_default]
    pub right: Option<AttrValue>,

    #[prop_or_default]
    pub left: Option<AttrValue>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

/// All generic form controls, designed for consistency - [reference](https://bulma.io/documentation/form/general/)
///
/// Properties:
/// - `tag: AttrValue` the tag of the element - default: div
/// - `expanded: Expanded`
/// - `right: Option<AttrValue>` the icon to the right of the control
/// - `left: Option<AttrValue>` the icon to the left of the control
#[function_component(Control)]
pub fn control(props: &Props) -> Html {
    let class = classes!(
        "control",
        props.class.clone(),
        props.expanded,
        props.right.as_ref().map(|_| "has-icons-right"),
        props.left.as_ref().map(|_| "has-icons-left")
    );

    let view_icon = |icon: Option<&AttrValue>, right: bool| {
        let align = if right { "is-right" } else { "is-left" };
        let class = classes!("icon", "is-small", align);

        match icon {
            None => html! {},
            Some(name) => html! {<span {class}><i class={name.to_string()}> </i></span>},
        }
    };

    html! {
        <@{ props.tag.to_string() } style={props.style.clone()} {class}>
            { for props.children.iter() }
            { view_icon(props.right.as_ref(), true) }
            { view_icon(props.left.as_ref(), false) }
        </@>
    }
}
