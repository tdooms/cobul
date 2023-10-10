use cobul::{Button, Buttons, Color, Size};
use yew::*;

#[function_component(ButtonTest)]
pub fn button() -> Html {
    const TONE_COLORS: [(Color, &str); 6] = [
        (Color::White, "White"),
        (Color::Link, "Link"),
        (Color::Dark, "Dark"),
        (Color::Black, "Black"),
        (Color::Text, "Text"),
        (Color::Ghost, "Ghost"),
    ];

    const COLORS: [(Color, &str); 6] = [
        (Color::Primary, "Primary"),
        (Color::Link, "Link"),
        (Color::Info, "Info"),
        (Color::Success, "Success"),
        (Color::Warning, "Warning"),
        (Color::Danger, "Danger"),
    ];

    const SIZES: [(Size, &str); 4] = [
        (Size::Small, "Small"),
        (Size::Normal, "Normal"),
        (Size::Medium, "Medium"),
        (Size::Large, "Large"),
    ];

    html! {
        <>
        <Buttons>
        { for TONE_COLORS.iter().cloned().map(|(color, text)| html!{ <Button {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button light=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for SIZES.iter().cloned().map(|(size, text)| html!{ <Button {size} {text} /> })}
        </Buttons>

        <Buttons>
        { for SIZES.iter().cloned().map(|(size, text)| html!{ <Button fullwidth=true {size} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button outlined=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button inverted=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button rounded=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button hovered=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button focussed=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button active=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button loading=true {color} {text} /> })}
        </Buttons>

        <Buttons>
        <Button statik=true text="Static" />
        </Buttons>

        <Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button disabled=true {color} {text} /> })}
        </Buttons>
        </>
    }
}
