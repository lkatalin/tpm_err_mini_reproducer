use anyhow::Context as anyhow_context;
use std::env;

use std::str::FromStr;

use tss_esapi::{
    abstraction::{ak, ek, DefaultKey},
    interface_types::algorithm::{AsymmetricAlgorithm, HashingAlgorithm, SignatureSchemeAlgorithm},
    Context, Tcti,
};

fn main() {
    println!("Hello, TPM world!");

    let tcti_path = match env::var("TCTI") {
        Ok(val) => val,
        Err(_) => {
            if std::path::Path::new("/dev/tpmrm0").exists() {
                "device:/dev/tpmrm0".to_string()
            } else {
                "device:/dev/tpm0".to_string()
            }
        }
    };

    let tcti = Tcti::from_str(&tcti_path)
        .context("Error parsing TCTI specification")
        .unwrap();
    let mut ctx = Context::new(tcti)
        .context("Error initializing TPM2 context")
        .unwrap();

    let hash_alg = HashingAlgorithm::Sha256;
    let sign_alg = SignatureSchemeAlgorithm::RsaSsa;
    let enc_alg = AsymmetricAlgorithm::Rsa;

    println!("Preparing to kill the TPM of those Intel Whitley, Wilson City 2S, Ice Lake machines...");

    println!("Preparing for ek::create_ek_object()");
    let ek_result = ek::create_ek_object(&mut ctx, enc_alg, DefaultKey).unwrap();
    println!("Preparing for ak::create_ak()");
    let _ak = ak::create_ak(&mut ctx, ek_result, hash_alg, sign_alg, None, DefaultKey);
    println!("Done!");
}
