use super::OlegCommand;
use crate::bot_command::core::*;
use async_trait::async_trait;
use openai::chat::*;
use std::sync::Arc;
use teloxide::{prelude::*, types::InputFile};
use tokio::sync::Mutex;

pub struct Draw;

pub struct Args<'a> {
    pub bot: &'a Bot,
    pub msg: &'a Message,
    pub sd_draw: Arc<Mutex<SdDraw>>,
    pub db: Arc<Mutex<crate::DB>>,
    pub description: &'a str,
    pub nsfw: bool,
}

#[async_trait]
impl<'a> OlegCommand<Args<'a>> for Draw {
    fn desc() -> ChatCompletionFunctionDefinition {
        ChatCompletionFunctionDefinition {
            name: "draw".to_owned(),
            description: Some("Draw image".to_owned()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "Description of an image"
                    },
                    "nsfw": {
                        "type": "boolean",
                        "nsfw": "Is description NSFW or not"
                    },
                },
                "required": ["description", "nsfw"],
            })),
        }
    }

    async fn execute(args: Args<'a>) -> Option<Message> {
        match SdDraw::execute(sd_draw::Args {
            instance: args.sd_draw.clone(),
            description: args.description,
            msg: args.msg,
        })
        .await
        {
            Ok(img) => {
                let answer = args
                    .bot
                    .send_photo(args.msg.chat.id, InputFile::memory(img))
                    .reply_to_message_id(args.msg.id)
                    .has_spoiler(args.nsfw)
                    .send()
                    .await;
                if let Ok(answer) = answer.as_ref() {
                    args.db
                        .lock()
                        .await
                        .add_caption(&answer, Some(args.description));
                }
                answer
            }
            Err(err) => {
                args.bot
                    .send_message(args.msg.chat.id, err)
                    .reply_to_message_id(args.msg.id)
                    .send()
                    .await
            }
        }
        .ok()
    }
}
