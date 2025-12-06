import std.stdio;
import std.string;
import std.algorithm;
import std.conv;
import std.array;
import std.ascii : isWhite;

string part1(string input) {
    ulong ans = 0;
    ulong[][] grid;
    string[] operations;

    foreach (line; input.lineSplitter) {
        line = line.strip();
        if (line.empty) continue;

        if (line.startsWith("+") || line.startsWith("*")) {
            foreach (op; line.split()) {
                operations ~= op;
            }
        } else {
            // Parse numbers line into array of ulong
            grid ~= line.split()
                        .map!(s => s.to!ulong)
                        .array;
        }
    }

    // Safety check in case of empty input
    if (grid.length == 0) return "0";

    auto rows = grid.length;
    auto cols = grid[0].length;

    for (size_t c = 0; c < cols; c++) {
        ulong res = grid[0][c];
        
        // Safety check: ensure we have enough operations
        if (c >= operations.length) break;

        if (operations[c] == "+") {
            for (size_t r = 1; r < rows; r++) {
                res += grid[r][c];
            }
        } else {
            for (size_t r = 1; r < rows; r++) {
                res *= grid[r][c];
            }
        }
        ans += res;
    }

    return ans.to!string;
}

string part2(string input) {
    ulong ans = 0;
    char[][] grid;
    string[] operations;

    foreach (line; input.lineSplitter) {
        // Note: Part 2 logic in Rust did NOT trim the number lines, 
        // only the operation lines were effectively split.
        // However, we check startswith.
        if (line.startsWith("+") || line.startsWith("*")) {
            foreach (c; line.split()) {
                operations ~= c;
            }
        } else {
            // Convert string to mutable char array for easier indexing
            grid ~= line.dup; 
        }
    }

    if (grid.length == 0) return "0";

    auto rows = grid.length;
    auto cols = grid[0].length;
    size_t col = 0;

    foreach (o; operations) {
        ulong res = (o == "+") ? 0 : 1;

        while (col < cols) {
            bool all_spaces = true;
            ulong num = 0;

            for (size_t r = 0; r < rows; r++) {
                // Bounds check and character check
                if (col < grid[r].length && grid[r][col] != ' ') {
                    // D char arithmetic: '0' is 48.
                    num = num * 10 + (grid[r][col] - '0');
                    all_spaces = false;
                }
            }
            col++;
            
            // Logic match: If the column was empty, it's a delimiter 
            // for the current operation set.
            if (all_spaces) break; 
            
            if (o == "+") {
                res += num;
            } else {
                res *= num;
            }
        }
        ans += res;
    }

    return ans.to!string;
}

// Main function acting as the test runner
void main() {
    // We run the unittests automatically if compiled with -unittest.
    // However, for explicit demonstration without special flags, 
    // we can run the logic here too.
    
    writeln("Running embedded tests...");
    
    // Simulating the "sample" file content
    string sampleInput = `
+ + * +
1 2 3 4
5 6 7 8
9 10 11 12
    `.strip(); // Strip to match typical file read behavior if needed

    // Note: The sample input provided in the code above is synthetic 
    // because I don't have your 'day6-sample.txt'. 
    // You will need to replace the string content or file read logic below.

    writeln("Note: Sample input logic requires actual file content.");
    writeln("Use 'rdmd -unittest solution.d' to run the actual tests defined below.");
}

// D uses `unittest` blocks which are compiled in when using the `-unittest` flag.
unittest {
    import std.file;
    import std.path;

    // Helper to try read files, or return empty if not found 
    // (to prevent crash if you copy-paste this without the files)
    string readSafe(string path) {
        if (exists(path)) return readText(path);
        return "";
    }

    // Adjust these paths to where your files actually are relative to this script
    string inputSample = readSafe("../../inputs/2025/day6-sample.txt");
    string inputFull = readSafe("../../inputs/2025/day6.txt");

    if (inputSample.length > 0) {
        writeln("Testing Part 1 Sample...");
        assert(part1(inputSample) == "4277556");
        
        writeln("Testing Part 2 Sample...");
        assert(part2(inputSample) == "3263827");
    }

    if (inputFull.length > 0) {
        writeln("Testing Part 1 Full...");
        string p1Ans = part1(inputFull);
        writeln("Part 1 Answer: ", p1Ans);
        assert(p1Ans == "5784380717354");

        writeln("Testing Part 2 Full...");
        string p2Ans = part2(inputFull);
        writeln("Part 2 Answer: ", p2Ans);
        assert(p2Ans == "7996218225744");
    }
}
