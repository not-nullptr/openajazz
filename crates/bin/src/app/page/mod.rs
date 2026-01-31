pub mod home;
pub mod keyboard;

use ajazz::reports::rgb::Effect;
use iced::widget::combo_box;

use crate::manager::KeyboardId;

#[derive(Debug, Clone)]
pub enum Page {
    Home,
    Keyboard(KeyboardPage),
}

#[derive(Debug, Clone)]
pub struct KeyboardPage {
    pub id: KeyboardId,
    pub effect_state: combo_box::State<Effect>,
}
