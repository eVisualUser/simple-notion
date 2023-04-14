use crate::notion_data_base::{DataBaseElement, DataType, NotionFile, NotionDate, NotionFormula, NotionRelation};
use crate::non_magic::notion::*;

pub struct NotionResponseParser {
    value: json::JsonValue,
}

impl NotionResponseParser {
    pub fn new<T: ToString>(buffer: T) -> Self {
        Self {
            value: json::parse(&buffer.to_string()).unwrap(),
        }
    }

    pub fn parse_table(&self) -> Vec<Vec<DataBaseElement>> {
        let mut result = Vec::<Vec<DataBaseElement>>::new();

        let results = self.value[NOTION_JSON_RESULTS].clone();
        for i in results.members() {
            let mut elements: Vec<DataBaseElement> = Vec::new();
            for i in i[NOTION_JSON_PROPERTIES].entries() {
                let out = (i.0.to_owned(), i.1.clone());
                elements.push(out);
            }
            result.push(elements);
        }

        result
    }

    pub fn parse_element(&self, element: json::JsonValue) -> DataType {
        match element["type"].as_str().unwrap() {
            NOTION_TYPE_TITLE => {
                let last = crate::get_last!(element);
                match last.1.members().last().unwrap()[NOTION_KEYWORD_TEXT][NOTION_KEYWORD_CONTENT].as_str() {
                    Some(value) => {
                        DataType::Title(value.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_TITLE.to_owned()),
                }
            }
            NOTION_TYPE_FILES => {
                let last = crate::get_last!(element);
                let mut out = Vec::<NotionFile>::new();
                for file in last.1.members() {
                    if file["type"] != NOTION_KEYWORD_EXTERNAL {
                        match file[NOTION_KEYWORD_NAME].as_str() {
                            Some(name) => {
                                match file[NOTION_KEYWORD_FILE][NOTION_KEYWORD_URL].as_str() {
                                    Some(file) => {
                                        out.push(NotionFile::new(name.to_owned(), file.to_owned()));
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    } else {
                        match file[NOTION_KEYWORD_NAME].as_str() {
                            Some(name) => {
                                match file[NOTION_KEYWORD_EXTERNAL][NOTION_KEYWORD_URL].as_str() {
                                    Some(file) => {
                                        out.push(NotionFile::new(name.to_owned(), file.to_owned()));
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    }
                }
                DataType::Files(out)
            }
            NOTION_TYPE_SELECT => {
                let last = crate::get_last!(element);
                match last.1[NOTION_KEYWORD_NAME].as_str() {
                    Some(value) => {
                        DataType::Select(value.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_SELECT.to_owned()),
                }
            }
            NOTION_TYPE_MULTI_SELECT => {
                let last = crate::get_last!(element);
                let mut out = Vec::<String>::new();
                for select in last.1.members() {
                    match select[NOTION_KEYWORD_NAME].as_str() {
                        Some(select) => {
                            out.push(select.to_owned())        
                        }
                        None => (),
                    }
                }
                DataType::MultiSelect(out)
            }
            NOTION_TYPE_NUMBER => {
                let last = crate::get_last!(element);
                match last.1.as_f64() {
                    Some(number) => {
                        DataType::Number(number)
                    }
                    None => DataType::Null(NOTION_TYPE_NUMBER.to_owned()),
                }
            }
            NOTION_TYPE_CHECKBOX => {
                let last = crate::get_last!(element);
                match last.1.as_bool() {
                    Some(bool) => {
                        DataType::CheckBox(bool)
                    }
                    None => DataType::Null(NOTION_TYPE_CHECKBOX.to_owned())
                }
            }
            NOTION_TYPE_URL => {
                let last = crate::get_last!(element);
                match last.1.as_str() {
                    Some(url) => {
                        DataType::URL(url.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_URL.to_owned())
                }
            }
            NOTION_TYPE_EMAIL => {
                let last = crate::get_last!(element);
                match last.1.as_str() {
                    Some(email) => {
                        DataType::EMail(email.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_EMAIL.to_owned()),
                }
            }
            NOTION_TYPE_STATUS => {
                let last = crate::get_last!(element);
                match last.1[NOTION_KEYWORD_NAME].as_str() {
                    Some(status) => {
                        DataType::Status(status.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_STATUS.to_owned()),
                }
            }
            NOTION_TYPE_PEOPLE => {
                let last = crate::get_last!(element);
                let mut out = Vec::<String>::new();
                for select in last.1.members() {
                    match select[NOTION_KEYWORD_ID].as_str() {
                        Some(people) => {
                            out.push(people.to_owned());
                        }
                        None => (),
                    }
                }
                DataType::People(out)
            }
            NOTION_TYPE_CREATED_BY => {
                let last = crate::get_last!(element);
                match last.1["id"].as_str() {
                    Some(id) => {
                        DataType::CreatedBy(id.to_owned())       
                    }
                    None => {
                        DataType::Null(String::from(NOTION_TYPE_CREATED_BY))
                    }
                }
            }
            NOTION_TYPE_FORMULA => {
                
                let last = crate::get_last!(element);
                
                let mut formula = NotionFormula::default();
                
                match last.1["type"].as_str() {
                    Some(type_name) => {
                        formula.type_name = type_name.to_owned();
                    }
                    None => (),
                }
                
                if !formula.type_name.is_empty() {
                    match last.1[formula.type_name.clone()].as_str() {
                        Some(result) => {
                            formula.result_str = result.to_owned();
                        }
                        None => {
                            match last.1[formula.type_name.clone()].as_f64() {
                                Some(result) => {
                                    formula.result_number = result.to_owned();
                                }
                                None => return DataType::Null(NOTION_TYPE_FORMULA.to_owned()),
                            }
                        }
                    }
                }
                
                DataType::Formula(formula)
            }
            NOTION_TYPE_PHONE_NUMBER => {
                let last = crate::get_last!(element);
                
                match last.1.as_str() {
                    Some(phone_number) => {
                        DataType::PhoneNumber(phone_number.to_owned())
                    }
                    None => DataType::Null(NOTION_TYPE_PHONE_NUMBER.to_owned())
                }
            }
            NOTION_TYPE_RELATION => {
                let last = crate::get_last!(element);
                let mut notion_relation = NotionRelation::default();
                
                match last.1.as_bool() {
                    Some(has_more) => {
                        notion_relation.has_more = has_more;
                    }
                    None => (),
                }
                
                for relation in element[NOTION_TYPE_RELATION].members() {
                    match relation[NOTION_KEYWORD_ID].as_str() {
                        Some(id) => {
                            notion_relation.ids.push(id.to_owned());
                        }
                        None => (),
                    } 
                }
                
                if !notion_relation.ids.is_empty() {
                    DataType::Relation(notion_relation)
                } else {
                    DataType::Null(NOTION_TYPE_RELATION.to_owned())   
                }
            }
            NOTION_TYPE_DATE => {
                let last = crate::get_last!(element);

                let mut notion_date = NotionDate::default();
                
                match last.1[NOTION_KEYWORD_START].as_str() {
                    Some(date) => {
                        notion_date.start = Some(date.to_owned());
                    }
                    None => (),
                }
                
                match last.1[NOTION_KEYWORD_END].as_str() {
                    Some(date) => {
                        notion_date.end = Some(date.to_owned());
                    }
                    None => (),
                }
                
                match last.1[NOTION_KEYWORD_TIME_ZONE].as_str() {
                    Some(date) => {
                        notion_date.time_zone = Some(date.to_owned());
                    }
                    None => (),
                }

                DataType::Date(notion_date)
            }
            NOTION_TYPE_ROLLUP => {
                let mut out = Vec::<json::JsonValue>::new(); 
                for member in element[NOTION_TYPE_ROLLUP][NOTION_KEYWORD_ARRAY].members() {
                    out.push(member.clone());
                }
                
                if out.is_empty() {
                    DataType::Null(NOTION_TYPE_ROLLUP.to_owned())
                } else {
                    DataType::Rollup(out)
                }
            }
            NOTION_TYPE_CREATED_TIME => {
                match element[NOTION_TYPE_CREATED_TIME].as_str() {
                    Some(time) => DataType::CreatedTime(time.to_owned()),
                    None => DataType::Null(NOTION_TYPE_CREATED_TIME.to_owned()), 
                }
            }
            NOTION_TYPE_LAST_EDITED_TIME => {
                match element[NOTION_TYPE_LAST_EDITED_TIME].as_str() {
                    Some(time) => DataType::LastEditedTime(time.to_owned()),
                    None => DataType::Null(NOTION_TYPE_LAST_EDITED_TIME.to_owned()), 
                }
            }
            NOTION_TYPE_LAST_EDITED_BY => {
                match element[NOTION_TYPE_LAST_EDITED_BY][NOTION_KEYWORD_ID].as_str() {
                    Some(user) => DataType::LastEditedBy(user.to_owned()),
                    None => DataType::Uknown, 
                }
            }
            x => {
                println!("Unsupported: {}", x);
                DataType::Uknown
            }
        }
    }
}

#[macro_export]
macro_rules! get_last {
    ($element:ident) => {
        $element.entries().last().unwrap()
    };
}
