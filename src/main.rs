use std::io;
use std::io::Write;
use dotenv::dotenv;
use ureq;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Invite {
    r#type: i32,
    code: String,
    inviter: Option<Value>,
    expires_at: Option<String>,
    flags: Option<i32>,
    guild: Value,
    channel: Value,
}

#[derive(Deserialize)]
struct Inviter {
    id: String,
    username: String,
    global_name: Option<String>,
}

#[derive(Deserialize)]
struct Guild {
    id: String,
    name: String,
    splash: Option<String>,
    banner: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    verification_level: i32,
    vanity_url_code: Option<String>,
    nsfw_level: i32,
    nsfw: bool,
    premium_subscription_count: i32,
}

#[derive(Deserialize)]
struct Channel {
    id: String,
    r#type: i32,
    name: String,
}

fn check_token(token: &str) -> bool {
    let url = "https://discordapp.com/api/v9/users/@me";
    let resp = ureq::get(url)
        .set("authorization", &format!("Bot {}", token))
        .call();
    resp.is_ok()
}

fn get_invite() -> String {
    print!("inviteを入力: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
    input.trim().to_string().replace("https://discord.gg/", "").replace("https://discord.com/invite/", "")
}

fn get_link(id: &str, image_id: &str, image_type: &str) -> String {
    if image_id == "null" {
        "null".to_string()
    } else {
        let gif_url = format!("https://cdn.discordapp.com/{}/{}/{}.gif?size=4096", image_type, id, image_id);
        let png_url = format!("https://cdn.discordapp.com/{}/{}/{}.png?size=4096", image_type, id, image_id);

        let gif_resp = ureq::get(&gif_url).call();
        if gif_resp.is_ok() {
            gif_url
        } else {
            let png_resp = ureq::get(&png_url).call();
            if png_resp.is_ok() {
                png_url
            } else {
                "null".to_string()
            }
        }
    }
}

fn get_info(token: &str) {
    let invite = get_invite();
    println!("Invite: {}", invite);
    let url = format!("https://discord.com/api/v10/invites/{}", invite);
    let resp = ureq::get(&url)
        .set("authorization", &format!("Bot {}", token))
        .set("content-type", "application/json")
        .call();
    if let Ok(response) = resp {
        let response_text = response.into_string().unwrap();
        let info: Invite = serde_json::from_str(&response_text).expect("json読み込みエラー1");
        let mut inviter_print = String::new();
        if let Some(inviter) = info.inviter {
            let inviter: Inviter = serde_json::from_value(inviter).expect("json読み込みエラー2");
            inviter_print = format!("{},{},{}", inviter.username, inviter.global_name.unwrap_or("null".to_string()), inviter.id);
        }
        let guild: Guild = serde_json::from_value(info.guild).expect("json読み込みエラー3");
        let channel: Channel = serde_json::from_value(info.channel).expect("json読み込みエラー4");
        let r#type = info.r#type;
        let code = info.code;
        let expires_at = info.expires_at.unwrap_or("null".to_string());
        let flags = info.flags.unwrap_or(0);
        let server_id = guild.id;
        let server_name = guild.name;
        let server_splash = get_link(&server_id, &guild.splash.unwrap_or("null".to_string()), "splashes");
        let server_banner = get_link(&server_id, &guild.banner.unwrap_or("null".to_string()), "banners");
        let server_description = guild.description.unwrap_or("None".to_string());
        let server_icon = get_link(&server_id, &guild.icon.unwrap_or("null".to_string()), "icons");
        let server_verification_level = guild.verification_level;
        let server_vanity_url_code = guild.vanity_url_code.unwrap_or("None".to_string());
        let server_nsfw_level = guild.nsfw_level;
        let server_nsfw = guild.nsfw;
        let server_premium_subscription_count = guild.premium_subscription_count;
        let channel_id = channel.id;
        let channel_type = channel.r#type;
        let channel_name = channel.name;
        println!("Type: {}", r#type);
        println!("Code: {}", code);
        println!("Inviter: {}", inviter_print);
        println!("Expires at: {}", expires_at);
        println!("Flags: {}", flags);
        println!("Server ID: {}", server_id);
        println!("Server Name: {}", server_name);
        println!("Server Splash: {}", server_splash);
        println!("Server Banner: {}", server_banner);
        println!("Server Description: {}", server_description);
        println!("Server Icon: {}", server_icon);
        println!("Server Verification Level: {}", server_verification_level);
        println!("Server Vanity URL Code: {}", server_vanity_url_code);
        println!("Server NSFW Level: {}", server_nsfw_level);
        println!("Server NSFW: {}", server_nsfw);
        println!("Server Boost Count: {}", server_premium_subscription_count);
        println!("Channel ID: {}", channel_id);
        println!("Channel Type: {}", channel_type);
        println!("Channel Name: {}", channel_name);
    } else {
        println!("IDが正しくありません");
        eprint!("Error: {:?}", resp.unwrap_err());
    }
}

fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKENが設定されていません");
    if !check_token(&token) {
        eprintln!("トークンが正しくありません");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        std::process::exit(1);
    }
    loop {
        get_info(&token);
        println!();
    }
}