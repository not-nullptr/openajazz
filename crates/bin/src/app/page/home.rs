use crate::app::page::KeyboardPage;
use crate::app::{App, message::AppMessage, page::Page};
use ajazz::reports::rgb::Effect;
use ajazz::strum::IntoEnumIterator;
use iced::Element;
use iced::widget::*;

pub fn view<'a>(app: &'a App) -> impl Into<Element<'a, AppMessage>> {
    Column::from_iter(app.keyboards.iter().map(|(id, info)| {
        button(text(&info.name))
            .on_press(AppMessage::Navigate(Page::Keyboard(KeyboardPage {
                id: *id,
                effect_state: combo_box::State::new(Effect::iter().collect()),
            })))
            .into()
    }))
    .padding(20)
    .spacing(10)
}
