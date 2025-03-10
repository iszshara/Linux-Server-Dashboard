use sysinfo::{
    Components, Disks, Networks, System,
};

fn kib_to_gib(kib: u64) -> f64 {
    kib as f64 / 1_073_741_824.0
}

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    let total_memory = sys.total_memory();
    let total_memory_gib = kib_to_gib(total_memory);

    let used_memory = sys.used_memory();
    let used_memory_gib = kib_to_gib(used_memory);

    let total_swap = sys.total_swap();
    let total_swap_gib = kib_to_gib(total_swap);

    let used_swap = sys.used_swap();
    let used_swap_gib = kib_to_gib(used_swap);



    println!("=> system:");
    //Ram and Swap Info
    println!("total memory: {:.2} GiB", total_memory_gib);
    println!("used  memory: {:.2} GiB", used_memory_gib);
    println!("total swap  : {:.2} GiB", total_swap_gib);
    println!("used  swap  : {:.2} GiB", used_swap_gib);

    //Display system name
    println!("System name:              {:?}", System::name());
    println!("System kernel version:    {:?}", System::kernel_version());
    println!("System OS version:        {:?}", System::os_version());
    println!("System host name:         {:?}", System::host_name());

    //Number of CPUs
    println!("NB CPUs: {}", sys.cpus().len());

    //Display processes ID
    // for (pid, process) in sys.processes() {
    //     println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    // }

    //all disks information
    // println!("=> disks:");
    // let disks = Disks::new_with_refreshed_list();
    // for disk in &disks {
    //     println!("{disk:?}");
    // }

    //Network Interfaces name, total data received and total data transmitted
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
    }

}
