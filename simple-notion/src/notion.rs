#[derive(Default, Debug, Clone)]
pub struct NotionURL {
    pub base: String,
    pub name: String,
    pub is_https: bool,
    pub data_base_id: String,
    pub page_id: String,
}

impl NotionURL {
    pub fn new(url: &str) -> Self {
        
        let mut tmp = Self::default();
        
        let mut slash_count = 0;
        let mut buffer = String::new();
        for i in url.chars() {
            if i == '/' {
                match slash_count {
                    0 => {
                        tmp.is_https = buffer.contains("https"); 
                    }
                    2 => {
                        tmp.base = buffer.clone();
                    }
                    3 => {
                        tmp.name = buffer.clone();
                    }
                    _ => (),
                }
                if slash_count != 4 {
                    buffer = String::new();
                    slash_count += 1;
                }
            } else {
                buffer.push(i);
            }
        }
        
        if buffer.contains("?v=") {
            for i in buffer.chars() {
                if i == '?' {
                    break;
                } else {
                    tmp.data_base_id.push(i);
                }
            }
        } else {
            tmp.page_id = buffer;
        }
        
        tmp
    }
    
    pub fn is_page(&self) -> bool {
        !self.page_id.is_empty()
    }
    
    pub fn is_data_base(&self) -> bool {
        !self.is_page()
    }
}
