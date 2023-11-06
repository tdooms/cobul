use yew::prelude::*;

use cobul_props::{general::Disabled};
use cobul_model::Model;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub model: Model<bool>,

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

    let checked = props.model.value();
    let onclick = props.model.toggle().reform(|_| ());
    let disabled = props.disabled.0 || props.model.is_constant();

    html! {
        <label style={props.style.clone()} {class}>
            <input type="checkbox" {checked} {onclick} label={&props.label} {disabled} />
            { for props.children.iter() }
        </label>
    }
}
