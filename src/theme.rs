use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/toolbox.js")]
extern "C" {
    fn setDarkMode();
    fn setLightMode();
    fn getPrefersColorSchemeDark() -> bool;
}

#[derive(PartialEq)]
pub struct Theme {
}

impl Theme {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn set_automatic(&mut self) {
        if getPrefersColorSchemeDark() {
            setDarkMode();
        } else {
            setLightMode();
        }
    }

    pub fn set_dark_mode(&mut self) {
        setDarkMode();
    }

    pub fn set_light_mode(&mut self) {
        setLightMode();
    }
}