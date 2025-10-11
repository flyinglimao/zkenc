# zkenc-js

## 概述
`zkenc-js` 是一個將核心演算法包裝成 WebAssembly (WASM) 的 JavaScript 專案。它提供了一個簡單的 JavaScript 介面，方便開發者在網頁或 Node.js 環境中使用這些演算法。

## 安裝
要安裝 `zkenc-js`，請使用以下命令：

```bash
npm install
```

或使用 yarn：

```bash
yarn install
```

## 使用說明
在您的 TypeScript 或 JavaScript 檔案中，您可以這樣導入和使用 `zkenc-js`：

```javascript
import { yourFunction } from 'zkenc-js';

// 使用 yourFunction
yourFunction();
```

請確保在使用之前已經編譯了 WASM 模組。

## 開發
在開發過程中，您可以使用以下命令來啟動開發伺服器：

```bash
npm run dev
```

這將會啟動一個本地伺服器，並監控檔案變更。

## 貢獻
歡迎任何形式的貢獻！請查看 [貢獻指南](CONTRIBUTING.md) 以獲取更多資訊。

## 授權
本專案採用 MIT 授權。詳情請參閱 [LICENSE](LICENSE) 檔案。