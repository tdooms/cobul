use yew::*;

use cobul::{Box, Button, Buttons, Color};

#[function_component(TooltipTester)]
pub fn tooltip_tester() -> Html {
    html! {
        <Box>
        <Buttons>
        <Button color={Color::Danger} tooltip="don't press me"> {"I has tooltip"} </Button>
        <Button color={Color::Warning} disabled=true tooltip="haha you can't pres me"> {"Surprise"} </Button>
        <Button color={Color::Warning} disabled=true> {"Surprise"} </Button>
        </Buttons>
        </Box>
    }
}
