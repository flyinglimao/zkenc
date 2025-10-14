// serializable.rs - 可序列化的電路和見證格式
// 用於在 zkenc-cli 和 zkenc-core 之間傳遞測試數據

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 線性組合中的單項 (變數索引, 係數的位元組表示)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableFactor {
    /// 變數的索引 (wire ID)
    pub wire_id: u32,
    /// Field element 的小端序位元組表示
    pub coefficient_bytes: Vec<u8>,
}

/// 線性組合 = Σ(係數 × 變數)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLC {
    pub factors: Vec<SerializableFactor>,
}

/// 約束 A × B = C
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableConstraint {
    pub a: SerializableLC,
    pub b: SerializableLC,
    pub c: SerializableLC,
}

/// 完整的電路定義 - 對應 R1CS 格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableCircuit {
    /// Field 的位元組大小 (通常是 32 for BLS12-381)
    pub field_size: u32,
    
    /// Prime field 的模數 (小端序位元組)
    pub prime_bytes: Vec<u8>,
    
    /// 總變數數量 (包含常數 1)
    pub n_wires: u32,
    
    /// 公開輸出數量
    pub n_pub_out: u32,
    
    /// 公開輸入數量
    pub n_pub_in: u32,
    
    /// 私有輸入數量
    pub n_prv_in: u32,
    
    /// 約束數量
    pub n_constraints: u32,
    
    /// 所有約束
    pub constraints: Vec<SerializableConstraint>,
    
    /// 變數標籤 (可選,用於除錯)
    pub wire_labels: Option<HashMap<u32, String>>,
}

/// 見證數據 - 所有變數的賦值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableWitness {
    /// 變數索引 -> Field element 位元組
    pub assignments: HashMap<u32, Vec<u8>>,
}

/// 完整的測試案例 = 電路 + 見證
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTestCase {
    pub circuit: SerializableCircuit,
    pub witness: SerializableWitness,
    /// 測試名稱 (例如 "signature_circuit")
    pub name: String,
    /// 描述 (可選)
    pub description: Option<String>,
}

impl SerializableCircuit {
    /// 轉換為 JSON 字串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// 從 JSON 字串載入
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// 儲存為 JSON 檔案
    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// 從 JSON 檔案載入
    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }
    
    /// 轉換為 bincode (更緊湊的二進位格式)
    pub fn to_bincode(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        bincode::serialize(self)
    }
    
    /// 從 bincode 載入
    pub fn from_bincode(bytes: &[u8]) -> Result<Self, Box<bincode::ErrorKind>> {
        bincode::deserialize(bytes)
    }
    
    /// 儲存為 bincode 檔案
    pub fn save_bincode(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.to_bincode()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
    
    /// 從 bincode 檔案載入
    pub fn load_bincode(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        Ok(Self::from_bincode(&bytes)?)
    }
}

impl SerializableWitness {
    /// 轉換為 JSON 字串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// 從 JSON 字串載入
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// 儲存為 JSON 檔案
    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json()?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// 從 JSON 檔案載入
    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(Self::from_json(&json)?)
    }
}

impl SerializableTestCase {
    /// 儲存為 JSON 檔案
    pub fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    /// 從 JSON 檔案載入
    pub fn load_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializable_circuit_json() {
        let circuit = SerializableCircuit {
            field_size: 32,
            prime_bytes: vec![1, 2, 3, 4],
            n_wires: 100,
            n_pub_out: 1,
            n_pub_in: 5,
            n_prv_in: 10,
            n_constraints: 50,
            constraints: vec![],
            wire_labels: None,
        };
        
        // 測試 JSON 序列化
        let json = circuit.to_json().unwrap();
        assert!(json.contains("field_size"));
        assert!(json.contains("32"));
        
        // 測試反序列化
        let loaded = SerializableCircuit::from_json(&json).unwrap();
        assert_eq!(loaded.field_size, 32);
        assert_eq!(loaded.n_wires, 100);
    }

    #[test]
    fn test_serializable_witness_json() {
        let mut assignments = HashMap::new();
        assignments.insert(0, vec![1, 0, 0, 0]);
        assignments.insert(1, vec![5, 0, 0, 0]);
        
        let witness = SerializableWitness { assignments };
        
        // 測試 JSON 序列化
        let json = witness.to_json().unwrap();
        let loaded = SerializableWitness::from_json(&json).unwrap();
        
        assert_eq!(loaded.assignments.len(), 2);
        assert_eq!(loaded.assignments.get(&0).unwrap(), &vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_bincode_serialization() {
        let circuit = SerializableCircuit {
            field_size: 32,
            prime_bytes: vec![1, 2, 3, 4],
            n_wires: 100,
            n_pub_out: 1,
            n_pub_in: 5,
            n_prv_in: 10,
            n_constraints: 50,
            constraints: vec![],
            wire_labels: None,
        };
        
        // 測試 bincode 序列化
        let bytes = circuit.to_bincode().unwrap();
        let loaded = SerializableCircuit::from_bincode(&bytes).unwrap();
        
        assert_eq!(loaded.field_size, 32);
        assert_eq!(loaded.n_wires, 100);
    }
}
