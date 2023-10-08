use mongodb::{Client, bson::doc, options::SelectionCriteria};
use dotenv;

pub async fn mongodb_status(client: Client) -> Result<String, mongodb::error::Error> {
    let db_name = dotenv::var("DB_NAME").unwrap();
    
    let result = client.database(&db_name).run_command(doc! {
        "hostInfo": 1
    }, SelectionCriteria::ReadPreference(mongodb::options::ReadPreference::Primary)).await;

    match result {
        Ok(document) => {
            Ok(document.to_string())
        },
        Err(err) => {
            Err(err)
        }
    }
}