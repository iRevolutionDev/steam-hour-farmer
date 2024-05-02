use steamworks::Client;

async fn get_game_name(app_id: u32) -> String {
    const API_URL: &str = "https://store.steampowered.com/api/appdetails/?appids=";
    let url = format!("{}{}", API_URL, app_id);

    let response = reqwest::get(&url).await.unwrap();
    let json: serde_json::Value = response.json().await.unwrap();
    let game_name = json[&app_id.to_string()]["data"]["name"].as_str().unwrap();
    game_name.to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <app_id>", args[0]);
        std::process::exit(1);
    }

    let app_id = args[1].parse::<u32>().unwrap();
    Client::init_app(app_id)
        .unwrap_or_else(|err|
            match err {
                steamworks::SteamAPIInitError::NoSteamClient(_) => {
                    println!("Please run the steam client first");
                    std::process::exit(1);
                }
                _ => {
                    println!("Failed to initialize Steamworks: {:?}", err);
                    std::process::exit(1);
                }
            }
        );

    let game_name = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_game_name(app_id));

    println!("Running game: {}", game_name);
    loop {}
}
