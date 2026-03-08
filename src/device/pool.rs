use crate::device::context::DeviceContext;

pub struct DevicePool {
    devices: Vec<DeviceContext>,
    cursor: usize,
}

impl DevicePool {
    pub fn new(count: usize, id_prefix: &str) -> Self {
        let devices = (0..count)
            .map(|i| DeviceContext::new(i, id_prefix))
            .collect();
        Self { devices, cursor: 0 }
    }

    /// Return the next device in round-robin order.
    pub fn next(&mut self) -> &DeviceContext {
        let device = &self.devices[self.cursor % self.devices.len()];
        self.cursor += 1;
        device
    }

    #[allow(dead_code)]
    pub fn all(&self) -> &[DeviceContext] {
        &self.devices
    }
}
