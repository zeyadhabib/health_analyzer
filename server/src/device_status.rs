use crate::status::{StatusResponse, SpecsResponse};
use sysinfo::{System, Disks};

pub fn get_status() -> StatusResponse {
    let mut system = System::new_all();
    system.refresh_all();

    let used_ram = system.used_memory();
    let used_disk = Disks::new_with_refreshed_list().iter().map(
        |disk| disk.total_space() - disk.available_space()).sum::<u64>();
    let used_cpu = system.cpus().iter().map(
        |cpu| cpu.cpu_usage() as f64).collect::<Vec<f64>>();

    StatusResponse {
        used_ram,
        used_disk,
        used_cpu,
    }
}


pub fn get_specs() -> SpecsResponse {
    let mut system = System::new_all();
    system.refresh_all();

    let total_ram = system.total_memory();
    let total_disk = Disks::new_with_refreshed_list().iter().map(
        |disk| disk.total_space()).sum::<u64>();
    let total_cpu = system.cpus().iter().map(
        |cpu| cpu.frequency() as f64).collect::<Vec<f64>>();

    SpecsResponse {
        total_ram,
        total_disk,
        total_cpu,
    }
}