use derive_more::Display;
use strum::EnumIter;
use yew::prelude::*;

use cobul::props::{Color, ColumnOffset, ColumnSize, Size, TextColor};
use cobul::*;
use form_test::FormTester;

mod form_test;

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
        <FormTester/>
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

        <Buttons>
        { for TONE_COLORS.iter().map(|(c, n)| html!{ <Button color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button light=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for SIZES.iter().map(|(s, n)| html!{ <Button size={s.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for SIZES.iter().map(|(s, n)| html!{ <Button fullwidth=true size={s.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button outlined=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button inverted=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button rounded=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button hovered=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button focussed=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button active=true color={c.clone()}> {n} </Button> })}
        </Buttons>
        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button loading=true color={c.clone()}> {n} </Button> })}
        </Buttons>

        <Buttons>
        <Button r#static=true> {"Static"} </Button>
        </Buttons>

        <Buttons>
        { for COLORS.iter().map(|(c, n)| html!{ <Button disabled=true color={c.clone()}> {n} </Button> })}
        </Buttons>

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
        { for SIZES.iter().map(|(s, _)| html!{ <Delete size={s.clone()} onclick={Callback::noop()}/> })}
        </Block>

        <Block>
        <Icon icon={Icons::Plus}/>
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

        <Field>
            <Label> {"Name"} </Label>
            <Control>
                <Input placeholder="Text input" oninput={Callback::noop()}/>
            </Control>
        </Field>

        <Field>
            <Label> {"Username"} </Label>
            <Control right="fas fa-user" left="fas fa-check">
                <Input color={Color::Success} placeholder="Text input" value="bulma" oninput={Callback::noop()}/>
                <Help color={Color::Success}> {"This username is available"} </Help>
            </Control>
        </Field>

        <Field>
            <Label> {"Email"} </Label>
            <Control right="fas fa-envelope" left="fas fa-exclamation-triangle">
                <Input color={Color::Danger} placeholder="Text input" value="bulma" oninput={Callback::noop()}/>
                <Help color={Color::Danger}> {"This email is invalid"} </Help>
            </Control>
        </Field>

        <Field>
            <Label> {"Subject"} </Label>
            <Control>
                <Select<SelectEnum> selected={SelectEnum::SelectDropdown} onselect={Callback::noop()}/>
            </Control>
        </Field>

        <Field>
            <Label> {"Message"} </Label>
            <Control>
                <Textarea placeholder="Textarea" oninput={Callback::noop()}/>
            </Control>
        </Field>

        <Field grouped=true>
            <Control>
                <Button color={Color::Link}> {"Submit"} </Button>
            </Control>
            <Control>
                <Button color={Color::Link} light=true> {"Cancel"} </Button>
            </Control>
        </Field>

        </Column>
        </Columns>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
