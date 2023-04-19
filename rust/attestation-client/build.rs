use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let sgx_sdk_path = "/opt/intel/sgxsdk";
    // println!(r"cargo:rustc-link-search=.");
    println!(r"cargo:rustc-link-search=./lib/libs");
    println!("cargo:rustc-link-search=native={}/lib64", sgx_sdk_path);
    println!("cargo:include={}/include", sgx_sdk_path);
    println!("cargo:include=/usr/include");
    println!("cargo:rustc-link-lib=static=enclave_untrusted");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    // Tell cargo to tell rustc to link the system
    // sgx-dcap-quoteverify shared library.
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=sgx_dcap_quoteverify");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=bindings.h");

    // Set sdk to search path if SGX_SDK is in environment variable
    let mut sdk_inc = String::from("-I");
    if let Ok(val) = env::var("SGX_SDK") {
        sdk_inc.push_str(&val);
        sdk_inc.push_str("/include/");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("bindings.h")
        // Include search path
        .clang_arg(sdk_inc)
        // Convert C enum to Rust enum
        .rustified_enum("_quote3_error_t")
        .rustified_enum("_sgx_ql_request_policy")
        .rustified_enum("_sgx_ql_qv_result_t")
        .rustified_enum("sgx_qv_path_type_t")
        // Disable Debug trait for packed C structures
        .no_debug("_quote_t")
        .no_debug("_sgx_ql_auth_data_t")
        .no_debug("_sgx_ql_certification_data_t")
        .no_debug("_sgx_ql_ecdsa_sig_data_t")
        .no_debug("_sgx_quote3_t")
        .no_debug("_sgx_ql_att_key_id_param_t")
        // Enable Default trait
        .derive_default(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
