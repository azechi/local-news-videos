use serde::de::DeserializeOwned;
use worker::*;

pub async fn fetch<T>(req: Request) -> T 
    where 
        T: DeserializeOwned
{
    let mut res = Fetch::Request(req).send().await.unwrap();
    res.json::<T>().await.unwrap()
}
