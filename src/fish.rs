use async_graphql::{InputObject, SimpleObject, ID};

#[derive(serde::Deserialize, SimpleObject)]
pub struct FishD {
    pub fish_id: String,
    pub fish_type_id: String,
    pub name: String,
    pub anishinaabe_name: Option<String>,
    pub fish_image: Option<String>,
    pub woodland_fish_image: Option<String>,
    pub s3_fish_image: Option<String>,
    pub s3_woodland_image: Option<String>,
    pub mercury: Option<f32>,
    pub omega_3: Option<f32>,
    pub omega_3_ratio: Option<f32>,
    pub pcb: Option<f32>,
    pub protein: Option<f32>,
    pub lake: String,
    pub about: String,
}

#[derive(serde::Deserialize, SimpleObject)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub ingredients: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
}

#[derive(SimpleObject, serde::Deserialize)]
pub(crate) struct FishData {
    pub(crate) fish_data: FishD,
    pub(crate) recipe_data: Vec<Recipe>,
}

#[derive(InputObject)]
pub(crate) struct FishInput {
    pub(crate) id: ID,
}

pub(crate) async fn get_fish(id: String) -> Option<FishData> {
    let url = format!("{}/v1/fish/{}", std::env::var("HOST").unwrap(), id);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(std::env::var("APIKEY").unwrap())
        .send()
        .await
        .unwrap();
    let data = response.json::<FishData>().await.unwrap();

    Some(data)
}
