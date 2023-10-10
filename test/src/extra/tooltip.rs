use yew::*;

use cobul::{Button, Buttons, Color};

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
