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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_format() {
        let ctx = DeviceContext::new(0, "device");
        assert_eq!(ctx.device_id, "device-0000");
        assert_eq!(ctx.index, 0);
    }

    #[test]
    fn test_device_id_padding() {
        let ctx = DeviceContext::new(42, "sensor");
        assert_eq!(ctx.device_id, "sensor-0042");
        assert_eq!(ctx.index, 42);
    }

    #[test]
    fn test_device_id_large_index() {
        let ctx = DeviceContext::new(9999, "node");
        assert_eq!(ctx.device_id, "node-9999");
    }

    #[test]
    fn test_device_id_very_large_index() {
        let ctx = DeviceContext::new(10000, "dev");
        assert_eq!(ctx.device_id, "dev-10000");
    }

    #[test]
    fn test_custom_prefix() {
        let ctx = DeviceContext::new(1, "factory-machine");
        assert_eq!(ctx.device_id, "factory-machine-0001");
    }
}
