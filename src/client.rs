use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

fn send_request_to_server(number: i64, operation_flag: u8) -> i64 {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");

    let server_address = "127.0.0.1:8080";

    // Send the request to the server (operation_flag + number)
    let mut request = Vec::new();
    request.push(operation_flag);
    request.extend_from_slice(&number.to_be_bytes());

    socket
        .send_to(&request, server_address)
        .expect("Failed to send data to server");

    let mut buffer = [0; 8];
    // Receive the result from the server
    socket
        .recv_from(&mut buffer)
        .expect("Failed to receive data from server");

    let result = i64::from_be_bytes(buffer);
    result
}

fn main() {
    let number_of_requests = 1000;
    let delay_duration = Duration::from_secs(1);

    let mut total_duration = Duration::new(0, 0);

    for i in 0..number_of_requests {
        let iteration_start = Instant::now();

        let number_to_increment = 102;
        let result_increment = send_request_to_server(number_to_increment, 0); // 0 for increment
        println!("Server responded with increment: {}", result_increment);

        let number_to_decrement = 110;
        let result_decrement = send_request_to_server(number_to_decrement, 1); // 1 for decrement
        println!("Server responded with decrement: {}", result_decrement);

        let iteration_time = iteration_start.elapsed();
        total_duration += iteration_time;

        println!("Iteration {} took: {:?}", i + 1, iteration_time);

        // Add a delay between requests for better synchronization
        thread::sleep(delay_duration);
    }

    // Calculate and print the average duration
    let average_duration = total_duration / number_of_requests as u32;
    println!("Average iteration time: {:?}", average_duration);
}
