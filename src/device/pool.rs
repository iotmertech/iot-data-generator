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

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn all(&self) -> &[DeviceContext] {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_creates_correct_count() {
        let pool = DevicePool::new(5, "device");
        assert_eq!(pool.all().len(), 5);
    }

    #[test]
    fn test_pool_device_ids() {
        let pool = DevicePool::new(3, "sensor");
        let ids: Vec<&str> = pool.all().iter().map(|d| d.device_id.as_str()).collect();
        assert_eq!(ids, vec!["sensor-0000", "sensor-0001", "sensor-0002"]);
    }

    #[test]
    fn test_pool_round_robin() {
        let mut pool = DevicePool::new(3, "dev");
        assert_eq!(pool.next().device_id, "dev-0000");
        assert_eq!(pool.next().device_id, "dev-0001");
        assert_eq!(pool.next().device_id, "dev-0002");
        // wraps around
        assert_eq!(pool.next().device_id, "dev-0000");
        assert_eq!(pool.next().device_id, "dev-0001");
    }

    #[test]
    fn test_pool_single_device_always_returns_same() {
        let mut pool = DevicePool::new(1, "solo");
        for _ in 0..5 {
            assert_eq!(pool.next().device_id, "solo-0000");
        }
    }
}
