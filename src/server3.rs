use std::convert::TryInto;
use std::net::UdpSocket;
use std::thread;

fn read_request(socket: &UdpSocket) -> (u8, i64, std::net::SocketAddr) {
    let mut buffer = [0; 9];

    let (length, client_addr) = socket
        .recv_from(&mut buffer)
        .expect("Failed to receive data from client");

    let operation_flag = buffer[0];
    let result = i64::from_be_bytes(
        buffer[1..length]
            .try_into()
            .expect("Error converting bytes to number"),
    );

    (operation_flag, result, client_addr)
}

fn process_request(operation_flag: u8, number: i64) -> i64 {
    match operation_flag {
        0 => number.checked_add(1).unwrap_or(i64::MAX), // Increment with overflow handling
        1 => number.checked_sub(1).unwrap_or(i64::MIN), // Decrement with overflow handling
        _ => {
            eprintln!("Invalid operation flag received from client");
            0
        }
    }
}

fn send_response(socket: &UdpSocket, response: i64, client_addr: std::net::SocketAddr) {
    let response_data = response.to_be_bytes();
    socket
        .send_to(&response_data, client_addr)
        .expect("Error sending data to client");
}

fn handle_client(socket: UdpSocket) {
    loop {
        let (operation_flag, number, client_addr) = read_request(&socket);
        let new_result = process_request(operation_flag, number);
        send_response(&socket, new_result, client_addr);
    }
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8082").expect("Failed to bind to UDP socket");

    println!("Server listening on 127.0.0.1:8082...");

    let num_threads = 4; // Number of threads to handle clients

    for _ in 0..num_threads {
        let socket_clone = socket.try_clone().expect("Failed to clone socket");
        thread::spawn(move || {
            handle_client(socket_clone);
        });
    }

    // Block the main thread to keep the program running
    for _ in 0..num_threads {
        thread::park();
    }
}
