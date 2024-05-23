pub mod manifest;
pub mod io_utils;
pub mod sync_utils;
pub struct HandleEvent<T> {
    pub event: fn(T)
}
pub struct CounterEvent {
    pub(crate) total: usize,
    pub(crate) success: usize,
}
impl CounterEvent {
    pub fn percent(&self) -> usize {
        (self.total * self.success) / 100
    }
    pub fn new(t: usize, s: usize) -> CounterEvent {
        CounterEvent {
            total: t,
            success: s
        }
    }
}
