use machineid_rs::HWIDComponent;
use machineid_rs::{Encryption, IdBuilder};
use serde::Deserialize;
use std::collections::HashMap;
use sysinfo::{CpuExt, System, SystemExt};
use wmi::{COMLibrary, WMIConnection};

#[allow(non_snake_case)]

pub(crate) fn query_hwid() {
    #[derive(Deserialize, Debug)]
    struct WMIProcess {
        DeviceID: String,
        Name: String,
        Manufacturer: String,
        Product: String,
    }

    let mut system = System::new_all();
    system.refresh_all();

    let mut component_layout = HashMap::new();

    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();

    #[derive(Deserialize, Debug)]
    struct Win32_VideoController {
        Name: String,
        AdapterRAM: u64,
    }

    for gpus in wmi_con
        .raw_query("SELECT * FROM Win32_VideoController")
        .unwrap()
    {
        let gpus: Win32_VideoController = gpus;

        component_layout.insert("GPU", gpus.Name);
        component_layout.insert("VRAM", gpus.AdapterRAM.to_string());

        break;
    }

    //get motherboard info with the wmi crate
    #[derive(Deserialize, Debug)]
    struct Win32_BaseBoard {
        Manufacturer: String,
        Product: String,
        SerialNumber: String,
    }

    for motherboard in wmi_con.raw_query("SELECT * FROM Win32_BaseBoard").unwrap() {
        let motherboard: Win32_BaseBoard = motherboard;

        component_layout.insert("Motherboard", motherboard.Product);
        component_layout.insert("Manufacturer", motherboard.Manufacturer);
        component_layout.insert("SerialNumber", motherboard.SerialNumber);

        break;
    }

    for cpu in system.cpus() {
        component_layout.insert("CPU", cpu.brand().to_string());
        component_layout.insert("Frequency", cpu.frequency().to_string());

        break;
    }

    //get ram info with the wmi crate
    #[derive(Deserialize, Debug)]
    struct Win32_PhysicalMemory {
        Capacity: u64,
    }

    let mut total_ram = 0;
    for ram in wmi_con
        .raw_query("SELECT * FROM Win32_PhysicalMemory")
        .unwrap()
    {
        let ram: Win32_PhysicalMemory = ram;
        total_ram += ram.Capacity;
    }

    component_layout.insert("RAM", total_ram.to_string());

    //get disk info with the wmi crate
    #[derive(Deserialize, Debug)]
    struct Win32_DiskDrive {
        Model: String,
        SerialNumber: String,
    }

    for disk in wmi_con.raw_query("SELECT * FROM Win32_DiskDrive").unwrap() {
        let disk: Win32_DiskDrive = disk;

        component_layout.insert("Disk", disk.Model);
        component_layout.insert("DiskSerial", disk.SerialNumber);

        break;
    }

    component_layout.insert("OS", system.name().unwrap_or_else(|| "unknown".to_string()));
    component_layout.insert(
        "OSVersion",
        system
            .kernel_version()
            .unwrap_or_else(|| "unknown".to_string()),
    );
    component_layout.insert(
        "OSBuild",
        system.os_version().unwrap_or_else(|| "unknown".to_string()),
    );
    component_layout.insert(
        "Hostname",
        system.host_name().unwrap_or_else(|| "unknown".to_string()),
    );

    //get bios info with the wmi crate
    #[derive(Deserialize, Debug)]
    struct Win32_BIOS {
        Manufacturer: String,
        Version: String,
        SerialNumber: String,
    }

    for bios in wmi_con.raw_query("SELECT * FROM Win32_BIOS").unwrap() {
        let bios: Win32_BIOS = bios;

        component_layout.insert("BIOS", bios.Version);
        component_layout.insert("BIOSManufacturer", bios.Manufacturer);
        component_layout.insert("BIOSSerial", bios.SerialNumber);

        break;
    }

    //here we are creating an unique id
    let mut builder = IdBuilder::new(Encryption::SHA256);
    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores);

    //we get the unique id
    let id = builder.build("yourkeyhere");

    component_layout.insert("ID", id.unwrap().to_string());

    let mut component_layout_string = String::new();
    for (key, value) in &component_layout {
        component_layout_string.push_str(&format!("{}: {}\n", key, value));
    }
    println!("{:#?}", component_layout);
}