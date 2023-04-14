# Simple Notion

Easy to use library to read Notion DataBase using the NotionAPI.

# Disclaimer

Please note that this library is not officially supported by the Notion team.

## Features

- [X] Read Data-Base

## Getting Started

### Read Data-Base

```rust
// Setup notion API client
let mut client = crate::client::NotionClient::default();
client.set_url("You data-base url");
client.set_token("Your token");

// Get the data-base (send a request and return the json result)
let data_base = client.get_data_table_sync().unwrap();

// Parser of the data-base
let parser = crate::parser::NotionResponseParser::new(data_base);
// Convert the json to a Vec<Vec<JsonValue>> to easier manipulation
let data_table = crate::notion_data_base::NotionDataBase::new(parser.parse_table());

// Parse an element base on the line and column names
let version = parser.parse_element(data_table.get(&parser, "Line Title", "Column Name").unwrap().1);

// Print the result
println!("Version of \"Line Title\": {:?}", version);
```
