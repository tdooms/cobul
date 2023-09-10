use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use validator::Validate;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: Validate + PartialEq + Debug> {
    pub data: Rc<T>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub submit: Callback<()>,
}

#[derive(Clone, PartialEq)]
pub struct FieldData {
    error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct FormData {
    inner: HashMap<String, FieldData>
}

#[function_component(Form)]
pub fn form<T: Validate + PartialEq + Debug>(_props: &Props<T>) -> Html {
    html! {}
}