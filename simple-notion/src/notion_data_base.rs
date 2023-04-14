use std::any::TypeId;

pub type DataBaseElement = (String, json::JsonValue);

#[derive(Debug, Clone, Default)]
pub struct NotionRelation {
    pub ids: Vec<String>,
    pub has_more: bool,
}

impl NotionRelation {
    pub fn new(ids: Vec<String>, has_more: bool) -> Self {
        Self {
            ids,
            has_more,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NotionFormula {
    pub type_name: String,
    pub result_str: String,
    pub result_number: f64,
}

impl NotionFormula {
    pub fn new(type_name: String, result_str: String, result_number: f64) -> Self {
        Self {
            type_name,
            result_str,
            result_number,
        }
    }
    
    pub fn get_type_id(&self) -> TypeId {
        if self.result_str.is_empty() {
            TypeId::of::<f64>()
        } else {
            TypeId::of::<String>()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NotionDate {
    pub start: Option<String>,
    pub end: Option<String>,
    pub time_zone: Option<String>,
}

impl NotionDate {
    pub fn new(start: Option<String>, end: Option<String>, time_zone: Option<String>) -> Self {
        Self {
            start,
            end,
            time_zone,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotionFile {
    pub name: String,
    pub url: String
}

impl NotionFile {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Title(String),
    URL(String),
    Text(String),
    Number(f64),
    Files(Vec<NotionFile>),
    Select(String),
    MultiSelect(Vec<String>),
    CheckBox(bool),
    EMail(String),
    Status(String),
    People(Vec<String>),
    Date(NotionDate),
    CreatedBy(String),
    Formula(NotionFormula),
    PhoneNumber(String),
    Relation(NotionRelation),
    Rollup(Vec<json::JsonValue>),
    CreatedTime(String),
    LastEditedTime(String),
    LastEditedBy(String),
    Uknown,
    /// Null & return the type-name of the element
    Null(String),
}

pub struct NotionDataBase {
    content: Vec<Vec<DataBaseElement>>,
}

impl NotionDataBase {
    pub fn new(content: Vec<Vec<DataBaseElement>>) -> Self {
        Self { content }
    }
    
    pub fn get_line_count(&self) -> usize {
        self.content.len()
    }
    
    pub fn get_column_count(&self) -> usize {
        match self.content.first() {
            Some(first) => {
                first.len()
            }
            None => 0,
        }
    }
    
    /// Return the first element of a line
    pub fn get_first_at(&self, line: usize) -> Option<DataBaseElement> {
        self.get_at(line, 0)
    }
    
    /// Return the last element of a line
    #[deprecated(note="Notion not follow any logic in data base index")]
    pub fn get_last_at(&self, line: usize) -> Option<DataBaseElement> {
        self.get_at(line, self.get_column_count() - 1)
    }
    
    // Get an element of the data base using line and column index
    pub fn get_at(&self, mut line: usize, mut column: usize) -> Option<DataBaseElement> {
        line = (self.get_line_count() as i32 - 1_i32 - line as i32) as usize;
        column = (self.get_column_count() as i32 - 1_i32 - column as i32) as usize;
        
        match self.content.get(line) {
            Some(value) => {
                match value.get(column) {
                    Some(value) => {
                        Some(value.clone())
                    }
                    None => None,
                }
            }
            None => None,
        } 
    }
    
    pub fn get(&self, parser: &crate::parser::NotionResponseParser, line: &str, column_name: &str) -> Option<DataBaseElement> {
        for table_line in self.content.iter() {
            let mut name = String::new();
            match parser.parse_element(table_line.last().unwrap().1.clone()) {
                DataType::Title(title) => {
                    name = title;
                }
                _ => (),   
            }
            if name == line {
                for column in table_line {
                    if column.0 == column_name {
                        return Some(column.clone()); 
                    }
                }
            }
        }
        None
    }
}
