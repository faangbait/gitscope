
#[tokio::main]
async fn main() {
    let crab = octocrab::OctocrabBuilder::new()
        .app(
            octocrab::models::AppId(251845),
            jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("key.pem")).expect("Couldn't read encoding key")
        )
        .build()
        .expect("Could not get octocrab instance");

    
    let app = crab.current().app().await.expect("Could not get app");
    println!("{}", app.name);
    
    println!("{:?}",crab.repos("faangbait","fieldnotes").list_commits().send().await.expect(""));
}

