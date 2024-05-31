use scraper::{selectable::Selectable, Html, Selector};

const URL:&str = "https://www.bcv.org.ve/tasas-informativas-sistema-bancario";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get(URL)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let dolar_selector = Selector::parse("div#dolar")?;
    let results = document.select(&dolar_selector);

    if let Some(dolar_block) = results.into_iter().next() {
        let strong_selector = Selector::parse("strong")?;
        let price_selector = dolar_block.select(&strong_selector);
        if let Some(el) = price_selector.into_iter().next() {
            println!("{}", el.inner_html())
        }
    }
    Ok(())
}