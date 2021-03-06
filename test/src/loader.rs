use cobul::{Box, Button, Color, Direction, Loader};
use yew::*;
use ywt::callback;

#[function_component(LoaderTester)]
pub fn loader_tester() -> Html {
    let state = use_state(|| None);
    let timer = use_state(|| None);

    let onclick = |dir: Direction| {
        callback!(state, timer; move |_| {
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
        <Box>
        <Button onclick={onclick(Direction::BottomToTop)}> {"Loading screen"} </Button>
        {loader}
        </Box>
    }
}
