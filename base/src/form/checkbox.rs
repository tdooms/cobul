use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub label: String,

    pub checked: bool,

    #[prop_or_default]
    pub input: Callback<bool>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let classes = classes!("checkbox", props.class.clone());

    let checked = props.checked;
    let change = props.change.reform(move |_| !checked);

    html! {
        <label style={props.style.clone()} class={classes}>
            <input
                type="checkbox"
                checked={props.checked}
                label={props.label.clone()}
                onclick={change}
                disabled={props.disabled}
                />
            { for props.children.iter() }
        </label>
    }
}
