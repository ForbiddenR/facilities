use std::{io, vec};

use one_zero_four_client::{modbus_crc16, model::messages::{LoginRequest, LoginResponse}};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("121.199.192.223:8768").await?;
    println!("Connected to the server");
    let message = LoginRequest::new(LoginRequest {
        charger_code: "1000330".to_string(),
        charger_type: 1,
        connector_count: 1,
        protocol_version: 1.0,
        program_version: "v1.0.1".to_string(),
        network_type: 0x01,
        sim_card: "0".to_string(),
        operator: 0x04,
    })
    .marshal();
    let mut result = vec![0x68, (4 + message.len()) as u8];
    let mut m = vec![ 0x01, 0x00, 0x00, 0x01];
    m.extend_from_slice(&message);

    let crc = modbus_crc16(&m).to_be_bytes();    

    result.extend_from_slice(&m);
    result.extend_from_slice(&crc);
    
    println!("{:x?}", &result[6..6+7]);

    stream.write_all(&result).await?;
    println!("Sent message to the server");

    let mut buffer = [0u8; 1024];

    let n = stream.read(&mut buffer).await?;

    let mut response = LoginResponse::new();

    println!("response is {:x?}", &buffer[6..n-2]);

    response.unmarshal(&buffer[6..n-2]);

    println!("Received response from the server: {:?}", response);

    Ok(())
}
