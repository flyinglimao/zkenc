pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/comparators.circom";

// Sudoku 9x9 Verifier
// Public input: puzzle (81 values) - the initial sudoku puzzle with 0s for empty cells
// Private input: solution (81 values) - the complete sudoku solution
template Sudoku() {
    signal input puzzle[81];      // Public input: initial puzzle
    signal input solution[81];    // Private input: complete solution
    
    // Helper signals for squared values
    signal solutionSquared[81];
    for (var i = 0; i < 81; i++) {
        solutionSquared[i] <== solution[i] * solution[i];
    }
    
    // Verify that the solution matches the puzzle where puzzle has values
    for (var i = 0; i < 81; i++) {
        // If puzzle[i] is not 0, then solution[i] must equal puzzle[i]
        // If puzzle[i] is 0, then solution[i] can be anything (1-9)
        // Check: (puzzle[i] == 0) OR (puzzle[i] == solution[i])
        // This is equivalent to: puzzle[i] * (puzzle[i] - solution[i]) == 0
        puzzle[i] * (puzzle[i] - solution[i]) === 0;
    }
    
    // Verify rows: each row contains 1-9 exactly once
    for (var row = 0; row < 9; row++) {
        var sum = 0;
        var sumSquares = 0;
        for (var col = 0; col < 9; col++) {
            var idx = row * 9 + col;
            sum += solution[idx];
            sumSquares += solutionSquared[idx];
        }
        // Sum should be 1+2+...+9 = 45
        sum === 45;
        // Sum of squares should be 1^2+2^2+...+9^2 = 285
        sumSquares === 285;
    }
    
    // Verify columns: each column contains 1-9 exactly once
    for (var col = 0; col < 9; col++) {
        var sum = 0;
        var sumSquares = 0;
        for (var row = 0; row < 9; row++) {
            var idx = row * 9 + col;
            sum += solution[idx];
            sumSquares += solutionSquared[idx];
        }
        sum === 45;
        sumSquares === 285;
    }
    
    // Verify 3x3 boxes: each box contains 1-9 exactly once
    for (var boxRow = 0; boxRow < 3; boxRow++) {
        for (var boxCol = 0; boxCol < 3; boxCol++) {
            var sum = 0;
            var sumSquares = 0;
            for (var i = 0; i < 3; i++) {
                for (var j = 0; j < 3; j++) {
                    var row = boxRow * 3 + i;
                    var col = boxCol * 3 + j;
                    var idx = row * 9 + col;
                    sum += solution[idx];
                    sumSquares += solutionSquared[idx];
                }
            }
            sum === 45;
            sumSquares === 285;
        }
    }
}

component main {public [puzzle]} = Sudoku();
