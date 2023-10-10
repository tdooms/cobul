use yew::*;

use cobul::{Box, Button, Color, extra::Direction, extra::Loader};

#[function_component(LoaderTest)]
pub fn loader() -> Html {
    let state = use_state(|| None);
    let timer = use_state(|| None);

    let click = |dir: Direction| {
        let state = state.clone();
        let timer = timer.clone();

        Callback::from(move |_| {
            state.set(Some(dir));
            let cloned = state.clone();
            let timeout = gloo_timers::callback::Timeout::new(3_000, move || cloned.set(None));
            timer.set(Some(timeout))
        })
    };

    let loader = match *state {
        Some(direction) => html! { <Loader {direction} color={Color::Danger}/> },
        None => html! {},
    };

    html! {
        <>
        <Button click={click(Direction::BottomToTop)} text={"Loading screen"}> </Button>
        {loader}
        </>
    }
}
