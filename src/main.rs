mod components;
mod theme;

use crate::components::chmodcalculator::ChmodCalculator;
use crate::components::ipcalculator::IpCalculator;
use crate::components::navbar::Navbar;
use crate::components::numcalculator::NumCalculator;
use crate::components::numconverter::NumConverter;
use crate::components::timestampconverter::TimestampConverter;
use crate::theme::Theme;

use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{StorageService, Area};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const SELECTED_THEME_KEY: &'static str = "selected_theme";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SelectedTheme {
    Automatic,
    DarkMode,
    LightMode,
}

enum Msg {
    ThemeSetAutomatic,
    ThemeSetDarkMode,
    ThemeSetLightMode,
}

struct Model {
    link: ComponentLink<Self>,
    storage: Result<StorageService, &'static str>,
    theme: Theme,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            storage: StorageService::new(Area::Local),
            theme: Theme::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let selected_theme = match msg {
            Msg::ThemeSetAutomatic => {
                self.theme.set_automatic();
                SelectedTheme::Automatic
            },
            Msg::ThemeSetDarkMode  => {
                self.theme.set_dark_mode();
                SelectedTheme::DarkMode
            },
            Msg::ThemeSetLightMode => {
                self.theme.set_light_mode();
                SelectedTheme::LightMode
            },
        };

        if let Ok(storage) = &mut self.storage {
            storage.store(SELECTED_THEME_KEY, Json(&selected_theme))
        }

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
                <Navbar onautomatic = self.link.callback(|_| Msg::ThemeSetAutomatic)
                        ondarkmode  = self.link.callback(|_| Msg::ThemeSetDarkMode)
                        onlightmode = self.link.callback(|_| Msg::ThemeSetLightMode) />
                <div class="uk-container uk-margin-top uk-margin-large-bottom">
                    <h3 class="uk-heading-divider">
                        <a id="numcalculator"></a>
                        { "Numeric Calculator" }
                        { self.view_shortcut_label("ctrl", "1") }
                    </h3>
                    <div class="uk-container">
                        <NumCalculator />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a id="numconverter"></a>
                        { "Numeral Converter" }
                        { self.view_shortcut_label("ctrl", "2") }
                    </h3>
                    <div class="uk-container">
                        <NumConverter />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a id="ipcalculator"></a>
                        { "IP Calculator" }
                        { self.view_shortcut_label("ctrl", "3") }
                    </h3>
                    <div class="uk-container">
                        <IpCalculator />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a id="timestampconverter"></a>
                        { "Timestamp Converter" }
                        { self.view_shortcut_label("ctrl", "4") }
                    </h3>
                    <div class="uk-container">
                        <TimestampConverter />
                    </div>
                    <h3 class="uk-heading-divider">
                        <a id="chmodcalculator"></a>
                        { "CHMOD Calculator" }
                        { self.view_shortcut_label("ctrl", "5") }
                    </h3>
                    <div class="uk-container">
                        <ChmodCalculator />
                    </div>
                </div>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let selected_theme = match &self.storage {
                Ok(storage) => {
                    if let Json(Ok(theme)) = storage.restore(SELECTED_THEME_KEY) {
                        theme
                    } else {
                        SelectedTheme::Automatic
                    }
                },
                Err(_) => SelectedTheme::Automatic,
            };

            match selected_theme {
                SelectedTheme::Automatic => self.theme.set_automatic(),
                SelectedTheme::DarkMode  => self.theme.set_dark_mode(),
                SelectedTheme::LightMode => self.theme.set_light_mode(),
            }
        }
    }
}

impl Model {
    fn view_shortcut_label(&self, shortcut_first: &str, shortcut_second: &str) -> Html {
        html! {
            <span class="uk-align-right uk-text-small">
                <span class="uk-label">{ shortcut_first }</span>
                { " + " }
                <span class="uk-label">{ shortcut_second }</span>
            </span>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}