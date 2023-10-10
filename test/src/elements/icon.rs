use yew::*;
use cobul::{Block, IconText, TextColor, Icon, icons::Solid};

#[function_component(IconTest)]
pub fn icon() -> Html {
    html! {
        <>
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
        </>
    }
}