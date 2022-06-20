use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub name: String,

    pub checked: bool,

    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let classes = classes!("checkbox", props.class.clone());

    let copied = !props.checked;
    let onchange = props.onchange.reform(move |_| copied);

    html! {
        <label style={props.style.clone()} class={classes}>
            <input
                type="checkbox"
                checked={props.checked}
                name={props.name.clone()}
                onclick={onchange}
                disabled={props.disabled}
                />
            { for props.children.iter() }
        </label>
    }
}
