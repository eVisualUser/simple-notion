#[derive(Default, Debug, Clone)]
pub struct NotionClient {
    pub url: crate::notion::NotionURL,
    pub token: String,
}

impl NotionClient {
    pub fn set_url(&mut self, url: &str) {
        self.url = crate::notion::NotionURL::new(url);
    }
    
    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
    
    pub async fn get_table(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let database_id = self.url.data_base_id.clone();
        
        let url = format!("{}{}{}", crate::non_magic::notion::NOTION_API_DATABASE_START_URL, database_id, crate::non_magic::notion::NOTION_API_DATABASE_END_URL);
        
        match client.post(url)
        .header(reqwest::header::AUTHORIZATION, &format!("Bearer {}", self.token))
        .header(crate::non_magic::notion::NOTION_API_VERSION_HEADER, crate::non_magic::notion::NOTION_API_VERSION_VALUE)
        .send()
        .await {
            Ok(response) => {
                Ok(match response.text().await {
                    Ok(text) => text,
                    Err(error) => {
                        return Err(error.to_string());
                    }
                })
            }
            Err(error) => {
                Err(error.to_string())
            }
        }
    }
    
    pub fn get_data_table_sync(&self) -> Result<String, String> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_table())
    }

    pub async fn set_data_base_element(&self, line: &str, column: &str, element: crate::notion_data_base::DataType) {
        let value = element.to_json(column);
        let mut cmd = format!("\"filter\": {{
            \"property\": \"{}\",
            \"title\": {{
                \"equals\": \"New Record\"
            }}
        }},", column);

        cmd.push_str(&format!("\"properties\": {{ {} }}", value));
        cmd = format!("{{{}}}", cmd); 

        println!("Out: {}", cmd);

        let client = reqwest::Client::new();
        let database_id = self.url.data_base_id.clone();

        let url = format!("{}{}{}", crate::non_magic::notion::NOTION_API_DATABASE_START_URL, database_id, crate::non_magic::notion::NOTION_API_DATABASE_END_URL);

        match client.post(url)
        .header(reqwest::header::AUTHORIZATION, &format!("Bearer {}", self.token))
        .header(crate::non_magic::notion::NOTION_API_VERSION_HEADER, crate::non_magic::notion::NOTION_API_VERSION_VALUE)
        .header("Content-Type", "Content-Type: application/json")
        .body(cmd).send().await {
            Ok(response) => {
                println!("{}", response.text().await.unwrap());
            }
            Err(error) => {
                println!("{}", error.to_string());
            }
        }
    }
}
