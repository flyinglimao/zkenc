//! zkenc-js
//!
//! WASM bindings for zkenc-core

use wasm_bindgen::prelude::*;

// 當 `wee_alloc` feature 啟用時，使用 `wee_alloc` 作為全域分配器
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 初始化 panic hook（用於更好的錯誤訊息）
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// 範例：WASM 綁定函數
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! This is zkenc-js.", name)
}

/// 範例：加密函數的 WASM 介面
#[wasm_bindgen]
pub struct WasmEncryptor {
    // 內部狀態
}

#[wasm_bindgen]
impl WasmEncryptor {
    /// 創建新的加密器實例
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    /// 加密資料
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, JsValue> {
        // TODO: 實作加密邏輯
        Ok(data.to_vec())
    }

    /// 解密資料
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, JsValue> {
        // TODO: 實作解密邏輯
        Ok(data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World! This is zkenc-js.");
    }
}
