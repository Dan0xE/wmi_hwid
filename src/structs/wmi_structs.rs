#[allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct WMIProcess {
    pub(crate) DeviceID: String,
    pub(crate) Name: String,
    pub(crate) Manufacturer: String,
    pub(crate) Product: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Win32_VideoController {
    pub(crate) Name: String,
    pub(crate) AdapterRAM: u64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Win32_BaseBoard {
    pub(crate) Manufacturer: String,
    pub(crate) Product: String,
    pub(crate) SerialNumber: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Win32_DiskDrive {
    pub(crate) Model: String,
    pub(crate) SerialNumber: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Win32_BIOS {
    pub(crate) Manufacturer: String,
    pub(crate) Version: String,
    pub(crate) SerialNumber: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Win32_PhysicalMemory {
    pub(crate) Capacity: u64,
}
