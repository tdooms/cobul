use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<bool>,

    #[prop_or_default]
    pub value: bool,

    #[prop_or_default]
    pub model: Option<Model<String>>,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_else(|| "Label".into())]
    pub label: Option<AttrValue>,
}

/// The 2-state checkbox in its native format - [reference](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let class = classes!("checkbox", props.class.clone());

    let checked = props.value;
    let onclick = props.input.reform(move |_| !checked);

    let (onclick, checked) = match props.model {
        Some(Model { input, value }) => (ipnut, value),
        None => (props.input, props.value),
    };

    html! {
        <label style={props.style.clone()} {class}>
            <input type="checkbox" {checked} {onclick} label={&props.label} disabled={props.disabled.0} />
            { for props.children.iter() }
        </label>
    }
}
