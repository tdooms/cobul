use yew::prelude::*;
use cobul_props::{Model, general::Disabled};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub model: Option<Model<bool>>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub label: Option<AttrValue>,
}

/// The 2-state checkbox in its native format - [reference](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let class = classes!("checkbox", props.class.clone());
    let (value, input) = Model::split(&props.model);

    let checked = value.unwrap_or_default();
    let onclick = input.reform(move |_| !checked);

    html! {
        <label style={props.style.clone()} {class}>
            <input type="checkbox" {checked} {onclick} label={&props.label} disabled={props.disabled.0} />
            { for props.children.iter() }
        </label>
    }
}
