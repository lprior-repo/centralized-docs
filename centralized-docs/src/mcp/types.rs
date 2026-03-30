use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ToolContent {
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

impl ToolResult {
    pub fn text(text: impl Into<String>) -> Self {
        ToolResult {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }],
            is_error: false,
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        ToolResult {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }],
            is_error: true,
        }
    }

    #[must_use]
    pub fn text_content(&self) -> Option<&str> {
        self.content.first().map(|c| c.text.as_str())
    }
}

// --- Domain Newtypes (Parse, Don't Validate) ---

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "String")]
pub struct ValidQuery(String);

impl ValidQuery {
    pub fn parse(s: &str) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err("Query cannot be empty".to_string());
        }
        if s.len() > 1024 {
            return Err("Query exceeds maximum length of 1024 bytes".to_string());
        }
        Ok(Self(s.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ValidQuery {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(&s)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "u32")]
pub struct ValidLimit(u32);

impl ValidLimit {
    pub fn parse(limit: u32) -> Result<Self, String> {
        if limit == 0 {
            return Err("Limit must be greater than 0".to_string());
        }
        if limit > 1000 {
            return Err("Limit exceeds maximum of 1000".to_string());
        }
        Ok(Self(limit))
    }

    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for ValidLimit {
    type Error = String;
    fn try_from(limit: u32) -> Result<Self, Self::Error> {
        Self::parse(limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "String")]
pub struct ValidId(String);

impl ValidId {
    pub fn parse(s: &str, param_name: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Err(format!("{param_name} cannot be empty"));
        }
        if s.len() > 256 {
            return Err(format!("{param_name} exceeds maximum length of 256 bytes"));
        }
        if s.contains(char::is_whitespace)
            || s.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_' && c != '.')
        {
            return Err(format!("{param_name} contains invalid characters"));
        }
        Ok(Self(s.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ValidId {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::parse(&s, "ID")
    }
}

// --- Parameter Models ---

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "SearchDocsParamsRaw")]
pub struct SearchDocsParams {
    pub query: String,
    pub limit: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SearchDocsParamsRaw {
    query: String,
    limit: Option<u32>,
}

impl TryFrom<SearchDocsParamsRaw> for SearchDocsParams {
    type Error = String;
    fn try_from(raw: SearchDocsParamsRaw) -> Result<Self, Self::Error> {
        let q = ValidQuery::parse(&raw.query)?;
        let l = ValidLimit::parse(raw.limit.unwrap_or(10))?;
        Ok(Self {
            query: q.as_str().to_string(),
            limit: l.as_u32(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "ReadChunkParamsRaw")]
pub struct ReadChunkParams {
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ReadChunkParamsRaw {
    id: String,
}

impl TryFrom<ReadChunkParamsRaw> for ReadChunkParams {
    type Error = String;
    fn try_from(raw: ReadChunkParamsRaw) -> Result<Self, Self::Error> {
        let id = ValidId::parse(&raw.id, "Chunk ID")?;
        Ok(Self {
            id: id.as_str().to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "GetRelatedConceptsParamsRaw")]
pub struct GetRelatedConceptsParams {
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetRelatedConceptsParamsRaw {
    id: String,
}

impl TryFrom<GetRelatedConceptsParamsRaw> for GetRelatedConceptsParams {
    type Error = String;
    fn try_from(raw: GetRelatedConceptsParamsRaw) -> Result<Self, Self::Error> {
        let id = ValidId::parse(&raw.id, "Concept ID")?;
        Ok(Self {
            id: id.as_str().to_string(),
        })
    }
}
