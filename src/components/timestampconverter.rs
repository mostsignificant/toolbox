use crate::components::copytoclipboard::CopyToClipboard;

use chrono::prelude::*;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

const DATETIMEFORMATS: &'static [&'static str] = &["%Y-%m-%dT%H:%M:%SZ", "%Y-%m-%d %H:%M:%S", "%a, %e %b %Y %T"];

#[wasm_bindgen(module = "/toolbox.js")]
extern "C" {
    fn getCurrentUTC() -> String;
}

pub struct TimestampConverter {
    link: ComponentLink<Self>,
    epoch: String,
    human: String,
    format: String,
}

pub enum TimestampConverterMsg {
    ConvertEpoch(String),
    ConvertHuman(String),
    ChangeFormat(String),
    SetNow,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for TimestampConverter {
    type Message = TimestampConverterMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TimestampConverter {
            link: link,
            epoch: "".to_string(),
            human: "".to_string(),
            format: DATETIMEFORMATS[0].to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TimestampConverterMsg::ConvertEpoch(value) => {
                if let Ok(secs) =  value.parse::<u64>() {
                    let ts = UNIX_EPOCH + Duration::from_secs(secs);
                    let dt = DateTime::<Utc>::from(ts);
                    self.human = dt.format(&self.format).to_string();
                }

                self.epoch = value;
            }
            TimestampConverterMsg::ConvertHuman(value) => {
                match NaiveDateTime::parse_from_str(&value, &self.format) {
                    Ok(dt) => self.epoch = dt.timestamp().to_string(),
                    Err(_) => {}
                }

                self.human = value;
            }
            TimestampConverterMsg::ChangeFormat(format) => {
                self.format = format;

                if let Ok(secs) =  self.epoch.parse::<u64>() {
                    let ts = UNIX_EPOCH + Duration::from_secs(secs);
                    let dt = DateTime::<Utc>::from(ts);
                    self.human = dt.format(&self.format).to_string();
                    self.epoch = dt.timestamp().to_string();
                }
            }
            TimestampConverterMsg::SetNow => {
                match DateTime::parse_from_rfc2822(&getCurrentUTC()) {
                    Ok(dt) => {
                        self.epoch = dt.timestamp().to_string();
                        self.human = dt.format(&self.format).to_string();
                    }
                    Err(_) => {}
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <form class="uk-grid-small" uk-grid="">
                <div class="uk-width-1-4">
                    <label class="uk-form-label" for="epoch">{ "UNIX Epoch" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="epoch" />
                            <input class="uk-input uk-form-large"
                                id="epoch"
                                type="text"
                                oninput=self.link.callback(|d: InputData| TimestampConverterMsg::ConvertEpoch(d.value))
                                value=self.epoch.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-4">
                    <label class="uk-form-label" for="human">{ "Human" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="human" />
                            <input class="uk-input uk-form-large"
                                id="human"
                                type="text"
                                oninput=self.link.callback(|d: InputData| TimestampConverterMsg::ConvertHuman(d.value))
                                value=self.human.clone() />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-3">
                    <label class="uk-form-label" for="format">{ "Format" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <button class="uk-button uk-button-default uk-button-large" type="button" id="format">
                                { self.format.clone() }
                            </button>
                            <div uk-dropdown="">
                                <ul class="uk-nav uk-dropdown-nav">
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| TimestampConverterMsg::ChangeFormat(DATETIMEFORMATS[0].to_string()))>
                                            { DATETIMEFORMATS[0].to_string() }
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| TimestampConverterMsg::ChangeFormat(DATETIMEFORMATS[1].to_string()))>
                                            { DATETIMEFORMATS[1].to_string() }
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#" onclick=self.link.callback(|_| TimestampConverterMsg::ChangeFormat(DATETIMEFORMATS[2].to_string()))>
                                            { DATETIMEFORMATS[2].to_string() }
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="now">{ '\u{00a0}' }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <button class="uk-button uk-button-default uk-button-large" 
                                    type="button" 
                                    id="now"
                                    uk-tooltip="use current time"
                                    onclick=self.link.callback(|_| TimestampConverterMsg::SetNow)>
                                {"Now"}
                            </button>
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}