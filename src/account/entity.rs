pub struct Account {
    id: u32,
    name: String,
}

impl Account {
    pub fn create(id: u32, name: String) -> Self {
        Account { id, name }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn display(&self) {
        println!("Account ID: {}, Name: {}", self.id, self.name);
    }
}