pub struct Input {
    file: String,
    table: String,
    chain: String
}

impl Input {

    pub fn new(vec: Vec<String>) -> Input {
        Input {
            file: vec[1].clone(),
            chain: vec[2].clone(),
            table: vec[3].clone()
        }
    }
    
    pub fn get_file(&self) -> &String {
        &self.file
    }

    pub fn get_table(&self) -> &String {
        &self.table
    }

    pub fn get_chain(&self) -> &String {
        &self.chain
    }
}
