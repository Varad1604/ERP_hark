//! Readiness probe port (ADR-L5-0044). Liveness never depends on downstream dependencies;
//! readiness aggregates [`ReadinessCheck`]s.

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub name: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[async_trait::async_trait]
pub trait ReadinessCheck: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    async fn check(&self) -> Result<(), String>;
}
