use crate::components::copytoclipboard::CopyToClipboard;

use yew::prelude::*;

pub struct ColorHelper {
    link: ComponentLink<Self>,
    pub hex: String,
    pub rgb: String,
    pub cmyk: String,
}

pub enum ColorHelperMsg {
    ConvertHex(String),
    ConvertRGB(String),
    ConvertCMYK(String),
    Darker,
    Lighter,
    Complement,
    Random,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for ColorHelper {
    type Message = ColorHelperMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ColorHelper {
            link: link,
            hex: "".to_string(),
            rgb: "".to_string(),
            cmyk: "".to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ColorHelperMsg::ConvertHex(value) => {
                if value.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (u8::from_str_radix(&value[0..2], 16),
                                                    u8::from_str_radix(&value[2..4], 16),
                                                    u8::from_str_radix(&value[4..6], 16)) {
                        self.rgb = format!("{:?},{:?},{:?}", r, g, b);
                        self.cmyk = convert_rgb_to_cmyk_string(r, g, b); 
                    }
                }
                self.hex = value;
            }
            ColorHelperMsg::ConvertRGB(value) => {
                let rgb: Vec<&str> = value.split(",").collect();
                if rgb.len() == 3 {
                    if let (Ok(r), Ok(g), Ok(b)) = (rgb[0].parse::<u8>(),
                                                    rgb[1].parse::<u8>(),
                                                    rgb[2].parse::<u8>()) {
                        self.hex = format!("{:02X}{:02X}{:02X}", r, g, b);
                        self.cmyk = convert_rgb_to_cmyk_string(r, g, b); 
                    }
                }
                self.rgb = value;
            }
            ColorHelperMsg::ConvertCMYK(value) => {
                let cmyk: Vec<&str> = value.split(",").collect();
                if cmyk.len() == 4 {
                    if let (Ok(c), Ok(m), Ok(y), Ok(k)) = (cmyk[0].parse::<f32>(),
                                                           cmyk[1].parse::<f32>(),
                                                           cmyk[2].parse::<f32>(),
                                                           cmyk[3].parse::<f32>()) {
                        let r = (255.0 * (1.0 - c) * (1.0 - k)) as u8;
                        let g = (255.0 * (1.0 - m) * (1.0 - k)) as u8;
                        let b = (255.0 * (1.0 - y) * (1.0 - k)) as u8;

                        self.rgb = format!("{:?},{:?},{:?}", r, g, b);
                        self.hex = format!("{:02X}{:02X}{:02X}", r, g, b);
                    }
                }
                self.cmyk = value;
            }
            ColorHelperMsg::Darker => {
                if self.hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (u8::from_str_radix(&self.hex[0..2], 16),
                                                    u8::from_str_radix(&self.hex[2..4], 16),
                                                    u8::from_str_radix(&self.hex[4..6], 16)) {
                        let rdarker = r as f32 - r as f32 * 0.25;
                        let gdarker = g as f32 - g as f32 * 0.25;
                        let bdarker = b as f32 - b as f32 * 0.25;

                        return self.update(ColorHelperMsg::ConvertRGB(format!("{:?},{:?},{:?}", 
                            rdarker as u8, gdarker as u8, bdarker as u8)));
                    }
                }
            }
            ColorHelperMsg::Lighter => {
                if self.hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (u8::from_str_radix(&self.hex[0..2], 16),
                                                    u8::from_str_radix(&self.hex[2..4], 16),
                                                    u8::from_str_radix(&self.hex[4..6], 16)) {
                        let rdarker = if r == 0 { 4.0 } else { r as f32 + r as f32 * 0.25 };
                        let gdarker = if r == 0 { 4.0 } else { g as f32 + g as f32 * 0.25 };
                        let bdarker = if r == 0 { 4.0 } else { b as f32 + b as f32 * 0.25 };

                        return self.update(ColorHelperMsg::ConvertRGB(format!("{:?},{:?},{:?}", 
                            rdarker as u8, gdarker as u8, bdarker as u8)));
                    }
                }
            }
            ColorHelperMsg::Complement => {
                if self.hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (u8::from_str_radix(&self.hex[0..2], 16),
                                                    u8::from_str_radix(&self.hex[2..4], 16),
                                                    u8::from_str_radix(&self.hex[4..6], 16)) {
                        let rcomplement = 255 - r;
                        let gcomplement = 255 - g;
                        let bcomplement = 255 - b;

                        return self.update(ColorHelperMsg::ConvertRGB(format!("{:?},{:?},{:?}", 
                            rcomplement as u8, gcomplement as u8, bcomplement as u8)));
                    }
                }
            }
            ColorHelperMsg::Random => {
                let mut buf = [0u8; 3];
                getrandom::getrandom(&mut buf).unwrap();

                return self.update(ColorHelperMsg::ConvertRGB(format!("{:?},{:?},{:?}", buf[0], buf[1], buf[2])));
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form class="uk-grid-small uk-form-stacked" uk-grid="">
                    <div class="uk-width-1-4">
                        <label class="uk-form-label" for="hex">{ "Hex" }</label>
                        <div class="uk-form-controls">
                            <div class="uk-inline">
                                <CopyToClipboard from="hex" />
                                <input class="uk-input uk-form-large"
                                    id="hex"
                                    type="text"
                                    oninput=self.link.callback(|d: InputData| ColorHelperMsg::ConvertHex(d.value))
                                    value=self.hex.clone() />
                            </div>
                        </div>
                    </div>
                    <div class="uk-width-1-4">
                        <label class="uk-form-label" for="rgb">{ "RGB" }</label>
                        <div class="uk-form-controls">
                            <div class="uk-inline">
                                <CopyToClipboard from="rgb" />
                                <input class="uk-input uk-form-large"
                                    id="rgb"
                                    type="text"
                                    oninput=self.link.callback(|d: InputData| ColorHelperMsg::ConvertRGB(d.value))
                                    value=self.rgb.clone() />
                            </div>
                        </div>
                    </div>
                    <div class="uk-width-1-3">
                        <label class="uk-form-label" for="cmyk">{ "CMYK" }</label>
                        <div class="uk-form-controls">
                            <div class="uk-inline uk-width-expand">
                                <CopyToClipboard from="cmyk" />
                                <input class="uk-input uk-form-large"
                                    id="cmyk"
                                    type="text"
                                    oninput=self.link.callback(|d: InputData| ColorHelperMsg::ConvertCMYK(d.value))
                                    value=self.cmyk.clone() />
                            </div>
                        </div>
                    </div>
                    <div class="uk-width-1-6">
                        <label class="uk-form-label" for="random">{ '\u{00a0}' }</label>
                        <button class="uk-button uk-button-default uk-button-large uk-width-1-1" 
                                type="button" 
                                id="random"
                                uk-tooltip="pick a random color"
                                onclick=self.link.callback(|_| ColorHelperMsg::Random)>
                            {"Random"}
                        </button>
                    </div>
                </form>
                <div class="uk-container uk-margin-top">
                    <div class="uk-tile" style="background-color:#".to_string() + &self.hex>
                    </div>
                </div>
                <form class="uk-grid-small uk-form-stacked uk-margin-top" uk-grid="">
                    <div class="uk-width-1-6">
                        <button class="uk-button uk-button-default uk-button-large uk-width-1-1" 
                                type="button" 
                                id="darker"
                                uk-tooltip="go to 1/4 darker color"
                                onclick=self.link.callback(|_| ColorHelperMsg::Darker)>
                            {"Darker"}
                        </button>
                    </div>
                    <div class="uk-width-1-6">
                        <button class="uk-button uk-button-default uk-button-large uk-width-1-1" 
                                type="button" 
                                id="lighter"
                                uk-tooltip="go to 1/4 lighter color"
                                onclick=self.link.callback(|_| ColorHelperMsg::Lighter)>
                            {"Lighter"}
                        </button>
                    </div>
                    <div class="uk-width-1-6">
                        <button class="uk-button uk-button-default uk-button-large uk-width-1-1" 
                                type="button" 
                                id="complement"
                                uk-tooltip="switch to complementary color"
                                onclick=self.link.callback(|_| ColorHelperMsg::Complement)>
                            {"Complement"}
                        </button>
                    </div>
                </form>
            </>
        }
    }
}

fn convert_rgb_to_cmyk_string(r: u8, g: u8, b: u8) -> String {
    let rn: f32 = r as f32 / 255.0;
    let gn: f32 = g as f32 / 255.0;
    let bn: f32 = b as f32 / 255.0;

    let k = 1.0 - max_f32(rn, gn, bn);
    let c = if k == 1.0 { 0.0 } else { (1.0 - rn - k) / (1.0 - k) };
    let m = if k == 1.0 { 0.0 } else { (1.0 - gn - k) / (1.0 - k) };
    let y = if k == 1.0 { 0.0 } else { (1.0 - bn - k) / (1.0 - k) };

    format!("{:?},{:?},{:?},{:?}", c, m, y, k)
}

fn max_f32(a: f32, b: f32, c: f32) -> f32 {
    if a > b && a > c {
        return a;
    }

    if b > a && b > c {
        return b;
    }

    return c;
}
