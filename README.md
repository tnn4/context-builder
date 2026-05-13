Context Builder

A high-performance file aggregation pipeline designed to transform a directory of source code into a single, structured text file. This tool is optimized for creating context for Large Language Models (LLMs), code audits, or systemic analysis.
### The Architecture

The tool consists of two parts:

aggregate (Rust): The engine. It handles recursive directory traversal, prevents self-consumption loops via path canonicalization, and performs atomic file writes.

build.sh (Bash): The orchestrator. It manages the user interface, generates a visual directory tree using eza or tree, and handles temporary file cleanup.

## Features

Safety First: Automatically detects if the output file is within the search path to prevent infinite recursive growth.

Visual Structure: Prepends a directory tree to the output for immediate structural context.

Machine Readable: Wraps file contents in <file path="..."> tags for easy parsing.

Flexible Extensions: Supports multiple target extensions in a single pass.

Resilient: Falls back to standard system tools if modern alternatives (like eza) are missing.

## Prerequisites

Rust: To compile the core engine.

Bash: To run the orchestration script.

Optional: eza for a modernized directory tree view.

## Installation

Compile the engine:
Bash

`rustc aggregate.rs -o aggregate`

Make the script executable:

Bash

`chmod +x build-context.sh`

## Usage

The build.sh script is the primary entry point. It requires a target directory and accepts optional extension flags.
### Basic Usage (Defaults to .txt)
Bash

.`/build-context.sh ./my_project`

### Targeted Usage

Specify one or more extensions to include in the aggregation:
Bash

`./build-context.sh ./src -e lua -e fs -e rs`

### Output

The tool generates a context.txt file in the current directory with the following structure:
Plaintext
```
<begin tree>
.
├── src
│   ├── main.lua
│   └── utils.fs
└── README.md
<end tree>

<file path="src/main.lua">
-- File contents here...
</file>

<file path="src/utils.fs">
// File contents here...
</file>
```
## Error Handling

Missing Directory: The script will exit immediately if the provided path is invalid.

No Extensions: If no -e flags are provided, the tool defaults to searching for .txt files.

Missing Binary: If aggregate hasn't been compiled, the script will alert you and exit before attempting a partial build.

## Technical Specifications

Traversal: Depth-first recursive search.

Memory Profile: Constant memory overhead (streams file content to disk).

Conflict Resolution: Uses fs::canonicalize to ensure the source and destination are not the same physical file on the disk.
