use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/toolbox.js")]
extern "C" {
    fn copyToClipboard(id: String);
}

pub struct CopyToClipboard {
    link: ComponentLink<Self>,
    from: String,
}

pub enum CopyToClipboardMsg {
    Run,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub from: String,
}

impl Component for CopyToClipboard {
    type Message = CopyToClipboardMsg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CopyToClipboard {
            link: link,
            from: props.from,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CopyToClipboardMsg::Run => {
                copyToClipboard(self.from.clone());
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <a class="uk-form-icon uk-form-icon-flip" 
                href="#" 
                uk-icon="icon: copy"
                uk-tooltip="copy to clipboard"
                onclick=self.link.callback(|_| CopyToClipboardMsg::Run)>
            </a>
        }
    }
}