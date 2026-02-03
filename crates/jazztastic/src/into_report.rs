use crate::keyboards::DynKeyboard;
use hidapi::HidError;

pub trait WriteInto {
    fn write_into<K: DynKeyboard + ?Sized>(&self, keyboard: &mut K) -> Result<(), HidError>;
}

pub trait DynWriteInto {
    fn write_into_dyn(&self, keyboard: &mut dyn DynKeyboard) -> Result<(), HidError>;
}

impl<W: WriteInto> DynWriteInto for W {
    fn write_into_dyn(&self, keyboard: &mut dyn DynKeyboard) -> Result<(), hidapi::HidError> {
        self.write_into(keyboard)
    }
}
