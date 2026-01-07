use {crate::validator::ValidatorConfig, solana_turbine::xdp::XdpConfig, std::path::Path};

pub fn init(
    ledger_path: &Path,
    validator_config: &mut ValidatorConfig,
    enable_xdp: bool,
    xdp_interface: Option<&str>,
    xdp_zero_copy: bool,
) {
    let mut allnodes_client_store_paths = vec![ledger_path.to_path_buf()];
    if let Some(path) = &validator_config.identity_path {
        if let Some(dir) = path.parent() {
            allnodes_client_store_paths.insert(0, dir.to_path_buf());
        }
    }

    allnodes_client::CONSTANTS.load(allnodes_client_store_paths.clone());

    let expected_shred_version = validator_config
        .expected_shred_version
        .expect("expected_shred_version should not be None");

    allnodes_client::resolve_endpoints(expected_shred_version);

    let mut xdp_recommended_vcore_id = 0;

    if let Some((_, cores)) = allnodes_solana::poh::process_core_config() {
        for i in 0..cores.len() / 2 {
            if cores[i]
                .vcore_ids
                .contains(&(validator_config.poh_pinned_cpu_core as u32))
            {
                xdp_recommended_vcore_id =
                    cores[cores.len().saturating_sub(1)].vcore_ids[0] as usize;
                break;
            }
        }
    }

    if validator_config.retransmit_xdp.is_none()
        && (enable_xdp || xdp_interface.is_some() || xdp_zero_copy)
    {
        validator_config.retransmit_xdp = Some(XdpConfig::new(
            xdp_interface,
            vec![xdp_recommended_vcore_id],
            xdp_zero_copy,
        ));
    }
}
