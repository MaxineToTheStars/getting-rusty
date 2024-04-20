/**
 * Me: Please let this be a normal hello world!
 * Monkey Brain: With me? No way!
 * Me: Oh!
 */

use std::net::UdpSocket;
use std::str;

fn main() {
    let clientOne: UdpSocket = UdpSocket::bind("127.0.0.1:5000").expect("Client one init error");
    let clientTwo: UdpSocket = UdpSocket::bind("127.0.0.1:5001").expect("Client two init error");

    let clientOneBuffer: &[u8] = "Hello World".as_bytes();
    let mut clientTwoBuffer:[u8; 500] = [0; 500];
    clientOne.send_to(&clientOneBuffer, "127.0.0.1:5001").expect("Client one send error");

    let (bytes, srcAddr) = clientTwo.recv_from(&mut clientTwoBuffer).expect("No data :(");

    let filledClientTwoBuffer: &mut [u8] = &mut clientTwoBuffer[..bytes];

    let out: &str = str::from_utf8(&filledClientTwoBuffer).expect("yeah");

    println!("Out: {}", out)

}
