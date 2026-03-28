use std::path::Path;

use anyhow::{Result, bail};
use tree_sitter::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Rust,
    Cpp,
    Typescript,
    Tsx,
    Python,
    Protobuf,
}

impl Lang {
    pub fn detect(path: &Path) -> Result<Self> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext {
            "rs" => Ok(Self::Rust),
            "cpp" | "cc" | "cxx" | "hpp" | "h" => Ok(Self::Cpp),
            "ts" => Ok(Self::Typescript),
            "tsx" => Ok(Self::Tsx),
            "py" => Ok(Self::Python),
            "proto" => Ok(Self::Protobuf),
            _ => bail!("unsupported file extension: .{ext}"),
        }
    }

    pub fn tree_sitter_language(self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Self::Typescript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::Protobuf => tree_sitter_proto::LANGUAGE.into(),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Cpp => "cpp",
            Self::Typescript => "typescript",
            Self::Tsx => "tsx",
            Self::Python => "python",
            Self::Protobuf => "protobuf",
        }
    }

    /// Body placeholder for skeleton mode.
    pub fn body_placeholder(self) -> &'static str {
        match self {
            Self::Rust => "{ todo!() }",
            Self::Cpp => "{ /* ... */ }",
            Self::Typescript | Self::Tsx => "{ /* ... */ }",
            Self::Python => "...",
            Self::Protobuf => "{ /* ... */ }",
        }
    }
}

/// Extract the text of a tree-sitter node from the source.
pub fn node_text(node: tree_sitter::Node, source: &str) -> Result<String> {
    Ok(node.utf8_text(source.as_bytes())?.to_string())
}

/// Check if a tree-sitter node kind is a body container (declaration_list, block, etc.)
pub fn is_body_container(kind: &str) -> bool {
    matches!(
        kind,
        "declaration_list"
            | "field_declaration_list"
            | "block"
            | "class_body"
            | "interface_body"
            | "enum_body"
            | "message_body"
            | "service_body"
    )
}

/// Parse a file and return the tree + source text.
pub fn parse_file(path: &Path) -> Result<(tree_sitter::Tree, String, Lang)> {
    let lang = Lang::detect(path)?;
    let source = std::fs::read_to_string(path)?;
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&lang.tree_sitter_language())?;
    let tree = parser
        .parse(&source, None)
        .ok_or_else(|| anyhow::anyhow!("failed to parse {}", path.display()))?;
    Ok((tree, source, lang))
}
