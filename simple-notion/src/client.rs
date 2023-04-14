#[derive(Default, Debug, Clone)]
pub struct NotionClient {
    pub url: crate::notion::NorionURL,
    pub token: String,
}

impl NotionClient {
    pub fn set_url(&mut self, url: &str) {
        self.url = crate::notion::NorionURL::new(url);
    }
    
    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
    
    pub async fn get_table(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let database_id = self.url.data_base_id.clone();
        
        let url = format!("{}{}{}", crate::non_magic::notion::NOTION_API_DATABASE_START_URL, database_id, crate::non_magic::notion::NOTION_API_DATABASE_END_URL);
        
        let response = client.post(url)
        .header(reqwest::header::AUTHORIZATION, &format!("Bearer {}", self.token))
        .header(crate::non_magic::notion::NOTION_API_VERSION_HEADER, crate::non_magic::notion::NOTION_API_VERSION_VALUE)
        .send()
        .await.unwrap();
        
        let text = response.text().await.unwrap();
              
        Ok(text)
    }
    
    pub fn get_data_table_sync(&self) -> Result<String, String> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_table())
    }
}
