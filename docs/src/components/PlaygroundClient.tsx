// Client-side only component - imports zkenc-js
import React, { useState, useEffect } from "react";
import styles from "../pages/playground.module.css";
import { Sudoku, generator } from "@forfuns/sudoku";
import { encrypt, decrypt, getPublicInput } from "zkenc-js";

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
  const [mode, setMode] = useState<"encrypt" | "decrypt">("encrypt");
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
        <h1>🎮 Sudoku Witness Encryption Playground</h1>
        <p>
          Encrypt messages that can only be decrypted with a valid Sudoku
          solution
        </p>
      </div>

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
            <button onClick={handleGenerateRandom} className={styles.button}>
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

      {error && <div className={styles.error}>❌ {error}</div>}

      <div className={styles.info}>
        <h3>ℹ️ About This Demo</h3>
        <p>
          This playground demonstrates witness encryption using Sudoku puzzles.
          The message can only be decrypted by providing a valid solution to the
          puzzle.
        </p>
        <p>
          <strong>How it works:</strong>
        </p>
        <ul>
          <li>✅ Uses real zkenc-js library for encryption/decryption</li>
          <li>✅ Loads compiled Circom circuits</li>
          <li>✅ Verifies Sudoku solutions cryptographically</li>
          <li>
            ✅ Only valid witnesses (correct solutions) can decrypt the message
          </li>
        </ul>
        <p>
          <a href="/zkenc/docs/getting-started/zkenc-js">Learn more →</a>
        </p>
      </div>
    </div>
  );
}
