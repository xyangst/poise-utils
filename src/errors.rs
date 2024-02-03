use poise::serenity_prelude::Error;
///TODO dont use serenity error
type UtilsError = Error;
pub type UtilsResult<T> = Result<T, UtilsError>;
