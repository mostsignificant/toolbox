use crate::components::copytoclipboard::CopyToClipboard;

use std::net::Ipv4Addr;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/toolbox.js")]
extern "C" {
    fn getMyIP() -> String;
}

pub struct IpCalculator {
    link: ComponentLink<Self>,
    ipv4: String,
    integer: String,
    bits: String,
}

pub enum IpCalculatorMsg {
    ConvertBits(String),
    ConvertInteger(String),
    ConvertIpv4(String),
    MyIP,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for IpCalculator {
    type Message = IpCalculatorMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IpCalculator {
            link: link,
            ipv4: "".to_string(),
            integer: "".to_string(),
            bits: "".to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            IpCalculatorMsg::ConvertBits(value) => {
                let bytes: Vec<&str> = value.split(".").collect();

                if bytes.len() > 0 && bytes.len() <= 4 {
                    let mut converted_bytes: [u8; 4] = [0; 4];

                    for (i, byte) in bytes.iter().enumerate() {
                        match u8::from_str_radix(byte, 2) {
                            Ok(byte) => converted_bytes[i] = byte,
                            Err(_) => return false,
                        }
                    }

                    let addr = Ipv4Addr::new(
                        converted_bytes[0], 
                        converted_bytes[1], 
                        converted_bytes[2], 
                        converted_bytes[3]);

                    self.ipv4 = addr.to_string();
                    self.integer = (((addr.octets()[0] as u32) << 24) + 
                                    ((addr.octets()[1] as u32) << 16) + 
                                    ((addr.octets()[2] as u32) <<  8) + 
                                    ((addr.octets()[3] as u32))).to_string();
                }

                self.bits = value;
            }
            IpCalculatorMsg::ConvertInteger(value) => {
                if let Ok(value) = value.parse::<u32>() {
                    let addr = Ipv4Addr::from(value);
                    self.ipv4 = addr.to_string();
                    self.bits = format!("{:08b}.{:08b}.{:08b}.{:08b}", 
                                        addr.octets()[0] as u32, 
                                        addr.octets()[1] as u32,
                                        addr.octets()[2] as u32,
                                        addr.octets()[3] as u32);
                }

                self.integer = value;
            }
            IpCalculatorMsg::ConvertIpv4(value) => {
                match value.parse::<Ipv4Addr>() {
                    Ok(addr) => { 
                        self.integer = (((addr.octets()[0] as u32) << 24) + 
                                        ((addr.octets()[1] as u32) << 16) + 
                                        ((addr.octets()[2] as u32) <<  8) + 
                                        ((addr.octets()[3] as u32))).to_string();
                        self.bits = format!("{:08b}.{:08b}.{:08b}.{:08b}", 
                                            addr.octets()[0] as u32, 
                                            addr.octets()[1] as u32,
                                            addr.octets()[2] as u32,
                                            addr.octets()[3] as u32);
                    }
                    Err(_) => self.clear()
                }

                self.ipv4 = value;
            }
            IpCalculatorMsg::MyIP => {
                let myip = getMyIP();
                match myip.parse::<Ipv4Addr>() {
                    Ok(addr) => { 
                        self.ipv4 = myip;
                        self.integer = (((addr.octets()[0] as u32) << 24) + 
                                        ((addr.octets()[1] as u32) << 16) + 
                                        ((addr.octets()[2] as u32) <<  8) + 
                                        ((addr.octets()[3] as u32))).to_string();
                        self.bits = format!("{:08b}.{:08b}.{:08b}.{:08b}", 
                                            addr.octets()[0] as u32, 
                                            addr.octets()[1] as u32,
                                            addr.octets()[2] as u32,
                                            addr.octets()[3] as u32);
                    }
                    Err(_) => self.clear()
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <form class="uk-grid-small uk-form-stacked" uk-grid="">
                <div class="uk-width-1-4">
                    <label class="uk-form-label" for="ipv4">{ "IPv4" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="ipv4" />
                            <input class="uk-input uk-form-large mousetrap"
                                   id="ipv4"
                                   type="text"
                                   oninput=self.link.callback(|d: InputData| IpCalculatorMsg::ConvertIpv4(d.value))
                                   value=self.ipv4.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-4">
                    <label class="uk-form-label" for="integer">{ "Integer" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="integer" />
                            <input class="uk-input uk-form-large"
                                   id="integer"
                                   type="text"
                                   oninput=self.link.callback(|d: InputData| IpCalculatorMsg::ConvertInteger(d.value))
                                   value=self.integer.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-3">
                    <label class="uk-form-label" for="bits">{ "Bits" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="bits" />
                            <input class="uk-input uk-form-large" 
                                   id="bits"
                                   type="text"
                                   oninput=self.link.callback(|d: InputData| IpCalculatorMsg::ConvertBits(d.value))
                                   value=self.bits.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="myip">{ '\u{00a0}' }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <button class="uk-button uk-button-default uk-button-large" 
                                    type="button" 
                                    id="myip"
                                    uk-tooltip="use my IP address"
                                    onclick=self.link.callback(|_| IpCalculatorMsg::MyIP)>
                                {"My IP"}
                            </button>
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}

impl IpCalculator {
    fn clear(&mut self) {
        self.ipv4.clear();
        self.integer.clear();
        self.bits.clear();
    }
}