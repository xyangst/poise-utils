use poise::{
    serenity_prelude::{CreateAttachment, CreateEmbed, Timestamp},
    CreateReply, ReplyHandle,
};

use crate::{embed, errors::UtilsResult, snowflake_to_timestamp};

pub trait IntoMessage
where
    Self: std::marker::Sized,
{
    fn into_message(self, builder: &mut CreateReply);
}
impl<T> IntoMessage for Option<T>
where
    T: IntoMessage,
{
    fn into_message(self, builder: &mut CreateReply) {
        if let Some(message) = self {
            message.into_message(builder);
        }
    }
}

impl IntoMessage for CreateEmbed {
    fn into_message(self, builder: &mut CreateReply) {
        builder.embeds.push(self);
    }
}

impl IntoMessage for String {
    fn into_message(self, builder: &mut CreateReply) {
        if let Some(content) = builder.content.as_mut() {
            content.push_str(&self);
        } else {
            builder.content = Some(self);
        };
    }
}
impl IntoMessage for &str {
    fn into_message(self, builder: &mut CreateReply) {
        if let Some(content) = builder.content.as_mut() {
            content.push_str(self);
        } else {
            builder.content = Some(self.to_owned());
        };
    }
}

impl IntoMessage for CreateAttachment {
    fn into_message(self, builder: &mut CreateReply) {
        builder.attachments.push(self);
    }
}

pub trait ContextMessenger<'a> {
    async fn send_message(self, message: impl IntoMessage) -> UtilsResult<ReplyHandle<'a>>;

    async fn send_reply(self, message: impl IntoMessage) -> UtilsResult<ReplyHandle<'a>>;

    async fn send_error(
        self,
        title: impl ToString,
        message: impl ToString,
    ) -> UtilsResult<ReplyHandle<'a>>;

    fn timestamp(self) -> u64;
}
impl<'a, U, E> ContextMessenger<'a> for poise::Context<'a, U, E> {
    async fn send_message(self, message: impl IntoMessage) -> UtilsResult<ReplyHandle<'a>> {
        let mut inner_builder = CreateReply::default();
        message.into_message(&mut inner_builder);
        self.send(inner_builder).await
    }
    async fn send_error(
        self,
        title: impl ToString,
        error: impl ToString,
    ) -> UtilsResult<ReplyHandle<'a>> {
        let mut inner_builder = CreateReply::default().reply(true);
        embed!(
            title:format!("Error: {}",title.to_string()),
            description:error.to_string(),
            timestamp:Timestamp::now(),
            color:0xFF3000
        )
        .into_message(&mut inner_builder);
        self.send(inner_builder).await
    }
    async fn send_reply(self, message: impl IntoMessage) -> UtilsResult<ReplyHandle<'a>> {
        let mut inner_builder = CreateReply::default().reply(true);
        message.into_message(&mut inner_builder);

        self.send(inner_builder).await
    }

    fn timestamp(self) -> u64 {
        let id = match self {
            poise::Context::Application(ctx) => ctx.id(),
            poise::Context::Prefix(ctx) => ctx.id(),
        };

        snowflake_to_timestamp(id)
    }
}

pub trait EditReply<'a, U, E> {
    // ReplyHandle<'_>
    async fn edit_message(
        &self,
        ctx: poise::Context<'a, U, E>,
        message: impl IntoMessage,
    ) -> UtilsResult<()>;
}
impl<'a, U, E> EditReply<'a, U, E> for ReplyHandle<'a> {
    async fn edit_message(
        &self,
        ctx: poise::Context<'a, U, E>,
        message: impl IntoMessage,
    ) -> UtilsResult<()> {
        let mut inner_builder = CreateReply::default();
        message.into_message(&mut inner_builder);

        self.edit(ctx, inner_builder).await
    }
}
