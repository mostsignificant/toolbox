use yew::prelude::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Navbar {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum NavbarMsg {
    SetAutomatic,
    SetDarkMode,
    SetLightMode,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onautomatic: Callback<()>,
    pub ondarkmode: Callback<()>,
    pub onlightmode: Callback<()>,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link: link,
            props: props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavbarMsg::SetAutomatic => self.props.onautomatic.emit(()),
            NavbarMsg::SetDarkMode  => self.props.ondarkmode.emit(()),
            NavbarMsg::SetLightMode => self.props.onlightmode.emit(()),
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="uk-navbar-container uk-navbar-transparent" uk-navbar="">
                <div class="uk-navbar-left">
                    <a class="uk-navbar-item uk-logo" href="#">
                        <span>
                            <span uk-icon="album" class="uk-margin-small-right uk-icon"></span>
                            { "toolbox" }
                            <span class="uk-text-small uk-text-muted uk-margin-small-left">
                                { VERSION }
                            </span>
                        </span>
                    </a>
                </div>
                <div class="uk-navbar-right">
                    <ul class="uk-navbar-nav">
                        <li>
                            <a href="#" class="uk-navbar-toggle">
                                <span class="uk-margin-small-right">{ "Menu" }</span>
                                <span uk-navbar-toggle-icon="" class="uk-icon uk-navbar-toggle-icon"></span>
                            </a>
                            <div class="uk-navbar-dropdown">
                                <ul class="uk-nav uk-navbar-dropdown-nav">
                                    <li class="uk-nav-header">{ "Tools" }</li>
                                    <li><a href="#numcalculator">{ "Numeric Calculator" }</a></li>
                                    <li><a href="#numconverter">{ "Numeral Converter" }</a></li>
                                    <li><a href="#ipcalculator">{ "IP Calculator" }</a></li>
                                    <li><a href="#timestampconverter">{ "Timestamp Converter" }</a></li>
                                    <li><a href="#chmodcalculator">{ "CHMOD Calculator" }</a></li>
                                    <li><a href="#colorhelper">{ "Color Helper" }</a></li>
                                    <li class="uk-nav-header">{ "Theme" }</li>
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| NavbarMsg::SetDarkMode)>
                                            { "Dark Mode" }
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| NavbarMsg::SetLightMode)>
                                            { "Light Mode" }
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| NavbarMsg::SetAutomatic)>
                                            { "Automatic" }
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}