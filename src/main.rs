use scraper::{selectable::Selectable, Html, Selector};

const URL: &str = "https://www.bcv.org.ve/tasas-informativas-sistema-bancario";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = fetch_html().await?;
    if let Some(currency) = retrieves_price(&document, "dolar")? {
        println!("1 USD = {} Bs.s", currency);
    }

    if let Some(currency) = retrieves_price(&document, "euro")? {
        println!("1 EUR = {} Bs.s", currency);
    }
    Ok(())
}

async fn fetch_html() -> Result<Html, Box<dyn std::error::Error>> {
    let html = reqwest::get(URL).await?.text().await?;

    let document = Html::parse_document(&html);
    Ok(document)
}

fn retrieves_price(
    document: &Html,
    currency: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let currency_request = format!("div#{}", currency);
    let Ok(currency_selector) = Selector::parse(&currency_request) else {
        return Err("Invalid currency selector".into());
    };
    let results = document.select(&currency_selector);

    if let Some(dolar_block) = results.into_iter().next() {
        let strong_selector = Selector::parse("strong")?;
        let price_selector = dolar_block.select(&strong_selector);
        if let Some(el) = price_selector.into_iter().next() {
            return Ok(Some(el.inner_html()));
        }
    }
    Ok(None)
}
