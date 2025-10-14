// export_test_data.rs - 將測試電路導出為 zkenc-core 可用的格式
//
// 運行: cargo test -p zkenc-cli --test export_test_data -- --nocapture --ignored
//
// 這個測試會將 signature.r1cs 轉換為 SerializableCircuit 並儲存到
// zkenc-core/tests/fixtures/ 目錄

use std::collections::HashMap;
use std::path::PathBuf;
use zkenc_cli::r1cs::R1csFile;
use zkenc_cli::serializable::{SerializableTestCase, SerializableWitness};

#[test]
#[ignore] // 只在需要時手動運行
fn export_signature_circuit() {
    // 1. 載入 R1CS
    let mut r1cs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    r1cs_path.push("tests/r1cs/signature.r1cs");

    println!("📂 Loading R1CS from: {:?}", r1cs_path);
    let r1cs = R1csFile::from_file(&r1cs_path).expect("Failed to parse R1CS");

    println!("✅ Loaded circuit:");
    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Wires: {}", r1cs.n_wires);
    println!("   - Public inputs: {}", r1cs.n_pub_in);

    // 2. 轉換為 SerializableCircuit
    let circuit = r1cs.to_serializable();

    // 3. 創建示例見證 (簡單的測試值)
    // 注意: 這些不是真正的有效見證,只是為了測試結構
    let mut assignments = HashMap::new();

    // Wire 0 永遠是常數 1
    let one_bytes = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    assignments.insert(0, one_bytes.clone());

    // 公開輸入 (wire 1..=n_pub_in)
    for i in 1..=r1cs.n_pub_in {
        // 使用簡單的測試值: i
        let mut bytes = vec![0u8; 32];
        bytes[0] = (i % 256) as u8;
        assignments.insert(i, bytes);
    }

    let witness = SerializableWitness { assignments };

    // 4. 創建測試案例
    let test_case = SerializableTestCase {
        circuit,
        witness,
        name: "signature_circuit".to_string(),
        description: Some(
            "EdDSA signature verification circuit from Circom. \
             This is a test fixture with dummy witness values."
                .to_string(),
        ),
    };

    // 5. 導出到 zkenc-core
    let mut output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output_path.pop(); // packages/zkenc-cli -> packages
    output_path.push("zkenc-core/tests/fixtures");

    // 創建目錄 (如果不存在)
    std::fs::create_dir_all(&output_path).expect("Failed to create fixtures directory");

    // 儲存為 JSON (方便閱讀和除錯)
    output_path.push("signature_circuit.json");
    println!("\n💾 Exporting to: {:?}", output_path);
    test_case
        .save_json(output_path.to_str().unwrap())
        .expect("Failed to save JSON");

    let json_size = std::fs::metadata(&output_path).unwrap().len();
    println!("✅ JSON exported: {} MB", json_size / 1_000_000);

    // 也儲存為 bincode (更緊湊)
    output_path.pop();
    output_path.push("signature_circuit.bin");

    let bincode_bytes = bincode::serialize(&test_case).expect("Failed to serialize to bincode");
    std::fs::write(&output_path, &bincode_bytes).expect("Failed to write bincode");

    let bin_size = bincode_bytes.len();
    println!("✅ Bincode exported: {} MB", bin_size / 1_000_000);
    println!(
        "   (Compression ratio: {:.1}%)",
        (bin_size as f64 / json_size as f64) * 100.0
    );

    println!("\n🎉 Test data exported successfully!");
    println!("   You can now use these files in zkenc-core tests");
}

#[test]
#[ignore]
fn export_sudoku_circuit() {
    // 同樣的邏輯,但針對 sudoku 電路
    let mut r1cs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    r1cs_path.push("tests/r1cs/sudoku.r1cs");

    if !r1cs_path.exists() {
        println!("⚠️  Sudoku circuit not found, skipping");
        return;
    }

    println!("📂 Loading R1CS from: {:?}", r1cs_path);
    let r1cs = R1csFile::from_file(&r1cs_path).expect("Failed to parse R1CS");

    println!("✅ Loaded circuit:");
    println!("   - Constraints: {}", r1cs.n_constraints);
    println!("   - Wires: {}", r1cs.n_wires);
    println!("   - Public inputs: {}", r1cs.n_pub_in);

    let circuit = r1cs.to_serializable();

    // Load witness from snarkjs-generated .wtns file
    println!("\n📥 Loading witness from .wtns file...");
    let mut wtns_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    wtns_path.push("tests/inputs/sudoku_sudoku_basic.wtns");

    let witness_file = zkenc_cli::witness::WitnessFile::from_file(wtns_path.to_str().unwrap())
        .expect("Failed to load witness file");

    println!("✅ Loaded witness with {} wires", witness_file.n_wires());

    // Use the witness assignments directly (already in the right format)
    let witness = SerializableWitness {
        assignments: witness_file.assignments,
    };
    let test_case = SerializableTestCase {
        circuit,
        witness,
        name: "sudoku_circuit".to_string(),
        description: Some("Sudoku puzzle verification circuit from Circom".to_string()),
    };

    let mut output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    output_path.pop();
    output_path.push("zkenc-core/tests/fixtures");
    std::fs::create_dir_all(&output_path).expect("Failed to create fixtures directory");

    output_path.push("sudoku_circuit.json");
    println!("\n💾 Exporting to: {:?}", output_path);
    test_case
        .save_json(output_path.to_str().unwrap())
        .expect("Failed to save JSON");

    let json_size = std::fs::metadata(&output_path).unwrap().len();
    println!("✅ JSON exported: {} KB", json_size / 1_000);

    // 也儲存為 bincode (更緊湊)
    output_path.pop();
    output_path.push("sudoku_circuit.bin");

    let bincode_bytes = bincode::serialize(&test_case).expect("Failed to serialize to bincode");
    std::fs::write(&output_path, &bincode_bytes).expect("Failed to write bincode");

    let bin_size = bincode_bytes.len();
    println!("✅ Bincode exported: {} KB", bin_size / 1_000);
    println!(
        "   (Compression ratio: {:.1}%)",
        (bin_size as f64 / json_size as f64) * 100.0
    );

    println!("\n🎉 Sudoku circuit exported successfully!");
}
