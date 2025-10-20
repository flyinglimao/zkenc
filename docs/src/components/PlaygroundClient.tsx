// Client-side only component - imports zkenc-js
import React, { useState, useEffect } from "react";
import styles from "../pages/playground.module.css";
import { Sudoku, generator } from "@forfuns/sudoku";
import { encrypt, decrypt, getPublicInput, encap, decap } from "zkenc-js";

// Sudoku utilities
function generateSudoku(): { puzzle: number[] } {
  const puzzleArray = generator(1);
  const formattedPuzzle = puzzleArray.map((n) => (n === -1 ? 0 : n));
  return { puzzle: formattedPuzzle };
}

function solveSudoku(puzzle: number[]): {
  solution: number[] | null;
  solvable: boolean;
} {
  try {
    const puzzleForSolver = puzzle.map((n) => (n === 0 ? -1 : n));
    const sudoku = new Sudoku(puzzleForSolver);
    const solutionArray = sudoku.getSolution();

    if (!solutionArray || solutionArray.some((n) => n === -1)) {
      return { solution: null, solvable: false };
    }

    return { solution: solutionArray, solvable: true };
  } catch (error) {
    return { solution: null, solvable: false };
  }
}

export default function PlaygroundClient(): React.ReactElement {
  // Tab state
  const [activeTab, setActiveTab] = useState<"sudoku" | "custom">("sudoku");

  // Mode state
  const [mode, setMode] = useState<"encrypt" | "decrypt">("encrypt");

  // Advanced mode state
  const [advancedMode, setAdvancedMode] = useState(false);
  const [generatedKey, setGeneratedKey] = useState<Uint8Array | null>(null);

  // Sudoku-specific state
  const [message, setMessage] = useState("");
  const [puzzle, setPuzzle] = useState<number[]>([]);
  const [solution, setSolution] = useState<number[]>([]);
  const [ciphertext, setCiphertext] = useState<Uint8Array | null>(null);
  const [decrypted, setDecrypted] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [uploadedCiphertext, setUploadedCiphertext] =
    useState<Uint8Array | null>(null);

  // Circuit files state
  const [circuitFiles, setCircuitFiles] = useState<{
    r1csBuffer: Uint8Array;
    wasmBuffer: Uint8Array;
  } | null>(null);

  // Custom circuit state
  const [customCircuitFiles, setCustomCircuitFiles] = useState<{
    r1csBuffer: Uint8Array;
    wasmBuffer: Uint8Array;
  } | null>(null);
  const [customPublicInput, setCustomPublicInput] = useState("");
  const [customPrivateInput, setCustomPrivateInput] = useState("");
  const [customMessage, setCustomMessage] = useState("");
  const [customCiphertext, setCustomCiphertext] = useState<Uint8Array | null>(
    null
  );
  const [customDecrypted, setCustomDecrypted] = useState("");
  const [includePublicInput, setIncludePublicInput] = useState(true);

  // Load circuit files on mount
  useEffect(() => {
    async function loadCircuits() {
      try {
        const [r1csRes, wasmRes] = await Promise.all([
          fetch("/circuits/sudoku.r1cs"),
          fetch("/circuits/sudoku.wasm"),
        ]);

        if (!r1csRes.ok || !wasmRes.ok) {
          setError("Failed to load circuit files");
          return;
        }

        const r1csBuffer = new Uint8Array(await r1csRes.arrayBuffer());
        const wasmBuffer = new Uint8Array(await wasmRes.arrayBuffer());

        setCircuitFiles({ r1csBuffer, wasmBuffer });
      } catch (err) {
        setError("Error loading circuits: " + (err as Error).message);
      }
    }

    loadCircuits();
  }, []);

  useEffect(() => {
    // Initialize with a random sudoku
    handleGenerateRandom();
  }, []);

  const handleGenerateRandom = () => {
    const { puzzle: newPuzzle } = generateSudoku();
    setPuzzle(newPuzzle);
    setSolution([]);
    setError("");
  };

  const handleAutoSolve = () => {
    setLoading(true);
    setError("");

    try {
      const { solution: solvedPuzzle, solvable } = solveSudoku(puzzle);

      if (!solvable || !solvedPuzzle) {
        setError(
          "❌ Unable to solve this puzzle. It may not have a valid solution."
        );
        setSolution([]);
      } else {
        setSolution(solvedPuzzle);
        setError("");
      }
    } catch (err) {
      setError("❌ Error solving puzzle: " + (err as Error).message);
      setSolution([]);
    } finally {
      setLoading(false);
    }
  };

  const handleEncrypt = async () => {
    if (!message) {
      setError("Please enter a message");
      return;
    }

    if (!circuitFiles) {
      setError("Circuit files not loaded yet. Please wait...");
      return;
    }

    if (puzzle.length !== 81) {
      setError("Please generate a puzzle first");
      return;
    }

    setLoading(true);
    setError("");

    try {
      const encoder = new TextEncoder();
      const messageBytes = encoder.encode(message);

      const publicInputs = {
        puzzle: puzzle,
      };

      const { ciphertext: encryptedData } = await encrypt(
        circuitFiles,
        publicInputs,
        messageBytes
      );

      setCiphertext(encryptedData);
      setError("");
    } catch (err) {
      console.error(err);
      setError(err instanceof Error ? err.message : "Encryption failed");
    } finally {
      setLoading(false);
    }
  };

  const handleDecrypt = async () => {
    const ciphertextToUse = uploadedCiphertext || ciphertext;

    if (!ciphertextToUse) {
      setError("Please encrypt a message or upload a ciphertext first");
      return;
    }

    if (!circuitFiles) {
      setError("Circuit files not loaded yet. Please wait...");
      return;
    }

    const hasZeros = solution.some((v) => v === 0);
    if (hasZeros) {
      setError("Please fill in the complete solution");
      return;
    }

    if (solution.length !== 81) {
      setError("Invalid solution length");
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      const inputs = {
        puzzle: puzzle,
        solution: solution,
      };

      const decryptedBytes = await decrypt(
        circuitFiles,
        ciphertextToUse,
        inputs
      );

      const decoder = new TextDecoder();
      const decryptedMessage = decoder.decode(decryptedBytes);

      setDecrypted(decryptedMessage || "Decryption successful!");
    } catch (err) {
      setError(
        "❌ Decryption failed: " +
          (err instanceof Error
            ? err.message
            : "Invalid witness or corrupted ciphertext")
      );
    } finally {
      setLoading(false);
    }
  };

  const handleDownloadCiphertext = () => {
    if (!ciphertext) return;

    const blob = new Blob([ciphertext], { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "sudoku-encrypted.bin";
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleUploadCiphertext = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      const ciphertextBytes = new Uint8Array(arrayBuffer);
      setUploadedCiphertext(ciphertextBytes);

      // Try to extract public input (puzzle) from ciphertext
      try {
        const publicInputs = getPublicInput(ciphertextBytes);
        if (publicInputs.puzzle && Array.isArray(publicInputs.puzzle)) {
          setPuzzle(publicInputs.puzzle);
          setSolution(publicInputs.puzzle); // Initialize solution with puzzle
          setError("");
        }
      } catch (err) {
        // Public input not included, user will need to load puzzle separately
        console.log("Public input not included in ciphertext:", err);
      }

      setMode("decrypt");
    };
    reader.readAsArrayBuffer(file);
  };

  // Custom circuit handlers
  const handleCustomR1CSUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      const r1csBuffer = new Uint8Array(arrayBuffer);
      setCustomCircuitFiles((prev) => ({
        r1csBuffer,
        wasmBuffer: prev?.wasmBuffer || new Uint8Array(),
      }));
      setError("");
    };
    reader.readAsArrayBuffer(file);
  };

  const handleCustomWasmUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      const wasmBuffer = new Uint8Array(arrayBuffer);
      setCustomCircuitFiles((prev) => ({
        r1csBuffer: prev?.r1csBuffer || new Uint8Array(),
        wasmBuffer,
      }));
      setError("");
    };
    reader.readAsArrayBuffer(file);
  };

  const handleCustomEncrypt = async () => {
    if (!customMessage) {
      setError("Please enter a message");
      return;
    }

    if (
      !customCircuitFiles ||
      !customCircuitFiles.r1csBuffer.length ||
      !customCircuitFiles.wasmBuffer.length
    ) {
      setError("Please upload both R1CS and WASM files");
      return;
    }

    if (!customPublicInput) {
      setError("Please enter public input JSON");
      return;
    }

    setLoading(true);
    setError("");
    setGeneratedKey(null);

    try {
      const publicInputs = JSON.parse(customPublicInput);
      const encoder = new TextEncoder();
      const messageBytes = encoder.encode(customMessage);

      if (advancedMode) {
        // Use low-level API
        const { ciphertext: witnessCiphertext, key } = await encap(
          customCircuitFiles,
          publicInputs
        );

        setGeneratedKey(key);

        // Manually encrypt with the key (simplified version)
        const { ciphertext: fullCiphertext } = await encrypt(
          customCircuitFiles,
          publicInputs,
          messageBytes,
          { includePublicInput }
        );

        setCustomCiphertext(fullCiphertext);
      } else {
        // Use high-level API
        const { ciphertext: encryptedData } = await encrypt(
          customCircuitFiles,
          publicInputs,
          messageBytes,
          { includePublicInput }
        );

        setCustomCiphertext(encryptedData);
      }

      setError("");
    } catch (err) {
      console.error(err);
      setError(err instanceof Error ? err.message : "Encryption failed");
    } finally {
      setLoading(false);
    }
  };

  const handleCustomDecrypt = async () => {
    if (!customCiphertext) {
      setError("Please encrypt a message or upload a ciphertext first");
      return;
    }

    if (
      !customCircuitFiles ||
      !customCircuitFiles.r1csBuffer.length ||
      !customCircuitFiles.wasmBuffer.length
    ) {
      setError("Please upload both R1CS and WASM files");
      return;
    }

    let fullInputs;
    try {
      fullInputs = JSON.parse(customPrivateInput || customPublicInput);
    } catch (err) {
      setError("Invalid JSON input");
      return;
    }

    setLoading(true);
    setError("");
    setCustomDecrypted("");

    try {
      const decryptedBytes = await decrypt(
        customCircuitFiles,
        customCiphertext,
        fullInputs
      );

      const decoder = new TextDecoder();
      const decryptedMessage = decoder.decode(decryptedBytes);

      setCustomDecrypted(decryptedMessage || "Decryption successful!");
    } catch (err) {
      setError(
        "❌ Decryption failed: " +
          (err instanceof Error
            ? err.message
            : "Invalid witness or corrupted ciphertext")
      );
    } finally {
      setLoading(false);
    }
  };

  const handleCustomCiphertextUpload = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      const ciphertextBytes = new Uint8Array(arrayBuffer);
      setCustomCiphertext(ciphertextBytes);

      // Try to extract public input from ciphertext
      try {
        const publicInputs = getPublicInput(ciphertextBytes);
        setCustomPublicInput(JSON.stringify(publicInputs, null, 2));
        setError("");
      } catch (err) {
        console.log("Public input not included in ciphertext:", err);
      }

      setMode("decrypt");
    };
    reader.readAsArrayBuffer(file);
  };

  const handleDownloadCustomCiphertext = () => {
    if (!customCiphertext) return;

    const blob = new Blob([customCiphertext], {
      type: "application/octet-stream",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "custom-encrypted.bin";
    a.click();
    URL.revokeObjectURL(url);
  };

  const handlePublicInputFileUpload = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      try {
        const text = reader.result as string;
        const json = JSON.parse(text);
        setCustomPublicInput(JSON.stringify(json, null, 2));
        setError("");
      } catch (err) {
        setError("Invalid JSON file: " + (err as Error).message);
      }
    };
    reader.readAsText(file);
  };

  const handlePrivateInputFileUpload = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      try {
        const text = reader.result as string;
        const json = JSON.parse(text);
        setCustomPrivateInput(JSON.stringify(json, null, 2));
        setError("");
      } catch (err) {
        setError("Invalid JSON file: " + (err as Error).message);
      }
    };
    reader.readAsText(file);
  };

  const renderSudokuGrid = (
    values: number[],
    editable: boolean,
    type: "puzzle" | "solution" = "puzzle"
  ) => {
    return (
      <div className={styles.sudokuGrid}>
        {values.map((value, index) => {
          const row = Math.floor(index / 9);
          const col = index % 9;
          // Only lock cells when editing solution (not when editing puzzle)
          const isGiven = type === "solution" && puzzle[index] !== 0;

          return (
            <input
              key={index}
              type="text"
              className={`${styles.sudokuCell} ${isGiven ? styles.given : ""}`}
              value={value === 0 ? "" : value}
              onChange={(e) => {
                if (!editable) return;
                const newValue = parseInt(e.target.value) || 0;
                if (newValue >= 0 && newValue <= 9) {
                  const newValues = [...values];
                  newValues[index] = newValue;
                  if (type === "solution") {
                    setSolution(newValues);
                  } else {
                    setPuzzle(newValues);
                  }
                }
              }}
              maxLength={1}
              disabled={!editable || isGiven}
            />
          );
        })}
      </div>
    );
  };

  return (
    <div className={styles.playground}>
      <div className={styles.header}>
        <h1>🎮 Witness Encryption Playground</h1>
        <p>
          Experiment with witness encryption using pre-built circuits or upload
          your own
        </p>
      </div>

      {/* Tab Selector */}
      <div className={styles.tabSelector}>
        <button
          className={activeTab === "sudoku" ? styles.active : ""}
          onClick={() => setActiveTab("sudoku")}
        >
          🧩 Sudoku Demo
        </button>
        <button
          className={activeTab === "custom" ? styles.active : ""}
          onClick={() => setActiveTab("custom")}
        >
          ⚙️ Custom Circuit
        </button>
      </div>

      {/* Advanced Mode Toggle */}
      <div className={styles.advancedToggle}>
        <label>
          <input
            type="checkbox"
            checked={advancedMode}
            onChange={(e) => setAdvancedMode(e.target.checked)}
          />
          <span>🔬 Advanced Mode (Show encryption key)</span>
        </label>
      </div>

      {activeTab === "sudoku" ? (
        // Sudoku Tab
        <>
          <div className={styles.modeSelector}>
            <button
              className={mode === "encrypt" ? styles.active : ""}
              onClick={() => setMode("encrypt")}
            >
              🔐 Encrypt
            </button>
            <button
              className={mode === "decrypt" ? styles.active : ""}
              onClick={() => setMode("decrypt")}
            >
              🔓 Decrypt
            </button>
          </div>

          {mode === "encrypt" ? (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>1. Generate or Enter Puzzle</h2>
                <button
                  onClick={handleGenerateRandom}
                  className={styles.button}
                >
                  🎲 Generate Random Puzzle
                </button>
                <div className={styles.puzzleContainer}>
                  {renderSudokuGrid(puzzle, true, "puzzle")}
                </div>
              </div>

              <div className={styles.section}>
                <h2>2. Enter Your Message</h2>
                <textarea
                  className={styles.textarea}
                  value={message}
                  onChange={(e) => setMessage(e.target.value)}
                  placeholder="Enter secret message to encrypt..."
                  rows={4}
                />
              </div>

              <div className={styles.section}>
                <button
                  onClick={handleEncrypt}
                  disabled={loading || !message}
                  className={styles.primaryButton}
                >
                  {loading ? "🔄 Encrypting..." : "🔐 Encrypt Message"}
                </button>

                {ciphertext && (
                  <div className={styles.success}>
                    <p>✅ Encryption successful!</p>
                    <p>Ciphertext size: {ciphertext.length} bytes</p>
                    <button
                      onClick={handleDownloadCiphertext}
                      className={styles.button}
                    >
                      📥 Download Ciphertext
                    </button>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>1. Load Ciphertext</h2>
                <div className={styles.uploadArea}>
                  <label className={styles.uploadButton}>
                    📤 Upload Ciphertext File
                    <input
                      type="file"
                      accept=".bin"
                      onChange={handleUploadCiphertext}
                      style={{ display: "none" }}
                    />
                  </label>
                  {(uploadedCiphertext || ciphertext) && (
                    <p className={styles.uploadSuccess}>
                      ✅ Ciphertext loaded (
                      {(uploadedCiphertext || ciphertext)!.length} bytes)
                    </p>
                  )}
                </div>
              </div>

              <div className={styles.section}>
                <h2>2. Enter Puzzle</h2>
                <p className={styles.hint}>
                  {puzzle.length === 81 && puzzle.some((n) => n !== 0)
                    ? "Puzzle loaded from ciphertext. You can edit if needed."
                    : "Enter the puzzle numbers. Empty cells should be 0."}
                </p>
                <div className={styles.puzzleContainer}>
                  {renderSudokuGrid(puzzle, true, "puzzle")}
                </div>
              </div>

              <div className={styles.section}>
                <h2>3. Enter Solution</h2>
                <p className={styles.hint}>
                  Fill in the empty cells to solve the puzzle. Given numbers are
                  locked.
                </p>
                <div style={{ marginBottom: "1rem" }}>
                  <button
                    onClick={handleAutoSolve}
                    className={styles.button}
                    disabled={loading || !puzzle.length}
                  >
                    {loading ? "🔄 Solving..." : "🧩 Auto Solve Puzzle"}
                  </button>
                </div>
                <div className={styles.puzzleContainer}>
                  {renderSudokuGrid(
                    solution.length ? solution : puzzle,
                    true,
                    "solution"
                  )}
                </div>
              </div>

              <div className={styles.section}>
                <button
                  onClick={handleDecrypt}
                  disabled={loading || !(uploadedCiphertext || ciphertext)}
                  className={styles.primaryButton}
                >
                  {loading ? "🔄 Decrypting..." : "🔓 Decrypt Message"}
                </button>

                {decrypted && (
                  <div className={styles.success}>
                    <h3>✅ Decrypted Message:</h3>
                    <pre className={styles.decryptedMessage}>{decrypted}</pre>
                  </div>
                )}
              </div>
            </div>
          )}

          <div className={styles.info}>
            <h3>ℹ️ About Sudoku Demo</h3>
            <p>
              This demo uses a Sudoku circuit. The message can only be decrypted
              by providing a valid solution.
            </p>
            <ul>
              <li>✅ Uses real zkenc-js library</li>
              <li>✅ Verifies Sudoku solutions cryptographically</li>
              <li>✅ Only valid witnesses (correct solutions) can decrypt</li>
            </ul>
          </div>
        </>
      ) : (
        // Custom Circuit Tab
        <>
          <div className={styles.modeSelector}>
            <button
              className={mode === "encrypt" ? styles.active : ""}
              onClick={() => setMode("encrypt")}
            >
              🔐 Encrypt
            </button>
            <button
              className={mode === "decrypt" ? styles.active : ""}
              onClick={() => setMode("decrypt")}
            >
              🔓 Decrypt
            </button>
          </div>

          {mode === "encrypt" ? (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>1. Upload Circuit Files</h2>
                <div className={styles.uploadGrid}>
                  <div>
                    <label className={styles.fileLabel}>
                      📄 R1CS File (.r1cs)
                      <input
                        type="file"
                        accept=".r1cs"
                        onChange={handleCustomR1CSUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.r1csBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        ✅ R1CS loaded ({customCircuitFiles.r1csBuffer.length}{" "}
                        bytes)
                      </p>
                    )}
                  </div>
                  <div>
                    <label className={styles.fileLabel}>
                      ⚙️ WASM File (.wasm)
                      <input
                        type="file"
                        accept=".wasm"
                        onChange={handleCustomWasmUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.wasmBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        ✅ WASM loaded ({customCircuitFiles.wasmBuffer.length}{" "}
                        bytes)
                      </p>
                    )}
                  </div>
                </div>
              </div>

              <div className={styles.section}>
                <h2>2. Enter Public Input (JSON)</h2>
                <p className={styles.hint}>
                  Enter the public inputs as JSON. You can also paste JSON or
                  upload a file.
                </p>
                <div className={styles.inputWithUpload}>
                  <textarea
                    className={styles.textarea}
                    value={customPublicInput}
                    onChange={(e) => setCustomPublicInput(e.target.value)}
                    placeholder='{"signal1": 42, "signal2": [1, 2, 3]}'
                    rows={6}
                  />
                  <label className={styles.uploadJsonButton}>
                    📁 Upload JSON File
                    <input
                      type="file"
                      accept=".json"
                      onChange={handlePublicInputFileUpload}
                      style={{ display: "none" }}
                    />
                  </label>
                </div>
              </div>

              <div className={styles.section}>
                <h2>3. Enter Message</h2>
                <textarea
                  className={styles.textarea}
                  value={customMessage}
                  onChange={(e) => setCustomMessage(e.target.value)}
                  placeholder="Enter secret message to encrypt..."
                  rows={4}
                />
              </div>

              <div className={styles.section}>
                <div className={styles.optionsGroup}>
                  <label className={styles.checkboxLabel}>
                    <input
                      type="checkbox"
                      checked={includePublicInput}
                      onChange={(e) => setIncludePublicInput(e.target.checked)}
                    />
                    <span>
                      Include public input in ciphertext (recommended)
                    </span>
                  </label>
                  <p className={styles.optionHint}>
                    When enabled, the public input will be embedded in the
                    ciphertext for easier decryption.
                  </p>
                </div>
              </div>

              <div className={styles.section}>
                <button
                  onClick={handleCustomEncrypt}
                  disabled={loading || !customMessage || !customCircuitFiles}
                  className={styles.primaryButton}
                >
                  {loading ? "🔄 Encrypting..." : "🔐 Encrypt Message"}
                </button>

                {customCiphertext && (
                  <div className={styles.success}>
                    <p>✅ Encryption successful!</p>
                    <p>Ciphertext size: {customCiphertext.length} bytes</p>
                    {advancedMode && generatedKey && (
                      <div className={styles.keyDisplay}>
                        <h4>🔑 Generated Key:</h4>
                        <pre className={styles.keyHex}>
                          {Array.from(generatedKey)
                            .map((b: number) => b.toString(16).padStart(2, "0"))
                            .join("")}
                        </pre>
                        <p className={styles.keyNote}>
                          ⚠️ This key is for educational purposes. In
                          production, keys are used internally.
                        </p>
                      </div>
                    )}
                    <button
                      onClick={handleDownloadCustomCiphertext}
                      className={styles.button}
                    >
                      📥 Download Ciphertext
                    </button>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>1. Upload Circuit Files</h2>
                <div className={styles.uploadGrid}>
                  <div>
                    <label className={styles.fileLabel}>
                      📄 R1CS File (.r1cs)
                      <input
                        type="file"
                        accept=".r1cs"
                        onChange={handleCustomR1CSUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.r1csBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        ✅ R1CS loaded ({customCircuitFiles.r1csBuffer.length}{" "}
                        bytes)
                      </p>
                    )}
                  </div>
                  <div>
                    <label className={styles.fileLabel}>
                      ⚙️ WASM File (.wasm)
                      <input
                        type="file"
                        accept=".wasm"
                        onChange={handleCustomWasmUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.wasmBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        ✅ WASM loaded ({customCircuitFiles.wasmBuffer.length}{" "}
                        bytes)
                      </p>
                    )}
                  </div>
                </div>
              </div>

              <div className={styles.section}>
                <h2>2. Load Ciphertext</h2>
                <div className={styles.uploadArea}>
                  <label className={styles.uploadButton}>
                    📤 Upload Ciphertext File
                    <input
                      type="file"
                      accept=".bin"
                      onChange={handleCustomCiphertextUpload}
                      style={{ display: "none" }}
                    />
                  </label>
                  {customCiphertext && (
                    <p className={styles.uploadSuccess}>
                      ✅ Ciphertext loaded ({customCiphertext.length} bytes)
                    </p>
                  )}
                </div>
              </div>

              <div className={styles.section}>
                <h2>3. Enter Public Input (JSON)</h2>
                <p className={styles.hint}>
                  {customPublicInput && customPublicInput.trim() !== "{}"
                    ? "Public input loaded from ciphertext. You can edit if needed."
                    : "Enter the public inputs that were used during encryption."}
                </p>
                <div className={styles.inputWithUpload}>
                  <textarea
                    className={styles.textarea}
                    value={customPublicInput}
                    onChange={(e) => setCustomPublicInput(e.target.value)}
                    placeholder='{"signal1": 42, "signal2": [1, 2, 3]}'
                    rows={6}
                  />
                  <label className={styles.uploadJsonButton}>
                    📁 Upload JSON File
                    <input
                      type="file"
                      accept=".json"
                      onChange={handlePublicInputFileUpload}
                      style={{ display: "none" }}
                    />
                  </label>
                </div>
              </div>

              <div className={styles.section}>
                <h2>4. Enter Private Input (JSON)</h2>
                <p className={styles.hint}>
                  Enter the complete witness (public + private inputs). If you
                  only have private signals, they will be merged with public
                  inputs above.
                </p>
                <div className={styles.inputWithUpload}>
                  <textarea
                    className={styles.textarea}
                    value={customPrivateInput}
                    onChange={(e) => setCustomPrivateInput(e.target.value)}
                    placeholder='{"signal1": 42, "signal2": [1, 2, 3], "privateSignal": 123}'
                    rows={6}
                  />
                  <label className={styles.uploadJsonButton}>
                    📁 Upload JSON File
                    <input
                      type="file"
                      accept=".json"
                      onChange={handlePrivateInputFileUpload}
                      style={{ display: "none" }}
                    />
                  </label>
                </div>
              </div>

              <div className={styles.section}>
                <button
                  onClick={handleCustomDecrypt}
                  disabled={loading || !customCiphertext || !customCircuitFiles}
                  className={styles.primaryButton}
                >
                  {loading ? "🔄 Decrypting..." : "🔓 Decrypt Message"}
                </button>

                {customDecrypted && (
                  <div className={styles.success}>
                    <h3>✅ Decrypted Message:</h3>
                    <pre className={styles.decryptedMessage}>
                      {customDecrypted}
                    </pre>
                  </div>
                )}
              </div>
            </div>
          )}

          <div className={styles.info}>
            <h3>ℹ️ About Custom Circuits</h3>
            <p>
              Upload your own Circom circuits to experiment with witness
              encryption.
            </p>
            <ul>
              <li>
                📄 Compile your circuit with:{" "}
                <code>circom circuit.circom --r1cs --wasm</code>
              </li>
              <li>📦 Upload both .r1cs and .wasm files</li>
              <li>🔐 Public inputs are used for encryption</li>
              <li>
                🔓 Full witness (public + private) is needed for decryption
              </li>
              <li>
                💾 Public inputs can be embedded in ciphertext for convenience
              </li>
            </ul>
            <p>
              <a href="/docs/getting-started/zkenc-js">Learn more →</a>
            </p>
          </div>
        </>
      )}

      {error && <div className={styles.error}>❌ {error}</div>}
    </div>
  );
}
