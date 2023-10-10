use web_sys::HtmlInputElement;
use yew::prelude::*;

use cobul_props::general::{Boxed, Fullwidth};
use cobul_props::{Align, Color, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<Vec<web_sys::File>>,

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
    pub align: Option<Align>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A custom file upload input, without JavaScript - [reference](https://bulma.io/documentation/form/file/)
///
/// Properties:
/// - `input: Callback<Vec<web_sys::File>>` Callback for when file or files are selected
/// - `fullwidth: Fullwidth`
/// - `filename: Option<AttrValue>`
/// - `accept: Option<AttrValue>`
/// - `size: Option<Size>`
/// - `color: Option<Color>`
/// - `boxed: Boxed`
#[function_component(File)]
pub fn file(props: &Props) -> Html {
    let boxed = use_context::<Boxed>();
    let fullwidth = use_context::<Fullwidth>();
    let align = use_context::<Align>();

    let class = classes!(
        "file",
        props.filename.as_ref().map(|_| "has-name"),
        props.boxed.or(boxed),
        props.fullwidth.or(fullwidth),
        props.align.or(align),
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
