use derive_more::Display;
use strum::EnumIter;
use yew::prelude::*;

use checkradio::CheckradioTester;
use cobul::*;
use dropdown::DropdownTester;
use form::{FormSize, FormHelp};
use elements::ButtonModelTest;
use loader::LoaderTester;
use pagination::PaginationTester;
use slider::SliderTester;
use switch::SwitchTester;
use tabs::TabsTester;
use tooltip::TooltipTester;
use cobul::icons::Solid;

mod elements;
mod checkradio;
mod dropdown;
mod form;
mod loader;
mod pagination;
mod slider;
mod switch;
mod tabs;
mod tooltip;

#[derive(Display, Clone, Copy, EnumIter, PartialEq)]
pub enum SelectEnum {
    #[display(fmt = "Select dropdown")]
    SelectDropdown,
    #[display(fmt = "With options")]
    WithOptions,
}

#[function_component(App)]
fn app() -> Html {
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
        <Columns>
        <Column offset={ColumnOffset::Is2} size={ColumnSize::Is6}>

        <SliderTester/>
        <SwitchTester/>
        <TabsTester/>
        <LoaderTester/>
        <TooltipTester/>
        <CheckradioTester/>
        <DropdownTester/>
        <PaginationTester/>

        <FormHelp/>
        <FormSize/>
        <ButtonModelTest/>

        <Block>
            {"This text is within a "} <strong>{"block"}</strong>{"."}
        </Block>
        <Block>
            {"This text is within a "}
            <strong>{"second block"}</strong>
            {". Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
            Aenean efficitur sit amet massa fringilla egestas. \
            Nullam condimentum luctus turpis. "}
        </Block>
        <Block>
            {"This text is within a "}
            <strong>{"third block"}</strong>
            {". This block has no margin at the bottom."}
        </Block>

        <Box> {"I'm in a box."} </Box>

        <raw::Buttons>
        { for TONE_COLORS.iter().cloned().map(|(color, text)| html!{ <Button {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button light=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for SIZES.iter().cloned().map(|(size, text)| html!{ <Button {size} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for SIZES.iter().cloned().map(|(size, text)| html!{ <Button fullwidth=true {size} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button outlined=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button inverted=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button rounded=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button hovered=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button focussed=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button active=true {color} {text} /> })}
        </raw::Buttons>
        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button loading=true {color} {text} /> })}
        </raw::Buttons>

        <raw::Buttons>
        <Button statik=true text="Static" />
        </raw::Buttons>

        <raw::Buttons>
        { for COLORS.iter().cloned().map(|(color, text)| html!{ <Button disabled=true {color} {text} /> })}
        </raw::Buttons>

        <Content>
            <h1>{"Hello World"}</h1>
            <p>
                {"Lorem ipsum"} <sup><a>{"[1]"}</a></sup>
                {" dolor sit amet, consectetur adipiscing elit. Nulla accumsan, \
                metus ultrices eleifend gravida, nulla nunc varius lectus, \
                nec rutrum justo nibh eu lectus. Ut vulputate semper dui. \
                Fusce erat odio, sollicitudin vel erat vel, \
                interdum mattis neque. Sub"}
                <sub>{"script"}</sub>
                {" works as well!"}
            </p>
        </Content>

        <Block>
        // { for SIZES.iter().map(|(s, _)| html!{ <Delete size={s.clone()} onclick={Callback::noop()}/> })}
        </Block>

        <Block>
        <Icon icon={Solid::Plus}/>
        </Block>

        <Block>
        <IconText>
            <Icon icon="fas fa-home"/>
            <span> {"Home"} </span>
        </IconText>
        </Block>

        <Block>
        <IconText>
            <Icon icon="fas fa-train"/>
            <span> {"Paris"} </span>
            <Icon icon="fas fa-arrow-right"/>
            <span> {"Budapest"} </span>
            <Icon icon="fas fa-arrow-right"/>
            <span> {"Bucharest"} </span>
            <Icon icon="fas fa-arrow-right"/>
            <span> {"Istanbul"} </span>
            <Icon icon="fas fa-flag-checkered"/>
        </IconText>
        </Block>

        <Block>
        <IconText>
            <Icon color={TextColor::Info} icon="fas fa-info-circle"/>
            <span>{"Information"}</span>
            <Block>
                {"Your package will be delivered on "}
                <strong> {"Tuesday at 08:00"} </strong>
                {"."}
            </Block>
        </IconText>
        </Block>

        // TODO: continue from image

        <raw::Field>
            <raw::Label> {"Name"} </raw::Label>
            <raw::Control>
                <raw::Input placeholder="Text input" input={Callback::noop()}/>
            </raw::Control>
        </raw::Field>

        <raw::Field>
            <raw::Label> {"Username"} </raw::Label>
            <raw::Control right="fas fa-user" left="fas fa-check">
                <raw::Input color={Color::Success} placeholder="Text input" value="bulma" input={Callback::noop()}/>
                <raw::Help color={Color::Success}> {"This username is available"} </raw::Help>
            </raw::Control>
        </raw::Field>

        <raw::Field>
            <raw::Label> {"Email"} </raw::Label>
            <raw::Control right="fas fa-envelope" left="fas fa-exclamation-triangle">
                <raw::Input color={Color::Danger} placeholder="Text input" value="bulma" input={Callback::noop()}/>
                <raw::Help color={Color::Danger}> {"This email is invalid"} </raw::Help>
            </raw::Control>
        </raw::Field>

        <raw::Field>
            <raw::Label> {"Subject"} </raw::Label>
            <raw::Control>
                <Select<SelectEnum> value={SelectEnum::SelectDropdown} input={Callback::noop()}/>
            </raw::Control>
        </raw::Field>

        <raw::Field>
            <raw::Label> {"Message"} </raw::Label>
            <raw::Control>
                <Textarea placeholder="Textarea" input={Callback::noop()}/>
            </raw::Control>
        </raw::Field>

        <raw::Field grouped=true>
            <raw::Control>
                <Button color={Color::Link} text="Submit" />
            </raw::Control>
            <raw::Control>
                <Button color={Color::Link} light=true text="Cancel" />
            </raw::Control>
        </raw::Field>

        </Column>
        </Columns>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
