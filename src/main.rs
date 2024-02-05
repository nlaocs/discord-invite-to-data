use std::io;
use std::io::Write;
use dotenv::dotenv;
use ureq;

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
    input.trim().to_string()
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
        println!("{}", response_text);
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