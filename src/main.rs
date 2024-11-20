use ping::ping; //for ping test
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

//POSTGRES TRY
use postgres::{Client, Config, GenericClient, NoTls, Row, SimpleQueryMessage, SimpleQueryRow };
use std::collections::HashMap;
use serde_json::json;
use std::sync::Arc;
fn main() {
    println!("Hello, world!");
    println!("Ping:");
    run_ping();
    println!("Done");

    //run_postgres();
    println!("Done postgres");

}

//https://github.com/tokio-rs/rdbc
//https://lib.rs/crates/jdbc
fn run_jdbc(){

}

fn run_postgres(){
    let mut config = Config::new();

    //NEED PORT CHANGED ALSO IF NEEDED=============
    config.host("localhost")
          .user("root")
          .password("password")
          .dbname("mydb");
    let result: Result<Client, postgres::Error> = config.connect(NoTls);

    let mut client = result.expect("FAILED TO UNWRAP CLIENT");  // This will panic if the connection fails

    //select * from Customer
    // Query the database: SELECT * FROM Customer
    //https://docs.rs/postgres/latest/postgres/struct.Client.html#method.simple_query
    //let rows_result = client.query("SELECT * FROM Customer", &[]); //DOESN"T WORK
    
    //WORKS:
    let name = "*";
    let query_string: String =  format!("SELECT {} FROM Customer", name);
    let query_result = client.simple_query(&query_string);
    //let query_result = client.simple_query("SELECT * FROM Customer");

    //let statement = client.prepare("SELECT $1 FROM Customer");
    //let rows_result = client.query(&statement, &[&"*"]);

    //The simplequerymessage represents a simple query request sent by the client to the PostgreSQL server. A "simple query" is essentially just a plain SQL query like SELECT * FROM users; or INSERT INTO table_name VALUES (...). This is in contrast to more complex query types, such as prepared statements or extended queries.

    let mut json_results = Vec::new();

    match query_result {
        Ok(rows) => {
 
            for row in &rows {
                match row {
                    SimpleQueryMessage::Row(row) => {
                        // Convert each row into a HashMap (we'll assume you know the structure of the row)
                        let mut row_map = HashMap::new();
                        
                        let columns = row.columns();  // Get column metadata

                        for (i, value) in columns.iter().enumerate() {
                        
                            row_map.insert(columns[i].name(), row.get(i));
                        
                        //can filter out parts not needed here or alter query, eg. don't need "@cat": "v", (vertex)
                        }
                        
                        json_results.push(row_map);
                    },
                    _ => {}
                }
            }

            // Convert to JSON string using serde_json
            let json_string = serde_json::to_string(&json_results).expect("MADE JSON WRONG");

            // Print the JSON string
            println!("{}", json_string);


    //client.simple_query(&statement, &[&"*"]);
    // Match on the result
    
            println!("rows {}", &rows.len());
            // Successfully fetched rows, process them
            for row in rows {

                match row {
                    SimpleQueryMessage::Row(row) => {

                        
                        // Access data inside SimpleQueryRow
                        println!("Row data: {:?}", row.get(0)); // Here, we're printing the `data` inside the row
                    }
                    _ => {

                    }
                }
            }
        }
        Err(e) => {
            // Handle error
            eprintln!("Error fetching customers: {}", e);
        }
    }
    //let rows_result = rows.expect("UNWRAPPED ROWS BAD");

    // Iterate over the results and print each row
    //for row in rows {

        //println!("row {}", row, name, email);
        // Assume the 'customer' table has fields: id (integer), name (text), and email (text)
        //let id: i32 = row.get(0);      // Column 0 is 'id'
        //let name: String = row.get(1); // Column 1 is 'name'
        //let email: String = row.get(2); // Column 2 is 'email'

        // Print the customer data
        //println!("Customer ID: {}, Name: {}, Email: {}", id, name, email);
    //}

    // Now you can use the `client` object to interact with the database
    println!("Successfully connected to the database!");
    //let mut client= Client::connect("host=localhost user=root", NoTls);
    //if no port 5432 is used
    //https://docs.rs/postgres/latest/postgres/config/struct.Config.html#method.port

}

fn run_ping(){
    // Ping google.com and display the result
    // Define the target IP address (google.com resolved to an IP address)
    let addr: IpAddr = Ipv4Addr::new(8, 8, 8, 8).into();  // Example using Google's DNS server IP

    // Set optional parameters for timeout, TTL, sequence count, and payload
    let timeout = Some(Duration::from_secs(2));  // Timeout of 2 seconds
    let ttl = Some(64);  // Typical TTL value
    let ident = Some(12345);  // Custom identifier
    let seq_cnt = Some(1);  // Sequence count
    let payload = None;  // Optionally add a payload

    // Call the ping function and handle the result
    match ping(addr, timeout, ttl, ident, seq_cnt, payload) {
        Ok(()) => println!("Ping successful!"),
        Err(e) => eprintln!("Ping failed: {}", e),
        // pub enum Error {
        //     InvalidProtocol,
        //     InternalError,
        //     DecodeV4Error,
        //     DecodeEchoReplyError,
        //     IoError {
        //         error: Error,
        //     },
        // }
    }

}