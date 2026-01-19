mod request_cat_photo;
use anyhow::Result;
use dotenvy::dotenv;
use request_cat_photo::request_cat_photo;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting get a funny cat bot");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let bytes = request_cat_photo().await;
        
        match bytes {
            Ok(bytes) => {
                let cat_photo = InputFile::memory(bytes);
                println!("Message event");
                bot.send_photo(msg.chat.id, cat_photo).await?;
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
       
        Ok(())
    })
    .await;

    Ok(())
}
