use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum ExtensionProvides {
    #[default]
    Themes,
    IconThemes,
    Languages,
    Grammars,
    LanguageServers,
    Snippets,
    DebugAdapters,
    AgentServers,
    IndexedDocsProviders,
    SlashCommands,
}

impl ExtensionProvides {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            ExtensionProvides::Themes,
            ExtensionProvides::IconThemes,
            ExtensionProvides::Languages,
            ExtensionProvides::Grammars,
            ExtensionProvides::LanguageServers,
            ExtensionProvides::Snippets,
            ExtensionProvides::DebugAdapters,
            ExtensionProvides::AgentServers,
            ExtensionProvides::IndexedDocsProviders,
            ExtensionProvides::SlashCommands,
        ]
        .iter()
        .copied()
    }

    pub fn is_deprecated(&self) -> bool {
        matches!(
            self,
            ExtensionProvides::AgentServers
                | ExtensionProvides::IndexedDocsProviders
                | ExtensionProvides::SlashCommands
        )
    }
}

impl std::fmt::Display for ExtensionProvides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ExtensionProvides::Themes => "themes",
            ExtensionProvides::IconThemes => "icon_themes",
            ExtensionProvides::Languages => "languages",
            ExtensionProvides::Grammars => "grammars",
            ExtensionProvides::LanguageServers => "language_servers",
            ExtensionProvides::Snippets => "snippets",
            ExtensionProvides::DebugAdapters => "debug_adapters",
            ExtensionProvides::AgentServers => "agent_servers",
            ExtensionProvides::IndexedDocsProviders => "indexed_docs_providers",
            ExtensionProvides::SlashCommands => "slash_commands",
        };
        write!(f, "{}", name)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExtensionMetadata {
    pub manifest: ExtensionApiManifest,
    pub id: Arc<str>,
    pub name: String,
    pub version: Arc<str>,
    pub download_count: u64,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExtensionApiManifest {
    pub id: String,
    pub name: String,
    pub version: Arc<str>,
    pub schema_version: Option<u32>,
    pub wasm_api_version: Option<String>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: Option<String>,
    pub provides: Vec<ExtensionProvides>,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GetExtensionsResponse {
    pub data: Vec<ExtensionMetadata>,
}
