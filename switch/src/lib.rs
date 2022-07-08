use yew::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    class: Classes,
}

#[function_component(Switch)]
pub fn switch(props: &Props) -> Html {
    // let classes = classes!("switch", props.class.clone());

    html! {
        "a beautiful switch"
    }
}
