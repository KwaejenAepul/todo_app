use std::io;
use rusqlite::{Connection, Result};
//use eframe::equi;

#[derive(Debug)]
struct DbEntry{
    priority: u8,
    name: String,
}

fn add_todo(conn: &Connection){
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

fn remove_todo(conn: &Connection){
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

fn read_todo(conn: &Connection) -> Result<Vec<DbEntry>>{
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

fn main() -> Result<()> {
    let conn = Connection::open("todo.db").unwrap();
    conn.execute(&"CREATE TABLE IF NOT EXISTS todos(priority INTERGER, task STRING)", ())
        .unwrap();
    let mut choice: String;
    loop {
        println!(
            "(1) see todo list\n(2) add task\n(3)remove task\n(4)end program"
        );
        choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .unwrap();
        choice = choice.trim().to_string();
        match choice.as_str(){
            "1" => {let db_entries = read_todo(&conn).unwrap();
                                    for entry in db_entries{
                                        println!("priority:{}, taks:{}", entry.priority, entry.name)
                                    }},
            "2"=> add_todo(&conn),
            "3"=> remove_todo(&conn),
            "4"=> break,
            _=> println!("`{}`, is not a valid option", choice),
        }

    }

       Ok(())
}
