use ajazz::reports::rgb::Effect;
use tokio::sync::mpsc;

use crate::{
    app::page::Page,
    manager::{FromKeyboardTask, KeyboardId, ToKeyboardTask},
};

#[derive(Debug, Clone)]
pub enum AppMessage {
    Refresh,
    FromKeyboardTask(FromKeyboardTask),
    Tx(mpsc::Sender<ToKeyboardTask>),
    Navigate(Page),
    SetEffect(KeyboardId, Effect),
}
