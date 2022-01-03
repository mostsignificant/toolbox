use crate::components::copytoclipboard::CopyToClipboard;

use yew::prelude::*;

pub struct ChmodCalculator {
    link: ComponentLink<Self>,
    owner_read: bool,
    owner_write: bool,
    owner_execute: bool,
    group_read: bool,
    group_write: bool,
    group_execute: bool,
    public_read: bool,
    public_write: bool,
    public_execute: bool,
    octal: String,
    text: String,
    command: String,
}

#[derive(Clone)]
pub enum ChmodCalculatorMsg {
    ChangeOwnerRead,
    ChangeOwnerWrite,
    ChangeOwnerExecute,
    ChangeGroupRead,
    ChangeGroupWrite,
    ChangeGroupExecute,
    ChangePublicRead,
    ChangePublicWrite,
    ChangePublicExecute,
    ChangeOctal(String),
    ChangeText(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for ChmodCalculator {
    type Message = ChmodCalculatorMsg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ChmodCalculator {
            link: link,
            owner_read: false,
            owner_write: false,
            owner_execute: false,
            group_read: false,
            group_write: false,
            group_execute: false,
            public_read: false,
            public_write: false,
            public_execute: false,
            octal: "000".to_string(),
            text: "---------".to_string(),
            command: "chmod 000".to_string(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChmodCalculatorMsg::ChangeOwnerRead        => { self.owner_read     = !self.owner_read; }
            ChmodCalculatorMsg::ChangeOwnerWrite       => { self.owner_write    = !self.owner_write; }
            ChmodCalculatorMsg::ChangeOwnerExecute     => { self.owner_execute  = !self.owner_execute; }
            ChmodCalculatorMsg::ChangeGroupRead        => { self.group_read     = !self.group_read; }
            ChmodCalculatorMsg::ChangeGroupWrite       => { self.group_write    = !self.group_write; }
            ChmodCalculatorMsg::ChangeGroupExecute     => { self.group_execute  = !self.group_execute; }
            ChmodCalculatorMsg::ChangePublicRead       => { self.public_read    = !self.public_read; }
            ChmodCalculatorMsg::ChangePublicWrite      => { self.public_write   = !self.public_write; }
            ChmodCalculatorMsg::ChangePublicExecute    => { self.public_execute = !self.public_execute; }
            ChmodCalculatorMsg::ChangeOctal(ref value) => { self.octal          = value.to_string(); }
            ChmodCalculatorMsg::ChangeText(ref value)  => { self.text           = value.to_string(); }
        }

        match msg {
            ChmodCalculatorMsg::ChangeOwnerRead    |
            ChmodCalculatorMsg::ChangeOwnerWrite   |
            ChmodCalculatorMsg::ChangeOwnerExecute |
            ChmodCalculatorMsg::ChangeGroupRead    |
            ChmodCalculatorMsg::ChangeGroupWrite   |
            ChmodCalculatorMsg::ChangeGroupExecute |
            ChmodCalculatorMsg::ChangePublicRead   |
            ChmodCalculatorMsg::ChangePublicWrite  |
            ChmodCalculatorMsg::ChangePublicExecute => {
                self.calc_text_from_bits();
                self.calc_octal_from_bits();
                self.calc_command_from_octal();
            }

            ChmodCalculatorMsg::ChangeOctal(ref value) => { 
                if value.len() == 3 {
                    match i64::from_str_radix(&value, 8) {
                        Ok(_) => {
                            self.calc_bits_from_octal();
                            self.calc_text_from_bits();
                            self.calc_command_from_octal();
                        }
                        Err(_) => {}
                    }    
                }
            }

            ChmodCalculatorMsg::ChangeText(ref value) => { 
                if is_valid_text(value) {
                    self.calc_bits_from_text();
                    self.calc_octal_from_text();
                    self.calc_command_from_octal();
                }
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <form class="uk-grid-small uk-form-stacked" uk-grid="">
                <div class="uk-width-1-6 ">
                    <label class="uk-form-label" for="owner">{ "Owner" }</label>
                    <div class="uk-form-controls" id="owner">
                        { self.label("read",    self.owner_read,    ChmodCalculatorMsg::ChangeOwnerRead) }
                        <br/>
                        { self.label("write",   self.owner_write,   ChmodCalculatorMsg::ChangeOwnerWrite) }
                        <br/>
                        { self.label("execute", self.owner_execute, ChmodCalculatorMsg::ChangeOwnerExecute) }
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="group">{ "Group" }</label>
                    <div class="uk-form-controls" id="group">
                        { self.label("read",    self.group_read,    ChmodCalculatorMsg::ChangeGroupRead) }
                        <br/>
                        { self.label("write",   self.group_write,   ChmodCalculatorMsg::ChangeGroupWrite) }
                        <br/>
                        { self.label("execute", self.group_execute, ChmodCalculatorMsg::ChangeGroupExecute) }
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="public">{ "Public" }</label>
                    <div class="uk-form-controls " id="public">
                        { self.label("read",    self.public_read,    ChmodCalculatorMsg::ChangePublicRead) }
                        <br/>
                        { self.label("write",   self.public_write,   ChmodCalculatorMsg::ChangePublicWrite) }
                        <br/>
                        { self.label("execute", self.public_execute, ChmodCalculatorMsg::ChangePublicExecute) }
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="octal">{ "Octal" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline">
                            <CopyToClipboard from="octal" />
                            <input class="uk-input uk-form-large mousetrap"
                                   id="octal"
                                   type="text"
                                   value=self.octal.clone()
                                   oninput=self.link.callback(|d: InputData| ChmodCalculatorMsg::ChangeOctal(d.value)) />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="text">{ "Text" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="text" />
                            <input class="uk-input uk-form-large" 
                                   id="text"
                                   type="text"
                                   value=self.text.clone()
                                   oninput=self.link.callback(|d: InputData| ChmodCalculatorMsg::ChangeText(d.value)) />
                        </div>
                    </div>
                </div>
                <div class="uk-width-1-6">
                    <label class="uk-form-label" for="command">{ "Command" }</label>
                    <div class="uk-form-controls">
                        <div class="uk-inline uk-width-expand">
                            <CopyToClipboard from="command" />
                            <input class="uk-input uk-form-large" 
                                   id="command"
                                   type="text"
                                   value=self.command.clone()
                                   disabled=true />
                        </div>
                    </div>
                </div>
            </form>
        }
    }
}

impl ChmodCalculator {
    /// Combines the rwx bits from owner, group, and public to the octal value
    fn calc_octal_from_bits(&mut self) {
        let mut owner: u8 = 0b0000_0000;
        if self.owner_read    { owner |= 0b0000_0100; }
        if self.owner_write   { owner |= 0b0000_0010; }
        if self.owner_execute { owner |= 0b0000_0001; }

        let mut group: u8 = 0b0000_0000;
        if self.group_read    { group |= 0b0000_0100; }
        if self.group_write   { group |= 0b0000_0010; }
        if self.group_execute { group |= 0b0000_0001; }

        let mut public: u8 = 0b0000_0000;
        if self.public_read    { public |= 0b0000_0100; }
        if self.public_write   { public |= 0b0000_0010; }
        if self.public_execute { public |= 0b0000_0001; }

        self.octal = format!("{:o}{:o}{:o}", owner, group, public);
    }

    /// Combines the rwx bits from owner, group, and public to the human-readable text value
    fn calc_text_from_bits(&mut self) {
        self.text.clear();

        if self.owner_read    { self.text.push('r'); } else { self.text.push('-') }
        if self.owner_write   { self.text.push('w'); } else { self.text.push('-') }
        if self.owner_execute { self.text.push('x'); } else { self.text.push('-') }

        if self.group_read    { self.text.push('r'); } else { self.text.push('-') }
        if self.group_write   { self.text.push('w'); } else { self.text.push('-') }
        if self.group_execute { self.text.push('x'); } else { self.text.push('-') }

        if self.public_read    { self.text.push('r'); } else { self.text.push('-') }
        if self.public_write   { self.text.push('w'); } else { self.text.push('-') }
        if self.public_execute { self.text.push('x'); } else { self.text.push('-') }
    }

    /// Reads the rwx bits from the octal value and writes it to the rwx bits from owner, group, and public
    fn calc_bits_from_octal(&mut self) {
        if self.octal.len() != 3 {
            return;
        }

        if let Ok(octal) = i64::from_str_radix(&self.octal[0..1], 8) {
            self.owner_read    = octal & (1 << 2) != 0;
            self.owner_write   = octal & (1 << 1) != 0;
            self.owner_execute = octal & (1 << 0) != 0;
        }

        if let Ok(octal) = i64::from_str_radix(&self.octal[1..2], 8) {
            self.group_read    = octal & (1 << 2) != 0;
            self.group_write   = octal & (1 << 1) != 0;
            self.group_execute = octal & (1 << 0) != 0;
        }

        if let Ok(octal) = i64::from_str_radix(&self.octal[2..3], 8) {
            self.public_read    = octal & (1 << 2) != 0;
            self.public_write   = octal & (1 << 1) != 0;
            self.public_execute = octal & (1 << 0) != 0;
        }
    }

    /// Reads the human-readable text value and writes the corresponding rwx bits from owner, group, and public
    fn calc_bits_from_text(&mut self) {
        if self.text.len() != 9 {
            return;
        }

        let mut char_indices = self.text.char_indices();

        self.owner_read     = char_indices.next() == Some((0, 'r'));
        self.owner_write    = char_indices.next() == Some((1, 'w'));
        self.owner_execute  = char_indices.next() == Some((2, 'x'));
        self.group_read     = char_indices.next() == Some((3, 'r'));
        self.group_write    = char_indices.next() == Some((4, 'w'));
        self.group_execute  = char_indices.next() == Some((5, 'x'));
        self.public_read    = char_indices.next() == Some((6, 'r'));
        self.public_write   = char_indices.next() == Some((7, 'w'));
        self.public_execute = char_indices.next() == Some((8, 'x'));
    }

    /// Reads the human-readable text value and writes to the octal value
    fn calc_octal_from_text(&mut self) {
        let mut char_indices = self.text.char_indices();

        let mut owner: u8 = 0b0000_0000;
        if char_indices.next() == Some((0, 'r')) { owner |= 0b0000_0100; }
        if char_indices.next() == Some((1, 'w')) { owner |= 0b0000_0010; }
        if char_indices.next() == Some((2, 'x')) { owner |= 0b0000_0001; }

        let mut group: u8 = 0b0000_0000;
        if char_indices.next() == Some((0, 'r')) { group |= 0b0000_0100; }
        if char_indices.next() == Some((1, 'w')) { group |= 0b0000_0010; }
        if char_indices.next() == Some((2, 'x')) { group |= 0b0000_0001; }

        let mut public: u8 = 0b0000_0000;
        if char_indices.next() == Some((0, 'r')) { public |= 0b0000_0100; }
        if char_indices.next() == Some((1, 'w')) { public |= 0b0000_0010; }
        if char_indices.next() == Some((2, 'x')) { public |= 0b0000_0001; }

        self.octal = format!("{:o}{:o}{:o}", owner, group, public);
    }

    /// Reads the octal value and prepends 'chmod ' for the command
    fn calc_command_from_octal(&mut self) {
        self.command = "chmod ".to_owned() + &self.octal;
    }

    /// Creates a label for one of the rwx bits
    fn label(&self, text: &str, checked: bool, msg: ChmodCalculatorMsg) -> Html {
        html! {
            <label class="uk-margin-small-right">
                <input class="uk-checkbox" type="checkbox" checked=checked onclick=self.link.callback(move |_| msg.clone()) />
                { format!(" {}", text) }
            </label>
        }
    }
}

/// Returns true if text length is 9 and chars are 'r', 'w', 'x', or '-' at allowed positions
fn is_valid_text(text: &String) -> bool {
    if text.len() != 9 {
        return false;
    }

    let allowed = [ 'r', 'w', 'x' ];
    let mut a = allowed.into_iter();
    for char_index in text.char_indices() {
        let mut rwx = a.next();

        if rwx.is_none() {
            a = allowed.into_iter();
            rwx = a.next();
        }

        let (_, ch) = char_index; 
        if ch != rwx.unwrap() && ch != '-' { 
            return false;
        }
    }

    true
}