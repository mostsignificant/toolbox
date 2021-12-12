use yew::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Navbar {
}

pub enum NavbarMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Navbar {
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="uk-navbar-container" uk-navbar="">
                <a class="uk-navbar-item uk-logo" href="#">
                    <span class="uk-margin-small-right" uk-icon="album"></span>
                    <span>
                        { "toolbox" }
                        <span class="uk-text-small uk-text-muted uk-margin-small-left">
                            { VERSION }
                        </span>
                    </span>
                </a>
                <div class="uk-navbar-center">
                    <ul class="uk-navbar-nav">
                        <li class="uk-parent"><a href="#numcalculator">{ "Numeric Calculator" }</a></li>
                        <li class="uk-parent"><a href="#numconverter">{ "Numeral Converter" }</a></li>
                        <li class="uk-parent"><a href="#ipcalculator">{ "IP Calculator" }</a></li>
                        <li class="uk-parent"><a href="#timestampconverter">{ "Timestamp Converter" }</a></li>
                    </ul>
                </div>
            </nav>
        }
    }
}