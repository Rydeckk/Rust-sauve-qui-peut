// encodage.rs

use super::*;
use crate::{structs::JsonWrapper, utils::debug_binary}; // On suppose que JsonWrapper est défini dans le module structs

/// Erreur de protocole regroupant les erreurs d'E/S et de sérialisation.
#[derive(Debug)]
pub enum ProtocolError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

impl From<io::Error> for ProtocolError {
    fn from(e: io::Error) -> Self {
        ProtocolError::IoError(e)
    }
}

impl From<serde_json::Error> for ProtocolError {
    fn from(e: serde_json::Error) -> Self {
        ProtocolError::SerdeError(e)
    }
}

/// Encode un message JSON selon le protocole d'échange.
/// Le format de sortie est :
///   - 4 octets (u32 en little-endian) indiquant la taille du message JSON,
///   - le message JSON encodé en UTF-8.
///
/// Cette fonction est utilisée pour envoyer des messages sur le flux TCP.
pub fn encode_message(message: &JsonWrapper) -> Result<Vec<u8>, ProtocolError> {
    let json_string = serde_json::to_string(message)?;
    let json_bytes = json_string.as_bytes();
    let payload_len = json_bytes.len() as u32;
    let mut buffer = Vec::with_capacity(4 + json_bytes.len());
    
    // Préfixe avec la taille du message en little-endian.
    buffer.extend_from_slice(&payload_len.to_le_bytes());
    buffer.extend_from_slice(json_bytes);
    
    Ok(buffer)
}

/// Encode des données binaires en une chaîne de caractères en base64 selon le schéma imposé.
/// Pour chaque groupe de 3 octets, on produit :
///   - 4 caractères si 3 octets complets sont disponibles,
///   - 3 caractères si 2 octets sont disponibles,
///   - 2 caractères si 1 octet est disponible.
///
/// L'alphabet utilisé est :
///   - 'a'–'z' pour les valeurs 0 à 25,
///   - 'A'–'Z' pour 26 à 51,
///   - '0'–'9' pour 52 à 61,
///   - '+' pour 62 et '/' pour 63.
pub fn encode_b64(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/";
    let mut output = String::new();
    let mut i = 0;

    while i < data.len() {
        let remaining = data.len() - i;
        if remaining >= 3 {
            let b1 = data[i];
            let b2 = data[i + 1];
            let b3 = data[i + 2];
            i += 3;

            let c1 = b1 >> 2;
            let c2 = ((b1 & 0b00000011) << 4) | (b2 >> 4);
            let c3 = ((b2 & 0b00001111) << 2) | (b3 >> 6);
            let c4 = b3 & 0b00111111;

            output.push(ALPHABET[c1 as usize] as char);
            output.push(ALPHABET[c2 as usize] as char);
            output.push(ALPHABET[c3 as usize] as char);
            output.push(ALPHABET[c4 as usize] as char);
        } else if remaining == 2 {
            let b1 = data[i];
            let b2 = data[i + 1];
            i += 2;

            let c1 = b1 >> 2;
            let c2 = ((b1 & 0b00000011) << 4) | (b2 >> 4);
            let c3 = (b2 & 0b00001111) << 2;

            output.push(ALPHABET[c1 as usize] as char);
            output.push(ALPHABET[c2 as usize] as char);
            output.push(ALPHABET[c3 as usize] as char);
        } else {
            let b1 = data[i];
            i += 1;

            let c1 = b1 >> 2;
            let c2 = (b1 & 0b00000011) << 4;

            output.push(ALPHABET[c1 as usize] as char);
            output.push(ALPHABET[c2 as usize] as char);
        }
    }
    output
}

pub fn encode_radar_view_binary(radar_view: [[char; 7]; 7]) -> Vec<u8> {
    let mut binary_radar_view = Vec::new();

    fn encode_wall(c: char) -> u8 {
        match c {
            '#' => 0b00,
            '|' | '-' => 0b10,
            ' ' => 0b01,
            _ => 0b00,
        }
    }

    fn encode_cell(c: char) -> u8 {
        match c {
            ' ' => 0b0000,
            '*' => 0b1000,
            '#' => 0b1111,
            _ => 0b1111
        }
    }

    //Horizontaux
    let mut horiz: u32 = 0;
    let mut bit_index = 0;
    for row in (0..7).step_by(2) {
        for column in (1..7).step_by(2) {
            let val = encode_wall(radar_view[row][column]);
            horiz |= (val as u32) << bit_index;
            bit_index += 2;
        }
    }
    binary_radar_view.extend_from_slice(&horiz.to_le_bytes()[..3]);

    //Verticaux
    let mut vert: u32 = 0;
    bit_index = 0;
    for row in (1..7).step_by(2) {
        for column in (0..7).step_by(2) {
            let val = encode_wall(radar_view[row][column]);
            vert |= (val as u32) << bit_index;
            bit_index += 2;
        }
    }
    binary_radar_view.extend_from_slice(&vert.to_le_bytes()[..3]);

    //Cells
    let mut cells: u64 = 0;
    bit_index = 0;
    for row in (1..7).step_by(2) { 
        for column in (1..7).step_by(2) {
            let val = encode_cell(radar_view[row][column]);
            cells |= (val as u64) << bit_index;
            bit_index += 4;
        }
    }
    binary_radar_view.extend_from_slice(&cells.to_le_bytes()[..5]);

    binary_radar_view

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{JsonWrapper, RegisterTeamResult};

    #[test]
    fn test_encode_message() {
        let message = JsonWrapper::RegisterTeamResult(
            RegisterTeamResult::Ok { 
                expected_players: 3, 
                registration_token: "SECRET".to_string() 
            }
        );
        let encoded = encode_message(&message).expect("Encodage message OK");
        // Le buffer doit comporter au moins 4 octets (pour le header).
        assert!(encoded.len() >= 4);
        let header_bytes = &encoded[..4];
        let payload_len = u32::from_le_bytes(header_bytes.try_into().unwrap()) as usize;
        let payload_bytes = &encoded[4..];
        assert_eq!(payload_len, payload_bytes.len());
        // Vérifie que le JSON désérialisé correspond au message original.
        let decoded_message: JsonWrapper = serde_json::from_slice(payload_bytes).expect("Désérialisation OK");
        assert_eq!(decoded_message, message);
    }

    #[test]
    fn test_encode_b64() {
        assert_eq!(encode_b64(&[0]), "aa");
        assert_eq!(encode_b64(&[25]), "gq");
        assert_eq!(encode_b64(&[26]), "gG");
        assert_eq!(encode_b64(&[51]), "mW");
        assert_eq!(encode_b64(&[52]), "na");
        assert_eq!(encode_b64(&[61]), "pq");
        assert_eq!(encode_b64(&[62]), "pG");
        assert_eq!(encode_b64(&[63]), "pW");
        assert_eq!(encode_b64(b"Hello, World!"), "sgvSBg8SifDVCMXKiq");
    }
}
