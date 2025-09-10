pub mod schema;
pub struct Store {
    conn: Connection
}

impl Default for Store {
    fn default() -> Self {
        
        Self {
            conn
        }
    } 
}

impl Store {
    pub fn create_user(&self) {
        print!("create user called");
        self.conn.execute("INSERT INTO xyz")
    }

    pub fn create_website(&self) -> String {
        String::from("1")
    }
}