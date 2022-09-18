use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub label: String,

    pub value: bool,

    #[prop_or_default]
    pub input: Callback<bool>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let class = classes!("checkbox", props.class.clone());

    let checked = props.value;
    let input = props.input.reform(move |_| !checked);

    html! {
        <label style={props.style.clone()} {class}>
            <input
                type="checkbox"
                checked={props.value}
                label={props.label.clone()}
                onclick={input}
                disabled={props.disabled}
                />
            { for props.children.iter() }
        </label>
    }
}
