pub trait IntoReport {
    fn write_into(&self, buf: &mut [u8; 63]);
}
