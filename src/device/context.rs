#[derive(Debug, Clone)]
pub struct DeviceContext {
    pub device_id: String,
    pub index: usize,
}

impl DeviceContext {
    pub fn new(index: usize, id_prefix: &str) -> Self {
        Self {
            device_id: format!("{}-{:04}", id_prefix, index),
            index,
        }
    }
}
