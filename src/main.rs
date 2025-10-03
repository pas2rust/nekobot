use nekobot::components::prelude::*;

#[tokio::main]
async fn main() -> MainWebDriverResult {
    NekoBot::chrome()
        .web_driver
        .launch()
        .await?
        .to("https://web.whatsapp.com/")
        .await?
        .find(Selector::css(".x6s0dn4.x1rg5ohu.x16dsc37.xxk0z11"))
        .await?
        .to("https://google.com")
        .await?
        .find(Selector::css(".gLFyf"))
        .await?
        ._loop()
        .await;

    Ok(())
}
