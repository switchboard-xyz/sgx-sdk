use crate::error::Err;
use getrandom::getrandom;

use sha2::{Digest, Sha256};
use std::fs;

// const MAX_QUOTE_SIZE: usize = 1024 * 8; // DCAP quotes are typically around 4KB => overallocate

pub struct Sgx {}

impl Sgx {
    pub fn gramine_generate_quote(user_data: &[u8]) -> std::result::Result<Vec<u8>, Err> {
        match fs::metadata("/dev/attestation/quote") {
            Ok(_) => (),
            Err(_) => return Err(Err::SgxError),
        }
        let mut hasher = Sha256::new();
        hasher.update(user_data);
        let hash_result = &hasher.finalize()[..32];

        let mut data = [0u8; 64];
        data[..32].copy_from_slice(hash_result);

        let user_report_data_path = "/dev/attestation/user_report_data";
        if fs::write(user_report_data_path, &data[..]).is_err() {
            return Err(Err::SgxWriteError);
        }

        fs::read("/dev/attestation/quote").map_err(|_| Err::SgxError)
    }

    pub fn read_rand(buf: &mut [u8]) -> std::result::Result<(), Err> {
        getrandom(buf).map_err(|_| Err::SgxError)
    }
}
