use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecipeError {
    #[error("Recipe is not yours to delete")]
    RecipeNotDeletable,
    #[error("Website not supported")]
    RecipeImportedWebsiteNotSupported,
    #[error("Recipe not imported")]
    RecipeNotImported,
    #[error("Recipe not found")]
    RecipeNotFound,
    #[error("unknown error")]
    Unknown,
}
