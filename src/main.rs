//PING STUFF
use ping::ping; //for ping test
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

//POSTGRES STUFF
use postgres::{Client, Config, GenericClient, NoTls, Row, SimpleQueryMessage, SimpleQueryRow };
use std::collections::HashMap;
use serde_json::{json, Value};
use std::sync::Arc;


//===================
//PROGRAM TO RUN
//===================
fn main() {
    println!("Hello, world!");
    println!("Ping:");
    run_ping();
    println!("Done");

    run_postgres_prepared(); // DOESN'T WORK
    //run_postgres_simple(); // WORKS (BUT NOT SAFE) 
    println!("Done postgres");

}

//https://github.com/tokio-rs/rdbc
//https://lib.rs/crates/jdbc
// fn run_jdbc(){

// }


fn run_postgres_prepared(){
    let mut config = Config::new();

    //NEED PORT CHANGED ALSO IF NEEDED=============
    config.host("localhost")
          .user("root")
          .password("password")
          .dbname("mydb");
    let result: Result<Client, postgres::Error> = config.connect(NoTls);

    let mut client = result.expect("FAILED TO UNWRAP CLIENT");  // This will panic if the connection fails

    //https://docs.rs/postgres/latest/postgres/struct.Client.html#method.simple_query
    
    let first_name="Carl";
    let query= "SELECT * FROM Customer WHERE firstName = $1";
    let query_result = client.query(query,&[&first_name]);

    //alternative method, trickier to make work:
    //let statement = client.prepare("SELECT * FROM Customer WHERE firstName = $1").expect("could not unwrap client prepare");
    //let query_result = client.query(&statement,&[&first_name]);

    let mut json_results = Vec::new();

    match query_result {
        Ok(rows) => {
 
            for row in &rows {

                // Convert each row into a HashMap (we'll assume you know the structure of the row)
                let mut row_map: HashMap<&str, serde_json::Value> = HashMap::new();
                
                let columns = row.columns();  // Get column metadata

                for (i, column) in columns.iter().enumerate() {
                    let value: Value = match row.try_get::<_, String>(i) {
                        Ok(val) => Value::String(val),
                        Err(_) => match row.try_get::<_, i32>(i) {
                            Ok(val) => Value::Number(val.into()),
                            Err(_) => match row.try_get::<_, bool>(i) {
                                Ok(val) => Value::Bool(val),
                                Err(_) => match row.try_get::<_, f64>(i) {
                                    Ok(val) => {
                                        Value::Number(serde_json::Number::from_f64(val).unwrap_or_else(|| serde_json::Number::from(0)))
                                    }
                                    Err(_) => Value::Null, // Handle unknown or NULL values gracefully
                                },
                            },
                        },
                    };
                    row_map.insert(column.name(), value);
                
                //can filter out parts not needed here or alter query, eg. don't need "@cat": "v", (vertex)
                }
                
                json_results.push(row_map);
        
            }

            // Convert to JSON string using serde_json
            let json_string = serde_json::to_string(&json_results).expect("JSON CREATION FAILED");

            // Print the JSON string
            println!("{}", json_string);
            
            // Optionally, you can print the number of rows fetched
            println!("Fetched {} rows", rows.len());

        }
        Err(e) => {
            // Handle error
            eprintln!("Error fetching customers: {}", e);
        }
    }
    
    // Now you can use the `client` object to interact with the database
    println!("Successfully connected to the database!");
    //let mut client= Client::connect("host=localhost user=root", NoTls);
    //if no port 5432 is used
    //https://docs.rs/postgres/latest/postgres/config/struct.Config.html#method.port

}


//============================================
// SIMPLE POST GRES MODE (UNSAFE)
//============================================
fn run_postgres_simple(){
    let mut config = Config::new();

    //NEED PORT CHANGED ALSO IF NEEDED=============
    config.host("localhost")
          .user("root")
          .password("password")
          .dbname("mydb");
    let result: Result<Client, postgres::Error> = config.connect(NoTls);

    let mut client = result.expect("FAILED TO UNWRAP CLIENT");  // This will panic if the connection fails

    //https://docs.rs/postgres/latest/postgres/struct.Client.html#method.simple_query
    //let rows_result = client.query("SELECT * FROM Customer", &[]); //DOESN"T WORK
    
    //string interpolation (not safe but works):
    //let name = "*";
    //let query_string: String =  format!("SELECT {} FROM Customer", name);
    //let query_result = client.simple_query(&query_string);
    let query_result = client.simple_query("SELECT * FROM Customer");

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
    
    //====================
    //PRINT EACH ROW
    //====================
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
    
    // Now you can use the `client` object to interact with the database
    println!("Successfully connected to the database!");
    //let mut client= Client::connect("host=localhost user=root", NoTls);
    //if no port 5432 is used
    //https://docs.rs/postgres/latest/postgres/config/struct.Config.html#method.port

}

//=======================
//PING TEST
//=======================
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