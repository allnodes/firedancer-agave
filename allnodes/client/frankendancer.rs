use std::path::PathBuf;

pub fn get_client_version_config() -> super::ClientVersionConfig {
    println!("cargo:rerun-if-changed=frankendancer.rs");

    super::ClientVersionConfig {
        client_name: "frankendancer",
        is_submodule: true,
        fallback_version_getter: Some(Box::new(fallback_get_version)),
    }
}

fn fallback_get_version(_git_root: &PathBuf) -> String {
    let major = std::env::var("FIREDANCER_VERSION_MAJOR");
    let minor = std::env::var("FIREDANCER_VERSION_MINOR");
    let patch = std::env::var("FIREDANCER_VERSION_PATCH");
    match (&major, &minor, &patch) {
        (Ok(major), Ok(minor), Ok(patch)) => format!("{major}.{minor}.{patch}"),
        _ => panic!("Failed to get Firedancer version: [{major:?}, {minor:?}, {patch:?}]"),
    }
}
