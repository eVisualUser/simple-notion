pub mod notion {
    pub const NOTION_JSON_RESULTS: &str = "results";
    pub const NOTION_JSON_PROPERTIES: &str = "properties";

    pub const NOTION_TYPE_TITLE: &str = "title";
    pub const NOTION_TYPE_FILES: &str = "files";
    pub const NOTION_TYPE_SELECT: &str = "select";
    pub const NOTION_TYPE_MULTI_SELECT: &str = "multi_select";
    pub const NOTION_TYPE_NUMBER: &str = "number";
    pub const NOTION_TYPE_CHECKBOX: &str = "checkbox";
    pub const NOTION_TYPE_URL: &str = "url";
    pub const NOTION_TYPE_EMAIL: &str = "email";
    pub const NOTION_TYPE_STATUS: &str = "status";
    pub const NOTION_TYPE_PEOPLE: &str = "people";
    pub const NOTION_TYPE_CREATED_BY: &str = "created_by";
    pub const NOTION_TYPE_FORMULA: &str = "formula";
    pub const NOTION_TYPE_PHONE_NUMBER: &str = "phone_number";
    pub const NOTION_TYPE_RELATION: &str = "relation";
    pub const NOTION_TYPE_DATE: &str = "date";
    pub const NOTION_TYPE_ROLLUP: &str = "rollup";
    pub const NOTION_TYPE_CREATED_TIME: &str = "created_time";
    pub const NOTION_TYPE_LAST_EDITED_TIME: &str = "last_edited_time";
    pub const NOTION_TYPE_LAST_EDITED_BY: &str = "last_edited_by";

    pub const NOTION_KEYWORD_ID: &str = "id";
    pub const NOTION_KEYWORD_NAME: &str = "name";
    pub const NOTION_KEYWORD_TEXT: &str = "text";
    pub const NOTION_KEYWORD_CONTENT: &str = "content";
    pub const NOTION_KEYWORD_FILE: &str = "file";
    pub const NOTION_KEYWORD_URL: &str = "url";
    pub const NOTION_KEYWORD_EXTERNAL: &str = "external";

    pub const NOTION_KEYWORD_START: &str = "start";
    pub const NOTION_KEYWORD_END: &str = "end";
    pub const NOTION_KEYWORD_TIME_ZONE: &str = "time_zone";
    pub const NOTION_KEYWORD_ARRAY: &str = "array";
    
    pub const NOTION_API_VERSION_HEADER: &str = "Notion-Version";
    pub const NOTION_API_VERSION_VALUE: &str = "2022-06-28";
    
    pub const NOTION_API_DATABASE_START_URL: &str = "https://api.notion.com/v1/databases/";
    pub const NOTION_API_DATABASE_END_URL: &str = "/query";
}
