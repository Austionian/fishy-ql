use async_graphql::{SimpleObject, ID};

#[derive(serde::Deserialize, SimpleObject)]
pub(crate) struct Fish {
    pub fish_id: ID,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub about: String,
    pub mercury: Option<f64>,
    pub omega_3: Option<f64>,
    pub omega_3_ratio: Option<f64>,
    pub pcb: Option<f64>,
    pub protein: Option<f64>,
}

pub(crate) async fn get_fishs() -> Option<Vec<Fish>> {
    let url = format!("{}/v1/fish_avgs", std::env::var("HOST").unwrap());
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(std::env::var("APIKEY").unwrap())
        .send()
        .await
        .unwrap();
    let data = response.json::<Vec<Fish>>().await.unwrap();

    Some(data)
}
