extern crate sqlite;

//use sqlite::Statement;
use sqlite::Connection;

use std::time::{SystemTime};

fn get_connection() ->sqlite::Connection {
    let connection = sqlite::open("test.db").unwrap();
    connection
}

fn turn_off_db_sync_write(conn: &Connection) {
    conn
    .execute(
        "
        PRAGMA synchronous = OFF; 
        ",
    ).unwrap();
}

fn create_account(conn: &Connection, account_name: &String) {
	conn.execute(format!("CREATE TABLE {} (token_id INTEGER PRIMARY KEY);", account_name),).unwrap();
}

fn super_roll_out_tokens(conn: &Connection, to_account: &String, start_id: i64, num_of_tokens: i64) {
	let now = SystemTime::now();
	conn.execute(
        "
        begin;
        ",

	let mut statement = conn.prepare(format!("insert into {} values(?)", to_account)).unwrap();

	let end_id = start_id + num_of_tokens;
	for token_id in start_id..end_id {
		statement.reset().unwrap();
		statement.bind(1, token_id).unwrap();
		statement.next().unwrap();
	}

	conn.execute(
	        "
	        commit;
	        ",
	).unwrap();
	
	match now.elapsed() {
		Ok(elapsed) => {
			println!("super roll out {} tokens to {} within time {} seconds", 
			num_of_tokens, to_account, elapsed.as_secs());
		}
		Err(e) => {
	        println!("Error: {:?}", e);
	    }
	}
}

fn main() {
	let total = 50000000000;
	let num_of_user = 2;
	let average_balance = total / num_of_user;
	let conn = get_connection();
	let now = SystemTime::now();
	turn_off_db_sync_write(&conn);
	
	for user_id in 0..num_of_user {
		let account = String::from(format!("usr_{}", user_id));
		create_account(&conn, &account);
		super_roll_out_tokens(&conn, &account, user_id*average_balance, average_balance);
	}

	match now.elapsed() {
		Ok(elapsed) => {
			println!("Super roll out {} tokens to {} accounts within time {} seconds", 
			total, num_of_user, elapsed.as_secs());
		}
		Err(e) => {
	        println!("Error: {:?}", e);
	    }
	}
}
