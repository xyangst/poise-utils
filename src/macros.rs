#[macro_export]
macro_rules! embed {
    ( $($key:ident : $value:expr),* $(,)?) => {
        {
            let embed =poise::serenity_prelude::CreateEmbed::default();
            $(let embed = embed.$key($value);)*
            embed
        }
    };
}

#[macro_export]
macro_rules! embed_footer {
( $($key:ident : $value:expr),* $(,)?) => {
    {
        let embed =poise::serenity_prelude::CreateEmbedFooter::new("");
        $(let embed = embed.$key($value);)*
        embed
    }
    };
}

#[macro_export]
macro_rules! message {
    ( $($key:ident : $value:expr),* $(,)?) => {
        {
            let mut embed =poise::serenity_prelude::CreateMessage::default();
            $(
                embed = embed.$key($value);
            )*
            embed
        }
    };
    }

/// Simple macro to deduplicate code. Can't be a function due to lifetime issues with `format_args`
#[macro_export]
macro_rules! full_command_name {
    ($ctx:expr) => {
        format_args!("{}{}", $ctx.prefix(), $ctx.command().qualified_name)
    };
}
