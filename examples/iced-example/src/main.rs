use core::fmt;

use iced::{
    executor, widget::{Column, PickList, Text}, Alignment, Application, Command, Element, Length, Settings
};


// we generate the Lang enum in this module
mod translations {
    rosetta_i18n::include_translations!();
}


use rosetta_i18n::Language;
use translations::Lang;

struct AppState {
    available_languages: Vec<Lang>,
    lang: Lang, // the language selected
}

#[derive(Debug, Clone)]
enum Message {
    SelectLanguage(Lang)
}

impl iced::Application for AppState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {

        let app_state = Self {
            available_languages: vec![Lang::En, Lang::Fr],
            lang: Lang::fallback()
        };

        (app_state, Command::none())
    }

    fn title(&self) -> String {
        String::from(self.lang.hello())
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {

        match message {
            Message::SelectLanguage(language) => {
                self.lang = language;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme, iced::Renderer> {

        let pick_list = PickList::new(
            self.available_languages.clone(),
            Some(self.lang.clone()),
            |language| Message::SelectLanguage(language),
        );

        // we obtain the string defined in the json files here
        let selected_language_string: String = self.lang.selected_language(self.lang);

        Column::new()
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(Text::new(selected_language_string))
            .push(pick_list)
            .into()
    }
}


impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::Fr => write!(f, "Fr"),
            Lang::En =>  write!(f, "En"),
        }
    }
}

pub fn main() -> iced::Result {
    AppState::run(Settings::default())
}
