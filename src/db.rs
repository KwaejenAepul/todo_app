use std::io;
use rusqlite::{Connection, Result};


#[derive(Debug)]
pub struct DbEntry{
    pub priority: u8,
    pub name: String,
}

pub fn add_todo(conn: &Connection){
    let mut task = String::new();
    let mut priority = String::new();
    println!("name task: ");
    io::stdin()
        .read_line(&mut task)
        .unwrap();
    println!("priority level(1-3): ");
    io::stdin()
        .read_line(&mut priority)
        .unwrap();

    let query = format!("INSERT INTO todos VALUES(?1, ?2);") ;
    let execute_result = conn.execute(&query, (&priority, &task));
    match execute_result{
        Ok(query) => println!("{} rows were updated", query),
        Err(error)=> panic!("Problem with inserting entry: {error:?}"),
    };
}

pub fn remove_todo(conn: &Connection){
    let mut task = String::new();
    println!("name task to remove: ");
    io::stdin()
        .read_line(&mut task)
        .unwrap();
    let execute_result = conn.execute(&"DELETE FROM todos WHERE task = ?1;", [&task]);
    match execute_result{
        Ok(_query) => println!("row deleted"),
        Err(error)=> println!("Error: {:?}", error),
    }
}

pub fn read_todo(conn: &Connection) -> Result<Vec<DbEntry>>{
    let mut db_entries: Vec<DbEntry> = Vec::new();
    let mut db_iter = conn.prepare("SELECT * FROM todos").unwrap();
    let entry_iter = db_iter.query_map([], |row|{
        Ok(DbEntry{
            priority: row.get(0)?,
            name : row.get(1)?,
        })
    })?;
    for i in entry_iter{
        db_entries.push(i.unwrap());
    }
    Ok(db_entries)
}
