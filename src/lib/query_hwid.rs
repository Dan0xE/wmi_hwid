#[allow(non_snake_case)]
use crate::structs::wmi_structs::*;
use machineid_rs::HWIDComponent;
use machineid_rs::{Encryption, IdBuilder};
use std::collections::HashMap;
use sysinfo::{CpuExt, System, SystemExt};
use wmi::{COMLibrary, WMIConnection};

/**
* ### This function queries the hardware id of the system and returns a string with the information
  ### ``` Key: The key that is used to build an unique id using the machineid_rs crate ```
* ### The returned string contains the following information:
* ++++```
* BIOSManufacturer: Manufacturer of the BIOS
* OS: Operating System
* RAM: Amount of RAM in bytes
* BIOSSerial: Serial number of the BIOS
* CPU: CPU Brand and Model
* HOSTNAME: Hostname of the system
* Motherboard: Motherboard Model
* OSVersion: Operating System Version
* OSBuild: Operating System Build
* GPU: GPU Brand and Model
* BIOS: BIOS Version
* DISK: Disk Model
* MANUFACTURER: Manufacturer of the system
* ID: Unique ID of the system
* DISKSERIAL: Serial number of the disk
* SERIALNUMBER: Serial number of the motherboard
* FREQUENCY: CPU Frequency
* VRAM: VRAM in bytes
*
* ## Erros that can be returned:
* "An Error occured, this might be because you called the function twice, but wmi cannot initialize twice."
           "Also, calling this from WRY will not work, try calling it before creating an event loop."
           "For more Information and incase you want to lookup the HINSTANCE Error code, visit: https://docs.microsoft.com/en-us/windows/win32/com/com-error-codes "
           "Common Error codes are:"
           "-2147221164: RPC_E_CHANGED_MODE - This error occurs when you call CoInitializeEx from a thread that is already initialized with a different concurrency model."
           "-2147221163: RPC_E_CANTCALLOUT_ININPUTSYNCCALL - This error occurs when you call an outgoing call while inside an input-synchronous call."
           "-2147417831 RPC_E_TOO_LATE - This error occurs when you call CoInitializeEx after the COM library has already been initialized."
           "RPC_E_NO_GOOD_SECURITY_PACKAGES - This error occurs when you call CoInitializeSecurity without specifying authentication services."
           "RPC_E_WRONG_THREAD - This error occurs when you call CoInitializeEx from a thread that is already initialized."
* ++++```
*/
pub(crate) fn query_hwid(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut component_layout = HashMap::new();

    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();

    for gpus in wmi_con
        .raw_query("SELECT * FROM Win32_VideoController")
        .unwrap()
    {
        let gpus: Win32_VideoController = gpus;

        component_layout.insert("GPU", gpus.Name);
        component_layout.insert("VRAM", gpus.AdapterRAM.to_string());

        break;
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

    let mut total_ram = 0;
    for ram in wmi_con
        .raw_query("SELECT * FROM Win32_PhysicalMemory")
        .unwrap()
    {
        let ram: Win32_PhysicalMemory = ram;
        total_ram += ram.Capacity;
    }

    component_layout.insert("RAM", total_ram.to_string());

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
    let id = builder.build(&key);

    component_layout.insert("ID", id.unwrap().to_string());

    let mut component_layout_string = String::new();
    for (key, value) in &component_layout {
        component_layout_string.push_str(&format!("{}: {}\n", key, value));
    }
    // println!("{:#?}", component_layout);

    return Ok(component_layout_string);
}
