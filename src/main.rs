use std::error::Error;

use nekobot::components::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    NekoBot::chrome()
        .web_driver
        .launch()
        .await?
        .to("https://web.whatsapp.com/")
        .await?
        .wait(500)
        .await?
        .find(Selector::CSS(".x6s0dn4.x1rg5ohu.x16dsc37.xxk0z11"), None)
        .await?
        .to("https://google.com")
        .await?
        .wait(500)
        .await?
        .find(Selector::CSS(".gLFyf"), None)
        .await?
        .print()
        .message("Debug")
        .info()
        .await;

    Ok(())
}
