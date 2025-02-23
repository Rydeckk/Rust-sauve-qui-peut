// decodage.rs

use serde_json;
use std::io::{self, Read};
use std::convert::TryInto;

use crate::structs::JsonWrapper; // On suppose que JsonWrapper est défini dans le module structs

/// Erreur de protocole regroupant les erreurs d'E/S et de désérialisation.
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

/// Lit depuis un flux un message complet en suivant le protocole d'échange.
/// Le flux doit contenir :
///   - 4 octets (u32 en little-endian) pour la taille du message JSON,
///   - le message JSON encodé en UTF-8.
///
/// La fonction désérialise ensuite le JSON et retourne le message de type `JsonWrapper`.
pub fn decode_message<R: Read>(reader: &mut R) -> Result<JsonWrapper, ProtocolError> {
    let mut size_buf = [0u8; 4];
    reader.read_exact(&mut size_buf)?;
    let payload_len = u32::from_le_bytes(size_buf) as usize;
    let mut payload_buf = vec![0u8; payload_len];
    reader.read_exact(&mut payload_buf)?;
    let message: JsonWrapper = serde_json::from_slice(&payload_buf)?;
    Ok(message)
}

/// Décode une chaîne encodée en base64 (selon notre alphabet personnalisé) en données binaires.
/// Renvoie une erreur si la chaîne a une taille invalide (de la forme 4n+1) ou contient un caractère non autorisé.
pub fn decode_b64(s: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8; 64] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/";
    // Construction d'une table de correspondance pour retrouver la valeur associée à chaque caractère.
    let mut rev_table = [255u8; 128];
    for (i, &byte) in ALPHABET.iter().enumerate() {
        rev_table[byte as usize] = i as u8;
    }

    // Vérification : seule une taille de la forme 4n+1 est invalide.
    if s.len() % 4 == 1 {
        return Err("Taille invalide pour un encodage base64".to_string());
    }

    let mut output = Vec::new();
    let mut chars = s.chars().peekable();

    while chars.peek().is_some() {
        let mut group = Vec::new();
        for _ in 0..4 {
            if let Some(c) = chars.peek() {
                let c_val = *c as usize;
                if c_val >= 128 || rev_table[c_val] == 255 {
                    return Err(format!("Caractère non autorisé: {}", c));
                }
                group.push(rev_table[c_val]);
                chars.next();
            } else {
                break;
            }
        }

        match group.len() {
            4 => {
                let byte1 = (group[0] << 2) | (group[1] >> 4);
                let byte2 = ((group[1] & 0x0F) << 4) | (group[2] >> 2);
                let byte3 = ((group[2] & 0x03) << 6) | group[3];
                output.push(byte1);
                output.push(byte2);
                output.push(byte3);
            },
            3 => {
                let byte1 = (group[0] << 2) | (group[1] >> 4);
                let byte2 = ((group[1] & 0x0F) << 4) | (group[2] >> 2);
                output.push(byte1);
                output.push(byte2);
            },
            2 => {
                let byte1 = (group[0] << 2) | (group[1] >> 4);
                output.push(byte1);
            },
            _ => return Err("Groupe de caractères invalide".to_string()),
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use crate::structs::{JsonWrapper, RegisterTeamResult};

    #[test]
    fn test_decode_message() {
        // On encode d'abord un message pour le décoder ensuite.
        let message = JsonWrapper::RegisterTeamResult(
            RegisterTeamResult::Ok { 
                expected_players: 3, 
                registration_token: "SECRET".to_string() 
            }
        );
        // On crée un buffer à partir du message encodé.
        let encoded = {
            let json_string = serde_json::to_string(&message).unwrap();
            let json_bytes = json_string.as_bytes();
            let payload_len = json_bytes.len() as u32;
            let mut buffer = Vec::with_capacity(4 + json_bytes.len());
            buffer.extend_from_slice(&payload_len.to_le_bytes());
            buffer.extend_from_slice(json_bytes);
            buffer
        };

        let mut cursor = io::Cursor::new(encoded);
        let decoded = decode_message(&mut cursor).expect("Décodage message OK");
        assert_eq!(decoded, message);
    }

    #[test]
    fn test_decode_b64() {
        let data = b"Hello, World!";
        let encoded = super::super::encodage::encode_b64(data);
        let decoded = decode_b64(&encoded).expect("Décodage base64 OK");
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_decode_b64_invalid_length() {
        let invalid = "abcde"; // Taille de la forme 4n+1 (ici 5 caractères)
        assert!(decode_b64(invalid).is_err());
    }
}
