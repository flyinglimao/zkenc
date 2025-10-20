import React, { useState, useEffect } from "react";
import Layout from "@theme/Layout";
import styles from "./playground.module.css";
import { Sudoku, generator } from "@forfuns/sudoku";

// Sudoku utilities
function generateSudoku(): { puzzle: number[] } {
  // Generate a random Sudoku puzzle using @forfuns/sudoku
  // Level 0: easy, 1: medium, 2: hard, 3: expert, 4: hell
  const puzzleString = generator(1); // Generate medium difficulty puzzle
  
  // Convert string format to number array
  // generator returns a string of 81 characters where '.' represents empty cells
  const puzzleArray = puzzleString.split("").map((char) => {
    return char === "." ? 0 : parseInt(char, 10);
  });

  return { puzzle: puzzleArray };
}

function solveSudoku(puzzle: number[]): { solution: number[] | null; solvable: boolean } {
  try {
    // Convert number array back to string format for the solver
    const puzzleString = puzzle.map((n) => (n === 0 ? "." : n.toString())).join("");
    
    // Create Sudoku solver instance and get solution
    const sudoku = new Sudoku(puzzleString);
    const solutionString = sudoku.getSolution();
    
    if (!solutionString || solutionString.includes(".")) {
      return { solution: null, solvable: false };
    }
    
    // Convert solution string to number array
    const solutionArray = solutionString.split("").map((char) => parseInt(char, 10));
    
    return { solution: solutionArray, solvable: true };
  } catch (error) {
    return { solution: null, solvable: false };
  }
}

export default function Playground(): React.ReactElement {
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

  useEffect(() => {
    // Initialize with a random sudoku
    handleGenerateRandom();
  }, []);

  const handleGenerateRandom = () => {
    const { puzzle: newPuzzle } = generateSudoku();
    setPuzzle(newPuzzle);
    setSolution([]); // Clear solution when generating new puzzle
    setError("");
  };

  const handleAutoSolve = () => {
    setLoading(true);
    setError("");

    try {
      const { solution: solvedPuzzle, solvable } = solveSudoku(puzzle);
      
      if (!solvable || !solvedPuzzle) {
        setError("❌ Unable to solve this puzzle. It may not have a valid solution.");
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

    setLoading(true);
    setError("");

    try {
      // In a real implementation, this would use zkenc-js
      // For demo, we'll simulate the encryption
      await new Promise((resolve) => setTimeout(resolve, 1000));

      const encoder = new TextEncoder();
      const messageBytes = encoder.encode(message);

      // Simulate ciphertext (in production, use zkenc.encrypt)
      const simulatedCiphertext = new Uint8Array([
        ...new Uint8Array([0, 0, 6, 40]), // length prefix
        ...new Uint8Array(1576).fill(0), // witness ciphertext
        ...messageBytes, // message
        ...new Uint8Array(28).fill(0), // AES overhead
      ]);

      setCiphertext(simulatedCiphertext);
      setError("");
    } catch (err) {
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

    // Verify solution
    const hasZeros = solution.some((v) => v === 0);
    if (hasZeros) {
      setError("Please fill in the complete solution");
      return;
    }

    setLoading(true);
    setError("");
    setDecrypted("");

    try {
      // In a real implementation, this would use zkenc-js
      // For demo, we'll simulate the decryption
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Simulate decryption (in production, use zkenc.decrypt)
      const messageStart = 4 + 1576;
      const messageEnd = ciphertextToUse.length - 28;
      const messageBytes = ciphertextToUse.slice(messageStart, messageEnd);
      const decoder = new TextDecoder();
      const decryptedMessage = decoder.decode(messageBytes);

      setDecrypted(decryptedMessage || "Decryption successful!");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Decryption failed");
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
      setUploadedCiphertext(new Uint8Array(arrayBuffer));
      setMode("decrypt");
    };
    reader.readAsArrayBuffer(file);
  };

  const renderSudokuGrid = (values: number[], editable: boolean) => {
    return (
      <div className={styles.sudokuGrid}>
        {values.map((value, index) => {
          const row = Math.floor(index / 9);
          const col = index % 9;
          const isGiven = mode === "decrypt" && puzzle[index] !== 0;

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
                  if (mode === "decrypt") {
                    setSolution(newValues);
                  } else {
                    setPuzzle(newValues);
                  }
                }
              }}
              maxLength={1}
              disabled={!editable || (mode === "decrypt" && isGiven)}
            />
          );
        })}
      </div>
    );
  };

  return (
    <Layout
      title="Playground"
      description="Interactive Sudoku Witness Encryption Playground"
    >
      <div className={styles.playground}>
        <div className={styles.header}>
          <h1>🎮 Sudoku Witness Encryption Playground</h1>
          <p>
            Encrypt messages that can only be decrypted by solving a Sudoku
            puzzle
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
                {renderSudokuGrid(puzzle, true)}
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
              <h2>2. View Puzzle</h2>
              <div className={styles.puzzleContainer}>
                {renderSudokuGrid(puzzle, false)}
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
                {renderSudokuGrid(solution.length ? solution : puzzle, true)}
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
            This playground demonstrates witness encryption using Sudoku
            puzzles. The message can only be decrypted by providing a valid
            solution to the puzzle.
          </p>
          <p>
            <strong>Note:</strong> This is a simplified demo. In production, you
            would:
          </p>
          <ul>
            <li>Use actual zkenc-js library for encryption/decryption</li>
            <li>Load compiled Circom circuits</li>
            <li>Verify Sudoku solutions cryptographically</li>
          </ul>
          <p>
            <a href="/docs/getting-started/zkenc-js">Learn more →</a>
          </p>
        </div>
      </div>
    </Layout>
  );
}
