use std::path::PathBuf;

use clap::Subcommand;

/// MCP server command for AI agent integration
#[derive(Subcommand, Debug)]
pub enum McpCommand {
    /// Start the MCP server for AI agent integration
    Serve {
        /// Directory containing INDEX.json (required)
        #[arg(value_name = "INDEX_DIR")]
        index_dir: PathBuf,
    },
}

impl McpCommand {
    /// Returns the index directory for the MCP command.
    #[must_use]
    pub fn index_dir(&self) -> &PathBuf {
        match self {
            McpCommand::Serve { index_dir } => index_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_serve_variant_index_dir() {
        let cmd = McpCommand::Serve {
            index_dir: PathBuf::from("/test/index"),
        };
        assert_eq!(cmd.index_dir(), &PathBuf::from("/test/index"));
    }

    #[test]
    fn test_mcp_serve_variant_is_serve() {
        let cmd = McpCommand::Serve {
            index_dir: PathBuf::from("/test"),
        };
        let McpCommand::Serve { .. } = cmd;
    }
}
