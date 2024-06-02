use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting parking shade bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, receive_location).await;
}

async fn receive_location(bot: Bot, msg: Message) -> ResponseResult<()> {
    log::info!("Received message from {:?}", msg.from());
    match msg.location() {
        Some(location) => {
            let lat = location.latitude;
            let long = location.longitude;
            let zoom = env::var("ZOOM").unwrap_or("18".to_string());
            let location = create_link(lat, long, zoom.parse().unwrap());
            bot.send_message(msg.chat.id, "Here is the shade map!")
                .await?;
            bot.send_message(msg.chat.id, location).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send me your location.")
                .await?;
        }
    }
    Ok(())
}

fn create_link(lat: f64, long: f64, zoom: i32) -> String {
    format!("https://shademap.app/@{},{},{}z", lat, long, zoom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_link() {
        let lat = 40.7128;
        let long = -74.0060;
        let zoom = 12;
        let expected_url = "https://shademap.app/@40.7128,-74.006,12z";
        assert_eq!(create_link(lat, long, zoom), expected_url);
    }
}
