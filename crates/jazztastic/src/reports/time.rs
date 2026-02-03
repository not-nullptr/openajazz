use chrono::{DateTime, Local};
use hidapi::HidError;

use crate::{
    into_report::WriteInto,
    keyboards::{
        DynKeyboard, Keyboard,
        ak35i::Ak35i,
        f75_max::{F75CommunicationGuard, F75DataMessage, F75Max},
    },
};

pub struct TimeSync {
    pub date_time: DateTime<Local>,
}

impl WriteInto for TimeSync {
    fn write_into<K: DynKeyboard + ?Sized>(&self, keyboard: &mut K) -> Result<(), HidError> {
        let kind = keyboard.keyboard_kind_dyn();

        match kind {
            k if k == Ak35i::keyboard_kind() || k == F75Max::keyboard_kind() => {
                let mut guard = F75CommunicationGuard::new(keyboard)?;
                guard.send(F75DataMessage::TimeSync(self))?;
            }

            _ => unimplemented!("no implementation available for time sync for your keyboard"),
        }

        Ok(())
    }
}
