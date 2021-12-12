mod components;

use crate::components::ipcalculator::IpCalculator;
use crate::components::navbar::Navbar;
use crate::components::numcalculator::NumCalculator;
use crate::components::numconverter::NumConverter;
use crate::components::timestampconverter::TimestampConverter;

use yew::prelude::*;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navbar />
                <div class="uk-container uk-margin-top">
                    <h3 class="uk-heading-divider">
                        <a href="#numcalculator"></a>
                        { "Numeric Calculator" }
                    </h3>
                    <div class="uk-container">
                        <NumCalculator />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a href="#numconverter"></a>
                        { "Numeral Converter" }
                    </h3>
                    <div class="uk-container">
                        <NumConverter />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a href="#ipcalculator"></a>
                        { "IP Calculator" }
                    </h3>
                    <div class="uk-container">
                        <IpCalculator />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a href="#timestampconverter"></a>
                        { "Timestamp Converter" }
                    </h3>
                    <div class="uk-container">
                        <TimestampConverter />
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}