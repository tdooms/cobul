use yew::*;

use cobul::{Box, Button, Color, Buttons};

#[function_component(TooltipTest)]
pub fn tooltip() -> Html {
    html! {
        <Buttons>
            <Button color={Color::Danger} tooltip="don't press me" text={"I has tooltip"} />
            <Button color={Color::Warning} disabled=true tooltip="haha you can't pres me" text={"I has tooltip"} />
            <Button color={Color::Warning} disabled=true text={"Surprise"} />
        </Buttons>
    }
}
