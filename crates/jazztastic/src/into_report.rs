use std::time::Duration;

use crate::keyboards::KeyboardKind;

#[derive(Debug, Clone)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bytes {
    SixtyFive,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Read(Bytes),
    Write([u8; 65]),
    Delay(Duration),
}

impl Instruction {
    pub fn execute<K: crate::keyboards::Keyboard>(
        &self,
        keyboard: &mut K,
    ) -> Result<(), crate::hidapi::HidError> {
        match self {
            Instruction::Read(bytes) => {
                match bytes {
                    Bytes::SixtyFive => {
                        let mut buf = [0u8; 65];
                        keyboard.read(&mut buf)?;
                    }
                }
                Ok(())
            }

            Instruction::Write(data) => {
                keyboard.write(data)?;
                Ok(())
            }

            Instruction::Delay(duration) => {
                std::thread::sleep(*duration);
                Ok(())
            }
        }
    }
}

pub trait IntoReport {
    fn report(&self, keyboard_kind: KeyboardKind) -> OneOrMany<Instruction>;
}

impl IntoReport for &dyn IntoReport {
    fn report(&self, keyboard_kind: KeyboardKind) -> OneOrMany<Instruction> {
        (**self).report(keyboard_kind)
    }
}
