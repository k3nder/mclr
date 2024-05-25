pub mod manifest;
pub mod io_utils;
pub mod sync_utils;
pub struct HandleEvent<T> {
    event: fn(T)
}

impl<T> HandleEvent<T> {
    pub fn new(event: fn(T)) -> Self {
        HandleEvent {
            event
        }
    }
    pub fn event(&self, v: T) {
        (self.event)(v)
    }
}
pub struct CounterEvent {
    pub(crate) total: usize,
    pub(crate) success: usize,
}
impl CounterEvent {
    pub fn percent(&self) -> usize {
        (self.success * 100) / self.total
    }
    pub fn new(t: usize, s: usize) -> CounterEvent {
        CounterEvent {
            total: t,
            success: s
        }
    }
}
