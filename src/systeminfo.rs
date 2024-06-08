use sysinfo::System;
use std::process::Command; 

pub fn system_information(){
    println!("this will print the system information of the machine it is running in");
    println!("--------------------------");
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("system information");
    println!("total memory: {:.2} GB", memory_bytes_to_gb(sys.total_memory()));
    println!("used memory: {:.2} GB", memory_bytes_to_gb(sys.used_memory())); 
    println!("total swap: {:.2} GB", memory_bytes_to_gb(sys.total_swap()));
    println!("used swap: {:.2} GB", memory_bytes_to_gb(sys.used_swap()));

    println!("System Name: {:?} GB", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

}

pub fn rust_information(){
    println!("this will print the rust information");
    let rust_details = Command::new("rustc").arg("--version").output().expect("Failed to execute rustc");
    let rust_version = String::from_utf8_lossy(&rust_details.stdout);
    println!("Rust version: {}", rust_version);
}
fn memory_bytes_to_gb(storage_size: u64) -> f64{
    let storage_in_gb = storage_size as f64 / 1_073_741_824.0;
    storage_in_gb
}
