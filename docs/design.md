# ast-cli -- Design

Last updated: 2026-03-26

## 1. Problem

AI coding agents hit three walls when working with large, multi-language codebases:

### 1.1 The Read-Tool Wall

The agent's `Read` tool has a line limit. A 5,000-line C++ file or a 3,000-line
TypeScript module simply cannot be loaded in one shot. The agent either truncates
(losing context) or reads in chunks (wasting turns and context window).

### 1.2 The Structural Blindness Problem

Even when a file fits, the agent sees **text**, not **structure**. It cannot answer
"what functions are in this file?" or "what are the fields of this struct?" without
reading the entire file and mentally parsing it. For a 2,000-line file where the
agent needs one function signature, this wastes 1,950 lines of context.

### 1.3 The Throwaway Script Problem

When agents need structural information (find all test cases, extract function
signatures, list class hierarchies), they write **throwaway Python scripts** --
ad-hoc parsers using regex or `ast` module. These scripts:
- Waste 2-3 turns to write, debug, and run
- Are fragile (regex-based) or language-specific (Python `ast` only parses Python)
- Are discarded after use, never reused
- Often fail on edge cases (nested classes, generics, decorators)

### 1.4 The Multi-Language Gap

Existing tool (`ast-agent`) solves problems 1.1-1.3 for **Rust only** using the `syn`
crate. But real projects span multiple languages. A typical repo might have:
- Rust services with protobuf definitions
- TypeScript frontend with Python ML pipelines
- C++ performance-critical libraries

The agent needs a **single tool** that handles all of these.

## 2. Solution

A multi-language structural code navigation CLI built on **tree-sitter**. The agent
calls `ast-cli` instead of writing throwaway parsers. It gets structured JSON about
any supported file in one command.

**Supported languages:** C++, Rust, TypeScript/TSX, Python, Protobuf

**Key principle:** tree-sitter provides a universal, language-agnostic parse tree.
We define per-language **queries** to extract the same structural concepts (functions,
types, imports, tests) across all languages.

## 3. Architecture

```
Agent
  |
  | ast-cli outline src/parser.cpp
  | ast-cli skeleton src/parser.cpp
  | ast-cli read src/parser.cpp "Parser::parse_field"
  |
  v
ast-cli
  |
  |-- tree-sitter core: incremental parsing
  |-- tree-sitter-{cpp,rust,typescript,python,proto}: grammars
  |-- per-language query definitions (.scm files)
  |
  v
JSON / text output to stdout
```

Single Rust binary. No runtime dependencies. All grammars compiled in.

## 4. Operations

### 4.1 `outline` -- Structural map of a file

**Purpose:** Answer "what's in this file?" without reading it.

```
ast-cli outline src/parser.cpp
ast-cli outline --format json src/parser.cpp
```

Returns every top-level definition with:
- Kind (function, class, struct, enum, trait, interface, message, service, ...)
- Name
- Line range (start:end)
- Visibility (pub/private/protected where applicable)
- Signature (parameters, return type)
- Nesting (methods under classes, fields under structs)
- Test marker (is this a test function?)

This replaces: `list` from ast-agent, but for all languages.

### 4.2 `skeleton` -- File with bodies stripped

**Purpose:** See the full structure (imports, type definitions, function signatures)
without function bodies. Typical 80-90% size reduction.

```
ast-cli skeleton src/parser.cpp
```

Returns the file with function/method bodies replaced by `/* ... */` or `todo!()` or
`pass` (language-appropriate placeholder). Preserves:
- All imports/includes
- All type definitions (structs, enums, classes) with fields
- All function/method signatures
- Comments on type/function declarations

This is the most powerful operation for the agent -- it turns a 2,000-line file into
a 200-line structural overview that fits easily in context.

### 4.3 `read` -- Extract a specific definition

**Purpose:** Read one function, class, struct, or enum without loading the whole file.

```
ast-cli read src/parser.cpp "Parser::parse_field"
ast-cli read src/parser.cpp 150:200
```

Two addressing modes:
- **By name:** `Parser::parse_field`, `validate`, `MyEnum`
- **By line range:** `150:200` (extract lines 150-200, snapped to nearest node boundaries)

Returns the original source text, preserving formatting and comments.

### 4.4 `find` -- Cross-file search by symbol name

**Purpose:** Find where a symbol is defined across a project.

```
ast-cli find . "Parser"
ast-cli find . "parse_field" --kind function
```

Searches all supported files under the given directory. Returns file path, line
number, kind, and signature for each match.

### 4.5 `query` -- Run a tree-sitter query

**Purpose:** Power-user escape hatch. Run an arbitrary tree-sitter S-expression
query against a file.

```
ast-cli query src/parser.cpp '(function_definition name: (identifier) @name)'
```

Returns all captures with their text and positions. This replaces the throwaway
Python scripts agents write -- any structural question can be answered with a query.

### 4.6 `project` -- Project-wide summary

**Purpose:** First step -- understand what's in a directory.

```
ast-cli project . --exclude testdata/ --exclude node_modules/
```

Returns per-file summary: language, line count, counts of functions/classes/structs,
test count. Helps the agent decide which files to explore further.

## 5. Language Detection

File extension based:
- `.rs` -> Rust
- `.cpp`, `.cc`, `.cxx`, `.hpp`, `.h` -> C++
- `.ts`, `.tsx` -> TypeScript/TSX
- `.py` -> Python
- `.proto` -> Protobuf

## 6. Addressing Scheme

Universal cross-language addressing:

```
# Top-level items
Parser                    -- class/struct/enum/trait/message
validate_field            -- free function
MAX_DEPTH                 -- constant

# Nested items (:: separator, universal)
Parser::new               -- method
Parser::field_count       -- field or property
MyService::GetUser        -- RPC method in protobuf service

# Trait/interface impls (+ separator)
Parser+Display::fmt       -- Rust trait impl
Parser+Serializable::serialize  -- interface impl
```

The `::` separator works naturally for C++ (native) and maps cleanly to `.` in
Python/TS and `::` in Rust. We use `::` as the canonical form in the CLI and
handle the mapping per-language internally.

## 7. Output Formats

- **text** (default): Human-readable, suitable for direct context injection
- **json**: Machine-readable, for programmatic consumption

All operations support `--format json` for structured output.

## 8. Key Design Decisions

### 8.1 Tree-sitter over language-specific parsers

`ast-agent` uses `syn` for Rust -- deep, precise, but locked to one language.
`ast-cli` uses tree-sitter because:
- One parsing framework for all languages
- Grammars are mature and battle-tested (used by GitHub, Neovim, Zed)
- Incremental parsing (future: watch mode, LSP-like server)
- Query language is expressive and declarative

Trade-off: tree-sitter gives a CST (concrete syntax tree), not an AST. We lose
some semantic precision (e.g., can't resolve types) but gain universality.

### 8.2 CLI-first, not JSON-protocol

`ast-agent` uses a JSON request/response protocol (`ast exec request.json`).
`ast-cli` uses direct CLI subcommands (`ast-cli outline file.rs`). Reasons:
- Agents call CLI tools more naturally than writing JSON files
- Subcommands are self-documenting (`--help`)
- Easier for humans to use directly
- JSON output is available via `--format json` when needed

### 8.3 Read-only

Same as ast-agent: never modify files. The tool's job is navigation and
understanding. Agents use Edit/Write tools for modifications.

### 8.4 Compiled-in grammars

All tree-sitter grammars are compiled into the binary. No need to download or
install grammars at runtime. The binary is self-contained.

### 8.5 Relationship to ast-agent

`ast-cli` is a **successor** for multi-language use. `ast-agent` remains useful
for Rust-specific deep analysis (it has richer Rust semantics via `syn`). For
cross-language work, `ast-cli` is the tool to use.

## 9. Agent Workflow

```bash
# 1. Understand project structure
$ ast-cli project /path/to/project --exclude node_modules/ --exclude target/
# -> 87 files across 4 languages, per-file summary

# 2. Get skeleton of a large file
$ ast-cli skeleton src/engine/parser.cpp
# -> 180 lines instead of 1,800 (90% reduction)

# 3. Find all definitions in a file
$ ast-cli outline src/engine/parser.cpp
# -> 47 items: 3 classes, 28 methods, 8 functions, 8 enums

# 4. Read a specific function
$ ast-cli read src/engine/parser.cpp "Parser::parse_field"
# -> original source, ~35 lines

# 5. Find where a type is defined across the project
$ ast-cli find . "AppState"
# -> src/state.rs:42 struct AppState (pub)
# -> src/types.ts:15 interface AppState (export)

# 6. Custom structural query (replaces throwaway Python scripts)
$ ast-cli query tests/test_parser.py '(function_definition name: (identifier) @name (#match? @name "^test_"))'
# -> test_parse_basic:12, test_parse_nested:45, test_parse_error:78
```

## 10. Implementation Plan

### Phase 1: Core + Rust (parity with ast-agent)
- Project scaffolding, tree-sitter integration
- Rust grammar: outline, skeleton, read
- JSON + text output

### Phase 2: All languages
- C++, TypeScript, Python, Protobuf grammars
- Per-language query definitions for outline/skeleton
- Language detection

### Phase 3: Cross-file operations
- `find` across directories
- `project` summary
- Exclude patterns

### Phase 4: Advanced
- `query` operation (raw tree-sitter queries)
- Line-range addressing in `read`
- Performance optimization for large projects

## 11. Tech Stack

- **Language:** Rust
- **Parsing:** tree-sitter 0.26.x
- **Grammars:** tree-sitter-{rust, cpp, typescript, python} (official), tree-sitter-proto
- **CLI:** clap v4
- **Serialization:** serde + serde_json
- **Error handling:** anyhow
- **Binary name:** `ast-cli` (installed to `~/.cargo/bin/ast-cli`)
