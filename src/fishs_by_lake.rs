use std::{fmt::Display, str::FromStr};

use crate::fishs::Fish;

pub(crate) enum Lakes {
    Michigan,
    Huron,
    Superior,
    Store,
}

impl FromStr for Lakes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Michigan" | "michigan" => Ok(Lakes::Michigan),
            "Huron" | "huron" => Ok(Lakes::Huron),
            "Superior" | "superior" => Ok(Lakes::Superior),
            "Store" | "store" => Ok(Lakes::Store),
            _ => Err(()),
        }
    }
}

impl Display for Lakes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Michigan => write!(f, "Michigan"),
            Self::Huron => write!(f, "Huron"),
            Self::Superior => write!(f, "Superior"),
            Self::Store => write!(f, "Store"),
        }
    }
}

pub(crate) async fn get_fishs_by_lake(lake: String) -> Option<Vec<Fish>> {
    match lake.parse::<Lakes>() {
        Ok(lake) => {
            let url = format!("{}/v1/fishs?lake={}", std::env::var("HOST").unwrap(), lake);
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
        Err(_) => None,
    }
}
