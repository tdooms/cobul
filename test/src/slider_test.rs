use cobul::Slider;
use yew::*;

use cobul::LabelAlignment;

#[function_component(SliderTester)]
pub fn slider_tester() -> Html {
    let value = use_state_eq(|| 50.0);
    let cloned = value.clone();
    let oninput = Callback::from(move |x| cloned.set(x));

    let floats = html! {
        <>
        <Slider<f32> id="1" oninput={oninput.clone()} range={0.0..100.0} value={*value} step=1.0 fullwidth=true label={LabelAlignment::Right} fmt="{}%"/>
        <Slider<f32> id="2" oninput={oninput.clone()} range={0.0..50.0} value={*value} step=1.0 fullwidth=true/>
        <p> {value.to_string()} </p>
        </>
    };

    let value = use_state_eq(|| 50);
    let cloned = value.clone();
    let oninput = Callback::from(move |x| cloned.set(x));

    let integers = html! {
        <>
        <Slider<u32> id="3" oninput={oninput.clone()} range={0..100} value={*value} step=1 fullwidth=true label={LabelAlignment::Tooltip}/>
        <Slider<u32> id="4" oninput={oninput.clone()} range={0..20} value={*value} step=1 fullwidth=true label={LabelAlignment::Right} fmt="{}px"/>
        <p> {value.to_string()} </p>
        </>
    };

    html! {
        <>
        {floats}
        {integers}
        </>
    }
}
