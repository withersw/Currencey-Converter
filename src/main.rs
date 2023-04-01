use serde::{Deserialize, Serialize};
//use serde_json::Value;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};

//create response struct for the API
#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRate {
    result: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u64,
    time_last_update_utc: String,
    time_next_update_unix: u64,
    time_next_update_utc: String,
    base_code: String,
    conversion_rates: HashMap<String, f64>,
}

//Make API request and parse response
async fn get_exchange_rate(url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    
    //create new header map
    let mut headers = HeaderMap::new();

    //get API to ExchangeRate struct
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    let response = client.get(url).send().await?.text().await?;
    let exchange_rate: ExchangeRate = serde_json::from_str(&response)?;
    Ok(exchange_rate.conversion_rates["USD"])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut url = String::new();
    let mut valid_url = false;

    //get currency user wants exchange to
    println!("Available currencies: Canadian dollar, Chinese yuan renminbi");
    println!("Egyptian Pound, the euro, pound sterling");
    let mut currency = String::new();
    println!("Enter name of currency you want to trade to (type answer as shown above): ");

    //set value of url to API with indicated currency
    while !valid_url {
        if currency == "Canadian dollar" {
            url = "https://v6.exchangerate-api.com/v6/23cc9744cec206e6ab07b8d8/latest/CAD".to_string();
            valid_url = true;
        }
        else if currency == "Chinese yuan renminbi" {
            url = "https://v6.exchangerate-api.com/v6/23cc9744cec206e6ab07b8d8/latest/CNY".to_string();
            valid_url = true;
        }
        else if currency == "Egyption Pound" {
            url = "https://v6.exchangerate-api.com/v6/23cc9744cec206e6ab07b8d8/latest/EGP".to_string();
            valid_url = true;
        }
        else if currency == "the euro" {
            url = "https://v6.exchangerate-api.com/v6/23cc9744cec206e6ab07b8d8/latest/EUR".to_string();
            valid_url = true;
        }
        else if currency == "pound sterling" {
            url = "https://v6.exchangerate-api.com/v6/23cc9744cec206e6ab07b8d8/latest/GBP".to_string();
            valid_url = true;
        }
        else {
            //reset currency to empty string
            println!("Please enter a currency from the list above as it is written.");
            currency.clear();
            std::io::stdin().read_line(&mut currency)?;
            currency = currency.trim().to_string();
        }
    }
    
    let exchange_rate = get_exchange_rate(&mut url).await?;
    println!("1 of your currency = {} USD", exchange_rate);

    //get amount of money user has
    let mut dollar_amount_str = String::new();
    println!("How much money in USD do you have? ");
    std::io::stdin().read_line(&mut dollar_amount_str).unwrap();
    let money_amount:f64 = dollar_amount_str.trim().parse().unwrap();

    //convert amount of money in USD to new currency
    let amount_new_curr:f64 = money_amount / exchange_rate;
    let format_amount = format!("{:.2}", amount_new_curr);
    println!("You have {} in this currency", format_amount);
    Ok(())
}
