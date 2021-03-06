use web_sys;
use yew::prelude::*;

use crate::props::{Alignment, Boxed, Color, Fullwidth, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    #[prop_or_default]
    pub boxed: Boxed,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub class: Classes,

    pub onupload: Callback<Vec<web_sys::File>>,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/form/file/](https://bulma.io/documentation/form/file/)
#[function_component(File)]
pub fn file(props: &Props) -> Html {
    let maybe_file = || match &props.filename {
        None => html! {},
        Some(file) => html! {<span class="file-name"> {file} </span>},
    };

    let classes = classes!(
        "file",
        props.filename.as_ref().map(|_| "has-name"),
        props.boxed,
        props.fullwidth,
        props.alignment,
        props.class.clone()
    );

    let onchange = props.onupload.reform(|e: Event| {
        let files = e
            .target_unchecked_into::<web_sys::HtmlInputElement>()
            .files()
            .unwrap();
        (0..files.length()).filter_map(|i| files.get(i)).collect()
    });

    html! {
        <div style={props.style.clone()} class={classes}>
            <label class="file-label">
            <input class="file-input" type="file" accept={props.accept.clone()} onchange={onchange} />
            <span class="file-cta">
                <span class="file-icon">
                <i class="fas fa-upload"></i>
                </span>
                <span class="file-label">
                    {"Choose a file..."}
                </span>
            </span>
                { maybe_file() }
            </label>
        </div>
    }
}
