use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::props::{Alignment, Boxed, Color, Fullwidth, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<Vec<web_sys::File>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub filename: Option<AttrValue>,

    #[prop_or_default]
    pub accept: Option<AttrValue>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub boxed: Boxed,

    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// [https://bulma.io/documentation/form/file/](https://bulma.io/documentation/form/file/)
#[function_component(File)]
pub fn file(props: &Props) -> Html {
    let boxed = use_context::<Boxed>();
    let fullwidth = use_context::<Fullwidth>();
    let alignment = use_context::<Alignment>();

    let class = classes!(
        "file",
        props.filename.as_ref().map(|_| "has-name"),
        props.boxed.or(boxed),
        props.fullwidth.or(fullwidth),
        props.alignment.or(alignment),
        props.class.clone()
    );

    let onchange = props.input.reform(|e: Event| {
        let target = e.target_unchecked_into::<HtmlInputElement>();
        let files = target.files().unwrap();
        (0..files.length()).filter_map(|i| files.get(i)).collect()
    });

    let filename = match &props.filename {
        None => html! {},
        Some(file) => html! {<span class="file-name"> {file} </span>},
    };

    html! {
        <div style={props.style.clone()} {class}>
            <label class="file-label">
            <input class="file-input" type="file" accept={props.accept.clone()} {onchange} />
            <span class="file-cta">
                <span class="file-icon">
                <i class="fas fa-upload"></i>
                </span>
                <span class="file-label">
                    {"Choose a file..."}
                </span>
            </span>
                { filename }
            </label>
        </div>
    }
}
