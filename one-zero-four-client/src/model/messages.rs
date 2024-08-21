use crate::tool::{bcd_decode, bcd_encode, reader::Reader};

use super::types::LoginStatus;

#[derive(Debug, Default)]
pub struct LoginRequest {
    pub charger_code: String,
    pub charger_type: u8,
    pub connector_count: u8,
    pub protocol_version: f32,
    pub program_version: String,
    pub network_type: u8,
    pub sim_card: String,
    pub operator: u8,
}

impl LoginRequest {
    pub fn new(request: LoginRequest) -> Self {
        request
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&bcd_encode(&self.charger_code, 7));

        buffer.push(self.charger_type);

        buffer.push(self.connector_count);

        buffer.push((self.protocol_version * 10.0) as u8);

        let mut version = [0u8; 8];

        version[..self.program_version.len()].copy_from_slice(self.program_version.as_bytes());

        buffer.extend_from_slice(&version);

        buffer.push(self.network_type);

        buffer.extend_from_slice(&bcd_encode(&self.sim_card, 10));

        buffer.push(self.operator);

        buffer
    }

    pub fn unmarshal(&mut self, data: &[u8]) {
        let mut reader = Reader::new(data);
        self.charger_code = bcd_decode(reader.unsafe_read(7))
            .trim_start_matches('0')
            .to_string();
        self.charger_type = reader.unsafe_read(1)[0];
        self.connector_count = reader.unsafe_read(1)[0];
        self.protocol_version = reader.unsafe_read(1)[0] as f32 / 10.0;
        self.program_version = String::from_utf8_lossy(reader.unsafe_read(8))
            .trim_end_matches('\0')
            .to_string();
        self.network_type = reader.unsafe_read(1)[0];
        self.sim_card = bcd_decode(reader.unsafe_read(10));
        self.operator = reader.unsafe_read(1)[0];
    }
}

#[derive(Debug, Default)]
pub struct LoginResponse {
    pub charger_code: String,
    pub status: LoginStatus,
}

impl LoginResponse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn marshal(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&bcd_encode(&self.charger_code, 7));
        buffer.push(self.status.into());
        buffer
    }

    pub fn unmarshal(&mut self, data: &[u8]) {
        let mut reader = Reader::new(data);
        self.charger_code = bcd_decode(reader.unsafe_read(7))
            .trim_start_matches('0')
            .to_string();
        self.status = reader.unsafe_read(1)[0].into();
    }
}
