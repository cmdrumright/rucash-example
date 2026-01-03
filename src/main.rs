use rucash::{Book, SQLiteQuery, Error};
use rucash::model::SplitBuilder;
use chrono::NaiveDate;
use rust_decimal::prelude::*;
use futures::executor::block_on;

async fn create_buy() -> Result<(), Error> {
    let query = SQLiteQuery::new("tests/sqlite/complex_sample.gnucash").unwrap();
    let book = Book::new(query).await.unwrap();

    let usd = book.commodity_with_mnemonic("EUR").await?;

    let post_date = NaiveDate::from_ymd_opt(2025, 12, 25)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let description = "bought 1.234 for $200".to_string();

    let account_1 = book.account_with_fullname("Asset:Broker:Foo stock").await?;

    let account_2 = book.account_with_fullname("Asset:Current:Checking").await?;

    let splits = vec![
        SplitBuilder::new(
            account_1,
            Decimal::from_str("200.00").unwrap(), // 200 to buy
        )
        .with_qty(Decimal::from_str("1.234").unwrap()) // +1.234 shares  
        .with_memo(description.clone()),
        SplitBuilder::new(
            account_2,
            Decimal::from_str("-200.00").unwrap(), // 200 from checking
        )
    ];

    let _result = book.create_transaction(
        usd,
        "",
        &post_date,
        description.as_str(),
        splits
    ).await?;
    Ok(())
}

fn main() {
    block_on(
        async{ 
            match create_buy().await {
                Ok(_r) => println!("ok"),
                Err(e) => println!("{e}"),
            }
        }
    );
}
