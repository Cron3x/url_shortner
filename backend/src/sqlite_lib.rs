use rusqlite::{Connection, Result};
pub struct Url{
    pub long_url:String,
    pub short_url:String
}

pub struct DBLib{
    db_name: String,
    table_name: String
}

impl DBLib{
    pub fn new(db_name:&str,table_name:&str) -> Self{
        Self { db_name: db_name.to_string(), table_name: table_name.to_string(), }
    }
    fn create(&self) -> Result<()> {
        let con = Connection::open("urls.db")?;
        con.execute(
            "create table if not exists urls (
                 id integer primary key,
                 log_url text not null unique,
                 short_url text not null unique
             )",
            (),
        )?;
        Ok(())
    }
    pub fn read(&self)->Result<Vec<Url>> {
        let con = Connection::open("urls.db")?;
        let mut stmt = con.prepare("SELECT * FROM urls")?;
        let url_iter = stmt.query_map([], |row| {
            Ok(Url {
                long_url: row.get(1)?,
                short_url: row.get(2)?,
            })
        })?;

        let mut vu:Vec<Url> = vec![];
        for i in url_iter {
            vu.push(i.unwrap());
        }

        Ok(vu)
    }

    pub fn write(&self, url:Url) -> Result<()>{
        self.create().expect("error creating table");
        let con = Connection::open("urls.db").expect("connection not made");
        con.execute("INSERT OR IGNORE INTO urls (log_url, short_url) VALUES (?1,?2)", 
            (url.long_url, url.short_url)).expect("inserting error");
        Ok(())
    }
}