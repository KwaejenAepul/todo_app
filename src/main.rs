use std::io;
use rusqlite::Connection;
//use eframe::equi;

mod db;

fn main(){
    let conn = Connection::open("todo.db").unwrap();
    conn.execute(&"CREATE TABLE IF NOT EXISTS todos(priority INTERGER, task STRING)", ())
        .unwrap();
    let mut choice: String = String::new();
    loop {
        choice.clear();
        println!(
            "(1) see todo list\n(2) add task\n(3)remove task\n(4)end program"
        );
        io::stdin()
            .read_line(&mut choice)
            .unwrap();
        choice = choice.trim().to_string();
        match choice.as_str(){
            "1" => {let db_entries = db::read_todo(&conn).unwrap();
                                    for entry in db_entries{
                                        println!("priority:{}, task:{}", entry.priority, entry.name)
                                    }},
            "2"=> db::add_todo(&conn),
            "3"=> db::remove_todo(&conn),
            "4"=> break,
            _=> println!("`{}`, is not a valid option", choice),
        }

    }

}
