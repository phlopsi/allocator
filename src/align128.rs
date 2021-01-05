#[derive(Debug)]
#[repr(align(128))]
pub struct Align128<T>(pub T);

impl<T> std::ops::Deref for Align128<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
