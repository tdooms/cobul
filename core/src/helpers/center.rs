use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct HCenterProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(HCenter)]
pub fn hcenter(props: &HCenterProps) -> Html {
    html! { <div class="is-flex is-justify-content-center"> {props.children.clone()} </div> }
}
