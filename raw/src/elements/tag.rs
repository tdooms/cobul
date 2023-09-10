use yew::prelude::*;

use cobul_props::{Addons, Color, Delete, Light, Rounded, Size};

// The turn into delete button is intentionally skipped,
// use the delete element for that functionality

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub delete: Delete,

    #[prop_or_else(|| "span".into())]
    pub tag: AttrValue,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Small tag labels to insert anywhere - [reference](https://bulma.io/documentation/elements/tag/)
///
/// Properties:
/// - `click: Callback<()>` &npbs; Callback for when the tag is clicked
/// - `light: Light`
/// - `color: Option<Color>`
/// - `delete: Delete`
/// - `tag: AttrValue` the tag of the element - default: span
/// - `rounded: Rounded`
/// - `size: Option<Size>`
#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let class = classes!(
        "tag",
        props.class.clone(),
        props.size,
        props.rounded,
        props.light,
        props.color,
        props.delete
    );

    let onclick = props.click.reform(|_| ());
    html! {
        <@{ props.tag.to_string() } style={props.style.clone()} {class} {onclick}>
            { for props.children.iter() }
        </@>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagsProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A list of tags - [reference](https://bulma.io/documentation/elements/tag/#list-of-tags)
#[function_component(Tags)]
pub fn tags(props: &TagsProps) -> Html {
    let class = classes!("tags", props.class.clone(), props.addons);

    html! {
        <div style={props.style.clone()} {class}>
            { for props.children.iter() }
        </div>
    }
}
