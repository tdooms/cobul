use yew::*;
use cobul::Content;

#[function_component(ContentTest)]
pub fn content() -> Html {
    html! {
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
    }
}