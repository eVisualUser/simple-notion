# Simple Notion

Easy to use library to read Notion DataBase using the NotionAPI.

# Disclaimer

Please note that this library is not officially supported by the Notion team.

## Features

- [X] Read Data-Base

## Getting Started

### API Client

To start you need to get access to the API using the NotionClient.

```rust
let mut client = crate::client::NotionClient::default();

// Added the full URL of the database page:
client.set_url("https://www.notion.so/user-name/data-base-id?v=other");

// And you need to pass your integration token
client.set_token("integration-token");
```

### Get the DataBase

Now you have the access, and need to download the data-base.

```rust
// Get the Raw Json data-base
let data_base = client.get_table_sync().unwrap();
```

### Parse the data-base

You will convert each element to an enum named DataType.
To do it you need the NotionResponseParser

```rust
// Take the data_base as input
let parser = crate::parser::NotionResponseParser::new(data_base);
```

### Let's simplify

Now you have have the data-base and the parser.
So you can create a NotionDataBase.

```rust
let data_table = crate::notion_data_base::NotionDataBase::new(parser.parse_table());
```

### Access an element

Great! You have done the setup, let's use it.

```rust
// Getting the element.
let element = parser.parse_element(data_table.get(&parser, "MyLineTitle", "MyColumnName").unwrap().1);

// Priting the result
println!("My Element: {:?}", element);
```

### Some other useful functions

```rust
// Get the Vec<String> of all lines titles
data_table.get_line_list(&parser);

// Get the Vec<String> of all columns names
data_table.get_column_list(&parser);

// Parse the full data-base to a Vec<Vec<DataType>>
data_table.get_all(&parser)
```
