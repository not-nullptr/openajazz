use crate::app::page::KeyboardPage;
use crate::app::{App, message::AppMessage, page::Page};
use iced::Element;
use iced::widget::*;

pub fn view<'a>(app: &'a App, page: &'a KeyboardPage) -> impl Into<Element<'a, AppMessage>> {
    let id = page.id;
    let Some(keyboard) = app.keyboards.get(&id) else {
        // immediately navigate back to home if the keyboard is gone
        return Column::from_iter([button(text("<"))
            .on_press(AppMessage::Navigate(Page::Home))
            .into()]);
    };

    Column::from_iter([
        Row::from_iter([
            button(text("<"))
                .on_press(AppMessage::Navigate(Page::Home))
                .into(),
            text(&keyboard.name).size(24).into(),
        ])
        .spacing(10)
        .into(),
        combo_box(
            &page.effect_state,
            "Effect",
            Some(&keyboard.rgb.effect),
            move |effect| AppMessage::SetEffect(id, effect),
        )
        .into(),
    ])
    .padding(20)
    .spacing(10)
}
