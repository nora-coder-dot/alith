pub use llm_client::utils::splitting::{
    Separator, SeparatorGroup, TextSplit, TextSplitter, split_text_into_indices,
};

#[inline]
pub fn split_text(text: &str) -> Vec<String> {
    match TextSplitter::new().split_text(text) {
        Some(splits) => splits
            .iter()
            .map(|split| split.text().to_string())
            .collect(),
        None => vec![],
    }
}
