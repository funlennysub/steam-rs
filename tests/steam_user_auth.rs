use steam_rs::Steam;

mod common;

#[tokio::test]
pub async fn authenticate_user_ticket() {
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
    let app_id = 12900; // Audiosurf
    let ticket = &std::env::var("STEAM_GAME_TICKET").unwrap_or(String::new());

    if !ticket.is_empty() {
        assert!(steam
            .authenticate_user_ticket(app_id, ticket.to_string()).send()
            .await
            .is_ok());
    }
}
