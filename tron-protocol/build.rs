use std::io::Cursor;
use std::{env, fs, path::PathBuf};

use reqwest::blocking::get;
use zip::ZipArchive;

const JAVA_TRON_ZIP_URL: &str = "https://github.com/tronprotocol/java-tron/archive/master.zip";
const GOOGLE_APIS_ZIP_URL: &str = "https://github.com/googleapis/googleapis/archive/master.zip";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = run() {
        eprintln!("\nBuild script failed: {}\n", e);
        let mut source = e.source();
        while let Some(s) = source {
            eprintln!("  Caused by: {}", s);
            source = s.source();
        }
        return Err(e);
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = env::current_dir().unwrap();
    let protocol = root_dir.join("src/protocol.rs");
    if protocol.exists() {
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tron_temp_dir = out_dir.join("tron_temp_protos");
    let google_temp_dir = out_dir.join("google_temp_protos");

    download_and_extract_zip(JAVA_TRON_ZIP_URL, &tron_temp_dir, "java-tron-master")?;
    let tron_protos_dir = tron_temp_dir.join("java-tron-master/protocol/src/main/protos");
    if !tron_protos_dir.exists() {
        return Err(format!(
            "Tron protos source base directory {:?} not found.",
            tron_protos_dir
        )
        .into());
    }
    let core_source = tron_protos_dir.join("core");
    if !core_source.exists() {
        return Err(format!("Tron 'core' protos directory {:?} not found.", core_source).into());
    }
    let api_source = tron_protos_dir.join("api");
    if !api_source.exists() {
        return Err(format!("Tron 'api' protos directory {:?} not found.", api_source).into());
    }

    download_and_extract_zip(GOOGLE_APIS_ZIP_URL, &google_temp_dir, "googleapis-master")?;
    let google_protos_dir = google_temp_dir.join("googleapis-master");

    // --- Compile protos ---
    tonic_prost_build::configure()
        .out_dir(root_dir.join("src"))
        .build_server(false)
        .file_descriptor_set_path(out_dir.join("tron_protocol_descriptor.bin"))
        .type_attribute(
            "SmartContract.ABI",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .type_attribute(
            "SmartContract.ABI.Entry",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .type_attribute(
            "SmartContract.ABI.Entry.Param",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.anonymous",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.constant",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.payable",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.name",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.outputs",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.Param.indexed",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .field_attribute(
            "SmartContract.ABI.Entry.stateMutability",
            "#[cfg_attr(feature = \"serde\", serde(default))]",
        )
        .type_attribute(
            "AccountResourceMessage",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .enum_attribute(
            "Transaction.Result.contractResult",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .enum_attribute(
            "Transaction.Contract.ContractType",
            "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .compile_protos(
            &[tron_protos_dir.join("api/api.proto").to_str().unwrap()],
            &[
                tron_protos_dir.to_str().unwrap(),
                google_protos_dir.to_str().unwrap(),
            ],
        )?;

    // --- Cleanup ---
    let unused_file = root_dir.join("src/google.api.rs");
    if unused_file.exists() {
        fs::remove_file(unused_file)?;
    }
    if tron_temp_dir.exists() {
        fs::remove_dir_all(&tron_temp_dir)?;
    }
    if google_temp_dir.exists() {
        fs::remove_dir_all(&google_temp_dir)?;
    }

    Ok(())
}

fn download_and_extract_zip(
    url: &str,
    dest_dir: &PathBuf,
    expected_root: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if dest_dir.exists() {
        fs::remove_dir_all(dest_dir)?;
    }
    fs::create_dir_all(dest_dir)?;
    eprintln!("Created temporary directory: {:?}", dest_dir);

    eprintln!("Downloading zip from: {}", url);
    let response = get(url)?.bytes()?;
    let cursor = Cursor::new(response);
    let mut archive = ZipArchive::new(cursor)?;
    eprintln!("Zip archive opened successfully.");

    archive.extract(dest_dir)?;
    eprintln!("Zip archive extracted to: {:?}", dest_dir);

    let extracted_root = dest_dir.join(expected_root);
    if !extracted_root.exists() {
        let entries = fs::read_dir(dest_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;
        eprintln!("Contents of {:?}: {:?}", dest_dir, entries);
        return Err(format!(
            "Expected extracted root directory {:?} not found.",
            extracted_root
        )
        .into());
    }

    Ok(())
}
