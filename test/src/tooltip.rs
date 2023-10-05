use yew::*;

use cobul::{raw, Box, Button, Color};

#[function_component(TooltipTester)]
pub fn tooltip_tester() -> Html {
    html! {
        <Box>
        <raw::Buttons>
        <Button color={Color::Danger} tooltip="don't press me" text={"I has tooltip"} />
        <Button color={Color::Warning} disabled=true tooltip="haha you can't pres me" text={"I has tooltip"} />
        <Button color={Color::Warning} disabled=true text={"Surprise"} />
        </raw::Buttons>
        </Box>
    }
}
