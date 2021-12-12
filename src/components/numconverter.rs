use crate::components::copytoclipboard::CopyToClipboard;

use yew::prelude::*;

pub struct NumConverter {
    link: ComponentLink<Self>,
    hex: String,
    dec: String,
    oct: String,
    bin: String,
}

pub enum NumConverterMsg {
    ConvertHex(String),
    ConvertDec(String),
    ConvertOct(String),
    ConvertBin(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}



impl Component for NumConverter {
    type Message = NumConverterMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NumConverter {
            link: link,
            hex: "".to_string(),
            dec: "".to_string(),
            oct: "".to_string(),
            bin: "".to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NumConverterMsg::ConvertHex(value) => {
                match i64::from_str_radix(&value, 16) {
                    Ok(v) => {
                        self.dec = format!("{}", v);
                        self.oct = format!("{:o}", v);
                        self.bin = format!("{:b}", v);
                    }
                    Err(_) => {
                        self.dec.clear();
                        self.oct.clear();
                        self.bin.clear();
                    }
                }

                self.hex = value;
            }
            NumConverterMsg::ConvertDec(value) => {
                match value.parse::<i64>() {
                    Ok(v) => {
                        self.hex = format!("{:X}", v);
                        self.oct = format!("{:o}", v);
                        self.bin = format!("{:b}", v);
                    }
                    Err(_) => {
                        self.hex.clear();
                        self.oct.clear();
                        self.bin.clear();
                    }
                }

                self.dec = value;
            }
            NumConverterMsg::ConvertOct(value) => {
                match i64::from_str_radix(&value, 8) {
                    Ok(v) => {
                        self.hex = format!("{:X}", v);
                        self.dec = format!("{}", v);
                        self.bin = format!("{:b}", v);
                    }
                    Err(_) => {
                        self.hex.clear();
                        self.dec.clear();
                        self.bin.clear();
                    }
                }

                self.oct = value;
            }
            NumConverterMsg::ConvertBin(value) => {
                match i64::from_str_radix(&value, 2) {
                    Ok(v) => {
                        self.hex = format!("{:X}", v);
                        self.dec = format!("{}", v);
                        self.oct = format!("{:o}", v);
                    }
                    Err(_) => {
                        self.hex.clear();
                        self.dec.clear();
                        self.oct.clear();
                    }
                }

                self.bin = value;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <form class="uk-grid-small" uk-grid="">
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="hex">{ "Hexadecimal" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="hex" />
                            <input class="uk-input uk-form-large"
                                   id="hex"
                                   type="text"
                                   oninput=self.link.callback(|d: InputData| NumConverterMsg::ConvertHex(d.value))
                                   value=self.hex.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="dec">{ "Decimal" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="dec" />
                            <input class="uk-input uk-form-large"
                                id="dec"
                                type="text"
                                oninput=self.link.callback(|d: InputData| NumConverterMsg::ConvertDec(d.value))
                                value=self.dec.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="oct">{ "Octal" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="oct" />
                            <input class="uk-input uk-form-large"
                                   id="oct"
                                   type="text"
                                   oninput=self.link.callback(|data: InputData| NumConverterMsg::ConvertOct(data.value))
                                   value=self.oct.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-2">
                    <label class="uk-form-label" for="bin">{ "Binary" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="bin" />
                            <input class="uk-input uk-form-large"
                                id="bin"
                                type="text"
                                oninput=self.link.callback(|data: InputData| NumConverterMsg::ConvertBin(data.value))
                                value=self.bin.clone() />
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}