use telebot::Bot;
use futures::stream::Stream;
use std::env;
 
// import all available functions
use telebot::functions::*;
 
fn main() {
    // Создаём бота
    let mut bot = Bot::new(&env::var("6398684315:AAHUZolIBxTKM5i0IU3QzuFVocn1-Vn2PVo").unwrap()).update_interval(200);
 
    // Register a reply command which answers a message
    let handle = bot.new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<empty>".into();
            }
 
            bot.message(msg.chat.id, text).send()
        })
        .for_each(|_| Ok(()));
 
    bot.run_with(handle);
}