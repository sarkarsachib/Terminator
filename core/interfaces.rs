use async_trait::async_trait;

/// Canonical component interface.
///
/// Phase 1 implementation lives in `core/rust/src/interfaces.rs`.
#[async_trait]
pub trait Component: Send + Sync {
    fn name(&self) -> &'static str;

    async fn start(&self) -> anyhow::Result<()>;
}
