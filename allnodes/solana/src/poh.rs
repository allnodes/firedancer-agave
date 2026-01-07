use allnodes_service_protos::CoreConfig;

pub fn process_core_config() -> Option<(u64, Vec<CoreConfig>)> {
    let cpu_info = read_cpu_info()?;

    allnodes_client::poh_process_core_config(&cpu_info)
}

fn read_cpu_info() -> Option<String> {
    std::fs::read_to_string("/proc/cpuinfo").ok()
}
