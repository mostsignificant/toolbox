use crate::components::copytoclipboard::CopyToClipboard;

use yew::prelude::*;

pub struct NumCalculator {
    link: ComponentLink<Self>,
    result: String,
}

pub enum NumCalculatorMsg {
    Eval(String)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for NumCalculator {
    type Message = NumCalculatorMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NumCalculator {
            link: link,
            result: "".to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NumCalculatorMsg::Eval(expression) => {
                let mut ns = fasteval::EmptyNamespace;
                let ez = fasteval::ez_eval(&expression, &mut ns);
                match ez {
                    Ok(result) => { 
                        self.result = result.to_string();
                    }
                    Err(_e) => {
                        self.result = "".to_string();
                    }
                }
                return true;
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <form class="uk-grid-small" uk-grid="">
                <div class="uk-width-4-4">
                    <label class="uk-form-label" for="expression">{ "Expression" }</label>
                    <div class="uk-form-controls">
                        <input class="uk-input uk-form-large" 
                               id="expression"
                               placeholder="1+2*3" 
                               oninput=self.link.callback(|d: InputData| NumCalculatorMsg::Eval(d.value)) />
                    </div>
                </div>
                <div class="uk-width-1-4">
                    <label class="uk-form-label" for="result">{ "Result" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="result" />
                            <input class="uk-input uk-form-large"
                                   id="result"
                                   value=self.result.clone()
                                   disabled=true />
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}
