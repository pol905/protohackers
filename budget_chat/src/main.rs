use clap::{Parser};
use protohackers_tcp_helper::{
    cli_helper::Args,
    tcp,
    errors::ProtoHackersError
};
use std::{
    thread,
    io::{Read, Write, BufRead, BufWriter, Error},
    collections::HashMap,
    net::TcpStream,
    sync::{Arc, Mutex}
};

enum UserNameError {
    NameTooShort,
    NameTooLong,
    InvalidName
}

impl UserNameError {
    fn geenrate_error_message(err: UserNameError) -> &'static str {
        match err {
            UserNameError::NameTooShort => "Name too short (Minimum 1 character)",
            UserNameError::NameTooLong => "Name too long (Maximum 16 characters)",
            _ => "Invalid name. Please ensure name consists of only alphabets and digits",
        }
    }
}

// Points to remember:
// 1. When client first connects it doesn't have a name, send a welcome message asking
// them what to be called. To which the client will reply with a name.
// 2. all messages will be ascii
// 3. name validation: min. 1 char, max. 16 chars, uppercase, lowercase, and digits.
//

fn validate_user_name(user_name: &str) -> Result<(), UserNameError> {
    let user_name_length = user_name.len();
    if user_name_length < 1 {
        return Err(UserNameError::NameTooShort);
    } else if user_name_length > 16 {
        return Err(UserNameError::NameTooLong);
    } else if !user_name.chars().all(char::is_alphanumeric) {
        return Err(UserNameError::InvalidName);
    }
    Ok(())
}

fn write_buf(buf_writer: &mut BufWriter<&TcpStream>, buf: &[u8]) {
    buf_writer.write(buf).unwrap();
    buf_writer.flush().unwrap();
}

fn handle_connection(connection: Result<TcpStream, Error>, c_users: Arc<Mutex<HashMap<String, TcpStream>>>) {
    match connection {
        Ok(stream) => {
            let hello_message = b"Welcome to budgetchat! What shall I call you?
";
            let mut buf_reader = tcp::create_buf_reader(&stream);
            let mut buf_writer = tcp::create_buf_writer(&stream);
            write_buf(&mut buf_writer, hello_message);
            let mut buf = vec![];
            let bytes_read = buf_reader.read_until(b'\n', &mut buf);
            if bytes_read.is_err() {
                return;
            }
            let user_name = std::str::from_utf8(&buf).unwrap().trim();
            let is_user_name_valid = validate_user_name(user_name);
            if let Err(err) = is_user_name_valid {
                let err_message: &str = UserNameError::geenrate_error_message(err);
                write_buf(&mut buf_writer, err_message.as_bytes());
                return;
            }
            if let Ok(mut connected_users) = c_users.lock() {

                let user_joined_message = format!("* {user_name} has entered the room\n");
                for mut connected_user_connection in connected_users.values() {
                    connected_user_connection.write(user_joined_message.as_bytes()).unwrap();
                }
                let connected_user_names: Vec<&String> = connected_users.keys().collect();
                let joined_user_names = connected_user_names
                .iter()
                .copied()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ");
                let new_user_announcement_message = format!("* The room contains: {}\n", joined_user_names);
                write_buf(&mut buf_writer, new_user_announcement_message.as_bytes());
                connected_users.insert(user_name.to_owned(), stream.try_clone().unwrap());
            }

            loop {
                let mut message = vec![];
                let bytes_read = buf_reader.read_until(b'\n', &mut message);
                if message.len() > 1000 {
                    write_buf(&mut buf_writer, b"Message too long (Maximum 1000 characters)");
                }
                if bytes_read.is_err() || message.is_empty() {
                    break;
                }
                if let Ok(connected_users) = c_users.lock() {
                    let new_message = format!("[{user_name}] {}\n", String::from_utf8(message).unwrap().trim());
                    for (connected_user_name, mut connected_user_connection) in connected_users.iter() {
                        if connected_user_name != user_name {
                            connected_user_connection.write(new_message.as_bytes()).unwrap();
                        }
                    }
                }
            }
            if let Ok(mut connected_users) = c_users.lock() {
                connected_users.remove(user_name);
                let final_response = format!("* {user_name} has left the room\n");
                for mut connected_user_connection in connected_users.values() {
                    connected_user_connection.write(final_response.as_bytes()).unwrap();
                }
            }
        }
        Err(err) => {
            let err: ProtoHackersError = err.into();
            println!("Failed to connect: {:?}", err);
        }
    }
}

fn main() {
    let args = Args::parse();
    let listener = tcp::create_listener(args.port).unwrap();
    let users: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    for connection in listener.incoming() {
        let c_users = users.clone();
        thread::spawn(move || { handle_connection(connection, c_users)});
    }
}
