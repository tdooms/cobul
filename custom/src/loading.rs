use yew::prelude::*;

use base::props::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub color: Option<Color>,
}

#[function_component(Loading)]
pub fn loading(props: &Props) -> Html {
    let classes = classes!("progress", props.color);

    html! {
        <section class="section is-fullheight p-0 m-0">
            <div class="columns is-centered is-desktop is-vcentered" style="height:100vh">
                <div class="column is-half">
                    <progress class={classes}></progress>
                </div>
            </div>
        </section>
    }
}
