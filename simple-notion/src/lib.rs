pub mod client;
pub mod non_magic;
pub mod notion;
pub mod notion_data_base;
pub mod parser;

mod tests {
    #[tokio::test]
    pub async fn test_async_reqwest() {
        let mut csv = pro_csv::CSV::default();
        csv.load_from_file("debug.csv");
        for mut line in csv.into_iter() {
            let token = line.pop().unwrap();
            let url = line.pop().unwrap();
            let name = line.pop().unwrap();

            println!("Test: {}", name);
            println!("Url: {}", url);
            println!("Token: {}", token);

            let mut client = crate::client::NotionClient::default();
            client.set_url(&url);
            client.set_token(&token);

            let data_base = client.get_table().await.unwrap();

            let parser = crate::parser::NotionResponseParser::new(data_base);
            let data_table = crate::notion_data_base::NotionDataBase::new(parser.parse_table());

            let version =
                parser.parse_element(data_table.get(&parser, "Launcher 1", "Version").unwrap().1);

            println!("All lines: {:?}", data_table.get_line_list(&parser));
            println!("All columns: {:?}", data_table.get_column_list(&parser));

            println!("Version of \"Launcher 1\": {:?}", version);
        }
    }
}
