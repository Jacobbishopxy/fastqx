//! file: build.rs
//! author: Jacob Xie
//! date: 2023/09/13 21:07:03 Wednesday
//! brief:
//!
//! https://pyo3.rs/main/building_and_distribution

fn main() {
    pyo3_build_config::add_extension_module_link_args();

    pyo3_build_config::use_pyo3_cfgs();
}
