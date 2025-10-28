// Client-side only component - imports zkenc-js
import React, { useState, useEffect } from "react";
import styles from "../pages/playground.module.css";
import { Sudoku, generator } from "@forfuns/sudoku";
import { encrypt, decrypt, getPublicInput } from "zkenc-js";
import Translate, { translate } from "@docusaurus/Translate";
import useBaseUrl from "@docusaurus/useBaseUrl";

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
    symContent: string;
  } | null>(null);

  // Custom circuit state
  const [customCircuitFiles, setCustomCircuitFiles] = useState<{
    r1csBuffer: Uint8Array;
    wasmBuffer: Uint8Array;
    symContent?: string;
  } | null>(null);
  const [customPublicInput, setCustomPublicInput] = useState("");
  const [customPrivateInput, setCustomPrivateInput] = useState("");
  const [customMessage, setCustomMessage] = useState("");
  const [customCiphertext, setCustomCiphertext] = useState<Uint8Array | null>(
    null
  );
  const [customDecrypted, setCustomDecrypted] = useState("");
  const [includePublicInput, setIncludePublicInput] = useState(true);

  const sudokuR1cs = useBaseUrl("/circuits/sudoku.r1cs");
  const sudokuWasm = useBaseUrl("/circuits/sudoku.wasm");
  const sudokuSym = useBaseUrl("/circuits/sudoku.sym");

  // Load circuit files on mount
  useEffect(() => {
    async function loadCircuits() {
      try {
        console.log(sudokuR1cs);
        const [r1csRes, wasmRes, symRes] = await Promise.all([
          fetch(sudokuR1cs),
          fetch(sudokuWasm),
          fetch(sudokuSym),
        ]);

        if (!r1csRes.ok || !wasmRes.ok || !symRes.ok) {
          setError("Failed to load circuit files");
          return;
        }

        const r1csBuffer = new Uint8Array(await r1csRes.arrayBuffer());
        const wasmBuffer = new Uint8Array(await wasmRes.arrayBuffer());
        const symContent = await symRes.text(); // Read sym as UTF-8 string

        setCircuitFiles({ r1csBuffer, wasmBuffer, symContent });
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
        symContent: prev?.symContent,
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
        symContent: prev?.symContent,
      }));
      setError("");
    };
    reader.readAsArrayBuffer(file);
  };

  const handleCustomSymUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = () => {
      const symContent = reader.result as string;
      setCustomCircuitFiles((prev) => ({
        r1csBuffer: prev?.r1csBuffer || new Uint8Array(),
        wasmBuffer: prev?.wasmBuffer || new Uint8Array(),
        symContent,
      }));
      setError("");
    };
    reader.readAsText(file); // Read as UTF-8 string
  };

  const handleCustomEncrypt = async () => {
    if (!customMessage) {
      setError("Please enter a message");
      return;
    }

    if (
      !customCircuitFiles ||
      !customCircuitFiles.r1csBuffer.length ||
      !customCircuitFiles.symContent
    ) {
      setError("Please upload both R1CS and SYM files");
      return;
    }

    if (!customPublicInput) {
      setError("Please enter public input JSON");
      return;
    }

    setLoading(true);
    setError("");

    try {
      const publicInputs = JSON.parse(customPublicInput);
      const encoder = new TextEncoder();
      const messageBytes = encoder.encode(customMessage);

      // Use high-level API - encrypt only needs r1csBuffer and symContent
      const { ciphertext: encryptedData } = await encrypt(
        {
          r1csBuffer: customCircuitFiles.r1csBuffer,
          symContent: customCircuitFiles.symContent,
        },
        publicInputs,
        messageBytes,
        { includePublicInput }
      );

      setCustomCiphertext(encryptedData);

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
        <h1>
          <Translate id="playground.header.title">
            🎮 Witness Encryption Playground
          </Translate>
        </h1>
        <p>
          <Translate id="playground.header.description">
            Experiment with witness encryption using pre-built circuits or
            upload your own
          </Translate>
        </p>
      </div>

      {/* Tab Selector */}
      <div className={styles.tabSelector}>
        <button
          className={activeTab === "sudoku" ? styles.active : ""}
          onClick={() => setActiveTab("sudoku")}
        >
          <Translate id="playground.tabs.sudoku">🧩 Sudoku Demo</Translate>
        </button>
        <button
          className={activeTab === "custom" ? styles.active : ""}
          onClick={() => setActiveTab("custom")}
        >
          <Translate id="playground.tabs.custom">⚙️ Custom Circuit</Translate>
        </button>
      </div>

      {activeTab === "sudoku" ? (
        // Sudoku Tab
        <>
          <div className={styles.modeSelector}>
            <button
              className={mode === "encrypt" ? styles.active : ""}
              onClick={() => setMode("encrypt")}
            >
              <Translate id="playground.mode.encrypt">🔐 Encrypt</Translate>
            </button>
            <button
              className={mode === "decrypt" ? styles.active : ""}
              onClick={() => setMode("decrypt")}
            >
              <Translate id="playground.mode.decrypt">🔓 Decrypt</Translate>
            </button>
          </div>

          {mode === "encrypt" ? (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>
                  <Translate id="playground.sudoku.encrypt.step1">
                    1. Generate or Enter Puzzle
                  </Translate>
                </h2>
                <button
                  onClick={handleGenerateRandom}
                  className={styles.button}
                >
                  <Translate id="playground.sudoku.encrypt.generateButton">
                    🎲 Generate Random Puzzle
                  </Translate>
                </button>
                <div className={styles.puzzleContainer}>
                  {renderSudokuGrid(puzzle, true, "puzzle")}
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.sudoku.encrypt.step2">
                    2. Enter Your Message
                  </Translate>
                </h2>
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
                  {loading ? (
                    <Translate id="playground.sudoku.encrypt.encrypting">
                      🔄 Encrypting...
                    </Translate>
                  ) : (
                    <Translate id="playground.sudoku.encrypt.encryptButton">
                      🔐 Encrypt Message
                    </Translate>
                  )}
                </button>

                {ciphertext && (
                  <div className={styles.success}>
                    <p>
                      <Translate id="playground.sudoku.encrypt.success">
                        ✅ Encryption successful!
                      </Translate>
                    </p>
                    <p>
                      <Translate
                        id="playground.sudoku.encrypt.ciphertextSize"
                        values={{ size: ciphertext.length }}
                      >
                        {"Ciphertext size: {size} bytes"}
                      </Translate>
                    </p>
                    <button
                      onClick={handleDownloadCiphertext}
                      className={styles.button}
                    >
                      <Translate id="playground.sudoku.encrypt.downloadButton">
                        📥 Download Ciphertext
                      </Translate>
                    </button>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>
                  <Translate id="playground.sudoku.decrypt.step1">
                    1. Load Ciphertext
                  </Translate>
                </h2>
                <div className={styles.uploadArea}>
                  <label className={styles.uploadButton}>
                    <Translate id="playground.sudoku.decrypt.uploadButton">
                      📤 Upload Ciphertext File
                    </Translate>
                    <input
                      type="file"
                      accept=".bin"
                      onChange={handleUploadCiphertext}
                      style={{ display: "none" }}
                    />
                  </label>
                  {(uploadedCiphertext || ciphertext) && (
                    <p className={styles.uploadSuccess}>
                      <Translate
                        id="playground.sudoku.decrypt.ciphertextLoaded"
                        values={{
                          size: (uploadedCiphertext || ciphertext)!.length,
                        }}
                      >
                        {"✅ Ciphertext loaded ({size} bytes)"}
                      </Translate>
                    </p>
                  )}
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.sudoku.decrypt.step2">
                    2. Enter Puzzle
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  {puzzle.length === 81 && puzzle.some((n) => n !== 0) ? (
                    <Translate id="playground.sudoku.decrypt.puzzleLoaded">
                      Puzzle loaded from ciphertext. You can edit if needed.
                    </Translate>
                  ) : (
                    <Translate id="playground.sudoku.decrypt.puzzleHint">
                      Enter the puzzle numbers. Empty cells should be 0.
                    </Translate>
                  )}
                </p>
                <div className={styles.puzzleContainer}>
                  {renderSudokuGrid(puzzle, true, "puzzle")}
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.sudoku.decrypt.step3">
                    3. Enter Solution
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  <Translate id="playground.sudoku.decrypt.solutionHint">
                    Fill in the empty cells to solve the puzzle. Given numbers
                    are locked.
                  </Translate>
                </p>
                <div style={{ marginBottom: "1rem" }}>
                  <button
                    onClick={handleAutoSolve}
                    className={styles.button}
                    disabled={loading || !puzzle.length}
                  >
                    {loading ? (
                      <Translate id="playground.sudoku.decrypt.solving">
                        🔄 Solving...
                      </Translate>
                    ) : (
                      <Translate id="playground.sudoku.decrypt.autoSolveButton">
                        🧩 Auto Solve Puzzle
                      </Translate>
                    )}
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
                  {loading ? (
                    <Translate id="playground.sudoku.decrypt.decrypting">
                      🔄 Decrypting...
                    </Translate>
                  ) : (
                    <Translate id="playground.sudoku.decrypt.decryptButton">
                      🔓 Decrypt Message
                    </Translate>
                  )}
                </button>

                {error && mode === "decrypt" && activeTab === "sudoku" && (
                  <div className={styles.error}>❌ {error}</div>
                )}

                {decrypted && (
                  <div className={styles.success}>
                    <h3>
                      <Translate id="playground.sudoku.decrypt.successTitle">
                        ✅ Decrypted Message:
                      </Translate>
                    </h3>
                    <pre className={styles.decryptedMessage}>{decrypted}</pre>
                  </div>
                )}
              </div>
            </div>
          )}

          <div className={styles.info}>
            <h3>
              <Translate id="playground.sudoku.info.title">
                ℹ️ About Sudoku Demo
              </Translate>
            </h3>
            <p>
              <Translate id="playground.sudoku.info.description">
                This demo uses a Sudoku circuit. The message can only be
                decrypted by providing a valid solution.
              </Translate>
            </p>
            <ul>
              <li>
                <Translate id="playground.sudoku.info.feature1">
                  ✅ Uses real zkenc-js library
                </Translate>
              </li>
              <li>
                <Translate id="playground.sudoku.info.feature2">
                  ✅ Verifies Sudoku solutions cryptographically
                </Translate>
              </li>
              <li>
                <Translate id="playground.sudoku.info.feature3">
                  ✅ Only valid witnesses (correct solutions) can decrypt
                </Translate>
              </li>
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
              <Translate id="playground.mode.encrypt">🔐 Encrypt</Translate>
            </button>
            <button
              className={mode === "decrypt" ? styles.active : ""}
              onClick={() => setMode("decrypt")}
            >
              <Translate id="playground.mode.decrypt">🔓 Decrypt</Translate>
            </button>
          </div>

          {mode === "encrypt" ? (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>
                  <Translate id="playground.custom.encrypt.step1">
                    1. Upload Circuit Files
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  <Translate id="playground.custom.encrypt.filesHint">
                    For encryption, you only need R1CS and SYM files.
                  </Translate>
                </p>
                <div className={styles.uploadGrid}>
                  <div>
                    <label className={styles.fileLabel}>
                      <Translate id="playground.custom.encrypt.r1csLabel">
                        📄 R1CS File (.r1cs)
                      </Translate>
                      <input
                        type="file"
                        accept=".r1cs"
                        onChange={handleCustomR1CSUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.r1csBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        <Translate
                          id="playground.custom.encrypt.r1csLoaded"
                          values={{
                            size: customCircuitFiles.r1csBuffer.length,
                          }}
                        >
                          {"✅ R1CS loaded ({size} bytes)"}
                        </Translate>
                      </p>
                    )}
                  </div>
                  <div>
                    <label className={styles.fileLabel}>
                      <Translate id="playground.custom.encrypt.symLabel">
                        🔑 Symbol File (.sym)
                      </Translate>
                      <input
                        type="file"
                        accept=".sym"
                        onChange={handleCustomSymUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.symContent && (
                      <p className={styles.fileSuccess}>
                        <Translate
                          id="playground.custom.encrypt.symLoaded"
                          values={{
                            size: customCircuitFiles.symContent.length,
                          }}
                        >
                          {"✅ Symbol loaded ({size} chars)"}
                        </Translate>
                      </p>
                    )}
                  </div>
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.custom.encrypt.step2">
                    2. Enter Public Input (JSON)
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  <Translate id="playground.custom.encrypt.publicInputHint">
                    Enter the public inputs as JSON. You can also paste JSON or
                    upload a file.
                  </Translate>
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
                    <Translate id="playground.custom.encrypt.uploadJson">
                      📁 Upload JSON File
                    </Translate>
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
                <h2>
                  <Translate id="playground.custom.encrypt.step3">
                    3. Enter Message
                  </Translate>
                </h2>
                <textarea
                  className={styles.textarea}
                  value={customMessage}
                  onChange={(e) => setCustomMessage(e.target.value)}
                  placeholder={translate({
                    id: "playground.custom.encrypt.messagePlaceholder",
                    message: "Enter secret message to encrypt...",
                  })}
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
                      <Translate id="playground.custom.encrypt.includePublicInput">
                        Include public input in ciphertext (recommended)
                      </Translate>
                    </span>
                  </label>
                  <p className={styles.optionHint}>
                    <Translate id="playground.custom.encrypt.includePublicInputHint">
                      When enabled, the public input will be embedded in the
                      ciphertext for easier decryption.
                    </Translate>
                  </p>
                </div>
              </div>

              <div className={styles.section}>
                <button
                  onClick={handleCustomEncrypt}
                  disabled={loading || !customMessage || !customCircuitFiles}
                  className={styles.primaryButton}
                >
                  {loading ? (
                    <Translate id="playground.custom.encrypt.encrypting">
                      🔄 Encrypting...
                    </Translate>
                  ) : (
                    <Translate id="playground.custom.encrypt.encryptButton">
                      🔐 Encrypt Message
                    </Translate>
                  )}
                </button>

                {customCiphertext && (
                  <div className={styles.success}>
                    <p>
                      <Translate id="playground.custom.encrypt.success">
                        ✅ Encryption successful!
                      </Translate>
                    </p>
                    <p>
                      <Translate
                        id="playground.custom.encrypt.ciphertextSize"
                        values={{ size: customCiphertext.length }}
                      >
                        {"Ciphertext size: {size} bytes"}
                      </Translate>
                    </p>
                    <button
                      onClick={handleDownloadCustomCiphertext}
                      className={styles.button}
                    >
                      <Translate id="playground.custom.encrypt.download">
                        📥 Download Ciphertext
                      </Translate>
                    </button>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className={styles.content}>
              <div className={styles.section}>
                <h2>
                  <Translate id="playground.custom.decrypt.step1">
                    1. Upload Circuit Files
                  </Translate>
                </h2>
                <div className={styles.uploadGrid}>
                  <div>
                    <label className={styles.fileLabel}>
                      <Translate id="playground.custom.decrypt.r1csLabel">
                        📄 R1CS File (.r1cs)
                      </Translate>
                      <input
                        type="file"
                        accept=".r1cs"
                        onChange={handleCustomR1CSUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.r1csBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        <Translate
                          id="playground.custom.decrypt.r1csLoaded"
                          values={{
                            size: customCircuitFiles.r1csBuffer.length,
                          }}
                        >
                          {"✅ R1CS loaded ({size} bytes)"}
                        </Translate>
                      </p>
                    )}
                  </div>
                  <div>
                    <label className={styles.fileLabel}>
                      <Translate id="playground.custom.decrypt.wasmLabel">
                        ⚙️ WASM File (.wasm)
                      </Translate>
                      <input
                        type="file"
                        accept=".wasm"
                        onChange={handleCustomWasmUpload}
                        className={styles.fileInput}
                      />
                    </label>
                    {customCircuitFiles?.wasmBuffer.length > 0 && (
                      <p className={styles.fileSuccess}>
                        <Translate
                          id="playground.custom.decrypt.wasmLoaded"
                          values={{
                            size: customCircuitFiles.wasmBuffer.length,
                          }}
                        >
                          {"✅ WASM loaded ({size} bytes)"}
                        </Translate>
                      </p>
                    )}
                  </div>
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.custom.decrypt.step2">
                    2. Load Ciphertext
                  </Translate>
                </h2>
                <div className={styles.uploadArea}>
                  <label className={styles.uploadButton}>
                    <Translate id="playground.custom.decrypt.uploadCiphertext">
                      📤 Upload Ciphertext File
                    </Translate>
                    <input
                      type="file"
                      accept=".bin"
                      onChange={handleCustomCiphertextUpload}
                      style={{ display: "none" }}
                    />
                  </label>
                  {customCiphertext && (
                    <p className={styles.uploadSuccess}>
                      <Translate
                        id="playground.custom.decrypt.ciphertextLoaded"
                        values={{ size: customCiphertext.length }}
                      >
                        {"✅ Ciphertext loaded ({size} bytes)"}
                      </Translate>
                    </p>
                  )}
                </div>
              </div>

              <div className={styles.section}>
                <h2>
                  <Translate id="playground.custom.decrypt.step3">
                    3. Enter Public Input (JSON)
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  {customPublicInput && customPublicInput.trim() !== "{}" ? (
                    <Translate id="playground.custom.decrypt.publicInputLoaded">
                      Public input loaded from ciphertext. You can edit if
                      needed.
                    </Translate>
                  ) : (
                    <Translate id="playground.custom.decrypt.publicInputHint">
                      Enter the public inputs that were used during encryption.
                    </Translate>
                  )}
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
                    <Translate id="playground.custom.decrypt.uploadJson">
                      📁 Upload JSON File
                    </Translate>
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
                <h2>
                  <Translate id="playground.custom.decrypt.step4">
                    4. Enter Private Input (JSON)
                  </Translate>
                </h2>
                <p className={styles.hint}>
                  <Translate id="playground.custom.decrypt.privateInputHint">
                    Enter the complete witness (public + private inputs). If you
                    only have private signals, they will be merged with public
                    inputs above.
                  </Translate>
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
                    <Translate id="playground.custom.decrypt.uploadPrivateJson">
                      📁 Upload JSON File
                    </Translate>
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
                  {loading ? (
                    <Translate id="playground.custom.decrypt.decrypting">
                      🔄 Decrypting...
                    </Translate>
                  ) : (
                    <Translate id="playground.custom.decrypt.decryptButton">
                      🔓 Decrypt Message
                    </Translate>
                  )}
                </button>

                {error && mode === "decrypt" && activeTab === "custom" && (
                  <div className={styles.error}>❌ {error}</div>
                )}

                {customDecrypted && (
                  <div className={styles.success}>
                    <h3>
                      <Translate id="playground.custom.decrypt.success">
                        ✅ Decrypted Message:
                      </Translate>
                    </h3>
                    <pre className={styles.decryptedMessage}>
                      {customDecrypted}
                    </pre>
                  </div>
                )}
              </div>
            </div>
          )}

          <div className={styles.info}>
            <h3>
              <Translate id="playground.custom.info.title">
                ℹ️ About Custom Circuits
              </Translate>
            </h3>
            <p>
              <Translate id="playground.custom.info.description">
                Upload your own Circom circuits to experiment with witness
                encryption.
              </Translate>
            </p>
            <ul>
              <li>
                <Translate id="playground.custom.info.compile">
                  📄 Compile your circuit with:
                </Translate>{" "}
                <code>circom circuit.circom --r1cs --wasm</code>
              </li>
              <li>
                <Translate id="playground.custom.info.upload">
                  📦 Upload both .r1cs and .wasm files
                </Translate>
              </li>
              <li>
                <Translate id="playground.custom.info.publicInputs">
                  🔐 Public inputs are used for encryption
                </Translate>
              </li>
              <li>
                <Translate id="playground.custom.info.witness">
                  🔓 Full witness (public + private) is needed for decryption
                </Translate>
              </li>
              <li>
                <Translate id="playground.custom.info.embed">
                  💾 Public inputs can be embedded in ciphertext for convenience
                </Translate>
              </li>
            </ul>
            <p>
              <a href="/docs/getting-started/zkenc-js">
                <Translate id="playground.custom.info.learnMore">
                  Learn more →
                </Translate>
              </a>
            </p>
          </div>
        </>
      )}

      {error && mode === "encrypt" && (
        <div className={styles.error}>❌ {error}</div>
      )}
    </div>
  );
}
