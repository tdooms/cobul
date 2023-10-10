use yew::*;
use cobul::Box;

#[function_component(BoxTest)]
pub fn r#box() -> Html {
    html! {
        <Box> {"I'm in a box."} </Box>
    }
}