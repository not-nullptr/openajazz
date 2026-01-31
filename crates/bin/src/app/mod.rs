mod message;
mod page;

use std::collections::HashMap;

use crate::{
    app::{message::AppMessage, page::Page},
    manager::{FromKeyboardTask, KeyboardId, KeyboardManager, ToKeyboardTask, task::KeyboardTask},
};
use ajazz::{hidapi::HidApi, reports::rgb::Rgb};
use iced::{Element, Subscription, Task, futures::SinkExt, stream};
use tokio::sync::mpsc;

pub struct App {
    tx: Option<mpsc::Sender<ToKeyboardTask>>,
    keyboards: HashMap<KeyboardId, KeyboardInfo>,
    page: Page,
}

#[derive(Debug, Clone)]
pub struct KeyboardInfo {
    pub name: String,
    pub rgb: Rgb,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        let mut s = Self {
            tx: None,
            keyboards: HashMap::new(),
            page: Page::Home,
        };

        let task = s.boot();
        (s, task)
    }

    fn boot(&mut self) -> Task<AppMessage> {
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        Subscription::run(|| {
            stream::channel(100, async |mut sender| {
                let api = HidApi::new().expect("failed to create HidApi");
                let (tx, rx) = KeyboardTask::spawn(api);
                sender.send(AppMessage::Tx(tx.clone())).await.ok();
                let mut manager = KeyboardManager::from_channels(tx.clone(), rx);

                while let Some(message) = manager.recv().await {
                    if sender
                        .send(AppMessage::FromKeyboardTask(message))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            })
        })
    }

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::FromKeyboardTask(k) => match k {
                FromKeyboardTask::Connected(id, name) => {
                    self.keyboards.insert(
                        id,
                        KeyboardInfo {
                            name,
                            rgb: Rgb::builder().build(),
                        },
                    );
                }

                FromKeyboardTask::Disconnected(id) => {
                    self.keyboards.remove(&id);
                }
            },

            AppMessage::Tx(tx) => {
                self.tx = Some(tx);
            }

            AppMessage::Navigate(page) => {
                self.page = page;
            }

            AppMessage::SetEffect(id, effect) => {
                let keyboard = self.keyboards.get_mut(&id);
                if let Some(keyboard) = keyboard {
                    keyboard.rgb.effect = effect;
                }

                self.update_rgb(id);
            }

            AppMessage::Refresh => {}
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, AppMessage> {
        match self.page {
            Page::Home => page::home::view(self).into(),
            Page::Keyboard(ref id) => page::keyboard::view(self, id).into(),
        }
    }

    fn update_rgb(&self, id: KeyboardId) {
        if let Some(tx) = &self.tx
            && let Some(keyboard) = self.keyboards.get(&id)
        {
            let rgb = keyboard.rgb;
            let tx = tx.clone();
            tokio::spawn(async move {
                if let Err(e) = tx.send(ToKeyboardTask::SetRgb(id, rgb)).await {
                    log::error!("failed to send SetRgb to keyboard task: {}", e);
                }
            });
        }
    }
}
