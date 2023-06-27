use async_trait::async_trait;

pub mod google_translate;
pub use google_translate::GoogleTranslate;

#[async_trait]
pub trait Core<Args, Ret> {
    async fn execute(args: Args) -> Ret
    where
        Args: 'async_trait;
}
