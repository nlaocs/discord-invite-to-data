use std::io;
use std::io::Write;
use dotenv::dotenv;
use ureq;
use serde::Deserialize;
use serde_json::Value;

/*{
    "type": 0,
    "code": "CMEqPXC9A5",
    "inviter": {
        "id": "302050872383242240",
        "username": "DISBOARD",
        "avatar": "67342a774a9f2d20d62bfc8553bb98e0",
        "discriminator": "2760",
        "public_flags": 589824,
        "premium_type": 0,
        "flags": 589824,
        "bot": true,
        "banner": null,
        "accent_color": null,
        "global_name": null,
        "avatar_decoration_data": null,
        "banner_color": null
    },
    "expires_at": null,
    "flags": 2,
    "guild": {
        "id": "1069405259002294313",
        "name": "\u307d\u304b\u307d\u304b",
        "splash": null,
        "banner": null,
        "description": null,
        "icon": "7c45935a1b420c5419cc3255fe8f95d3",
        "features": [
            "NEWS",
            "COMMUNITY",
            "MEMBER_VERIFICATION_GATE_ENABLED",
            "AUTO_MODERATION",
            "CHANNEL_ICON_EMOJIS_GENERATED",
            "SOUNDBOARD",
            "PREVIEW_ENABLED"
        ],
        "verification_level": 2,
        "vanity_url_code": null,
        "nsfw_level": 0,
        "nsfw": false,
        "premium_subscription_count": 0
    },
    "guild_id": "1069405259002294313",
    "channel": {
        "id": "1069438412576280699",
        "type": 0,
        "name": "\ud83d\udc4d\u3010bot\u30c1\u30e3\u30f3\u30cd\u30eb\u3011"
    }
} */

#[derive(Deserialize)]
struct Invite {
    r#type: i32,
    code: String,
    inviter: Option<Value>,
    expires_at: Option<String>,
    flags: i32,
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
        if let Some(inviter) = info.inviter {
            let inviter: Inviter = serde_json::from_value(inviter).expect("json読み込みエラー2");
            println!("Inviter: {},{},{}", inviter.username, inviter.global_name.unwrap_or("null".to_string()), inviter.id);
        }
        let guild: Guild = serde_json::from_value(info.guild).expect("json読み込みエラー3");
        let channel: Channel = serde_json::from_value(info.channel).expect("json読み込みエラー4");
        let r#type = info.r#type;
        let code = info.code;
        let expires_at = info.expires_at.unwrap_or("null".to_string());
        let flags = info.flags;
        let server_id = guild.id;
        let server_name = guild.name;
        let server_splash = guild.splash.unwrap_or("None".to_string());
        let server_banner = guild.banner.unwrap_or("None".to_string());
        let server_description = guild.description.unwrap_or("None".to_string());
        let server_icon = guild.icon.unwrap_or("None".to_string());
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
        println!("Server Premium Subscription Count: {}", server_premium_subscription_count);
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