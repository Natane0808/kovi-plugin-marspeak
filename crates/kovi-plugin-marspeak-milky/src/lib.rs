use std::sync::Arc;

use kovi::{PluginBuilder as P, utils::load_toml_data};
use kovi_milky::MsgEvent;
use marspeak::{Builder, Marspeak};

use crate::config::{Config, DEFAULT_CONFIG_PATH};
mod config;

#[kovi::plugin]
async fn main() {
    let config = Arc::new({
        let bot = P::get_runtime_bot();
        let path = bot.get_data_path().join(DEFAULT_CONFIG_PATH);
        load_toml_data(Config::default(), path).expect("加载配置文件失败")
    });

    let marspeak = Arc::new(Builder::new().build().unwrap());
    P::on(move |e| msg_handle(e, marspeak.clone(), config.clone()));
}

async fn msg_handle(e: Arc<MsgEvent>, marspeak: Arc<Marspeak<'static>>, config: Arc<Config>) {
    let Some(msg) = e.borrow_text() else {
        return;
    };

    // 没有命令前缀就不处理，避免在普通聊天里被误触。
    let Some(rest) = msg.strip_prefix(config.prefix.as_str()) else {
        return;
    };

    let trimmed = rest.trim();
    let result = if let Some(args) = trimmed.strip_prefix("encode") {
        marspeak.encode(args.trim())
    } else if let Some(args) = trimmed.strip_prefix("decode") {
        marspeak.decode(args.trim())
    } else {
        let reply = format!("请执行正确的操作：{} <encode|decode> 内容", config.prefix);
        e.reply(reply);
        return;
    };
    e.reply(result);
}
