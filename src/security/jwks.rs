
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::jwk::JwkSet;
use reqwest::Client;
use serde::{Deserialize};

// #[derive(Deserialize, Debug)]
// pub struct Jwks {
//     keys: Vec<Jwk>
// }
//
// #[derive(Deserialize, Debug)]
// pub struct Jwk {
//     kid: String,
//     kty: String,
//     n: Option<String>,
//     e: Option<String>,
//     x: Option<String>,
//     y: Option<String>,
//     alg: String,
//     #[serde(rename = "use")]
//     use_: String,
//     x5c: Vec<String>,
//     x5t: String,
//     #[serde(rename = "x5t#S256")]
//     x5t_s256: String,
// }
//
// impl Jwk {
//     pub fn first_x509(&self) {
//         let x509_base64encoded = self.x5c.get(0).expect("should be present!");
//         let block = openssl::base64::decode_block(x509_base64encoded.as_str()).unwrap();
//         let result = X509::from_der(&block);
//         println!("{:?}", result);
//     }
// }

#[derive(Deserialize, Debug)]
pub struct IdTokenClaims {
    acr: String,
    at_hash: String,
    aud: String,
    auth_time: u64,
    azp: String,
    email: String,
    email_verified: bool,
    exp: u64,
    family_name: String,
    given_name: String,
    iat: u64,
    iss: String,
    jti: String,
    name: String,
    nonce: Option<String>,
    preferred_username: String,
    sid: String,
    sub: String,
    typ: String,
}

impl IdTokenClaims {
    pub fn acr(&self) -> &str {
        &self.acr
    }

    pub fn at_hash(&self) -> &str {
        &self.at_hash
    }

    pub fn aud(&self) -> &str {
        &self.aud
    }

    pub fn auth_time(&self) -> u64 {
        self.auth_time
    }

    pub fn azp(&self) -> &str {
        &self.azp
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn exp(&self) -> u64 {
        self.exp
    }

    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    pub fn given_name(&self) -> &str {
        &self.given_name
    }

    pub fn iat(&self) -> u64 {
        self.iat
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn jti(&self) -> &str {
        &self.jti
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nonce(&self) -> &Option<String> {
        &self.nonce
    }

    pub fn preferred_username(&self) -> &str {
        &self.preferred_username
    }

    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn sub(&self) -> &str {
        &self.sub
    }

    pub fn typ(&self) -> &str {
        &self.typ
    }
}

pub async fn validate_access_token(access_token: &str) -> Result<TokenData<serde_json::Value>, TokenValidationError> {
    let jwks = match fetch_jwks().await {
        Ok(jwks) => jwks,
        Err(_) => return Err(TokenValidationError::Unknown),
    };

    let jwk = match jwks.keys.get(1) {
        Some(jwk) => jwk,
        None => {
            log::error!("Nessuna chiave JWK trovata.");
            return Err(TokenValidationError::Unknown);
        }
    };

    let decoding_key = match DecodingKey::from_jwk(jwk) {
        Ok(key) => key,
        Err(err) => {
            log::error!("Errore nella generazione della chiave di decodifica dal JWK: {:?}", err);
            return Err(TokenValidationError::Unknown);
        }
    };

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["account"]);

    match jsonwebtoken::decode::<serde_json::Value>(access_token, &decoding_key, &validation) {
        Ok(token_data) => {
            log::info!("Access token successfully validate {:#?}", token_data);
            Ok(token_data)
        },
        Err(err) => {
            log::error!("Errore nella validazione dell'access token: {:?}", err);
            match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenValidationError::ExpiredSignature),
                _ => Err(TokenValidationError::Unknown)
            }
        }
    }
}

pub enum TokenValidationError {
    ExpiredSignature,
    Unknown,
}

pub async fn validate_id_token(id_token: &str, nonce: Option<String>) -> Result<IdTokenClaims, TokenValidationError> {
    let jwks = match fetch_jwks().await {
        Ok(jwks) => jwks,
        Err(_) => return Err(TokenValidationError::Unknown),
    };

    let jwk = match jwks.keys.get(1) {
        Some(jwk) => jwk,
        None => {
            log::error!("Nessuna chiave JWK trovata.");
            return Err(TokenValidationError::Unknown);
        }
    };

    let decoding_key = match DecodingKey::from_jwk(jwk) {
        Ok(key) => key,
        Err(err) => {
            log::error!("Errore nella generazione della chiave di decodifica dal JWK: {:?}", err);
            return Err(TokenValidationError::Unknown);
        }
    };

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["fast-photo-hub-rest-api"]);

    match jsonwebtoken::decode::<IdTokenClaims>(id_token, &decoding_key, &validation) {
        Ok(token_data) => {
            log::info!("Id token successfully validate {:#?}", token_data);

            let id_token_claims = token_data.claims;

            if let Some(id_token_nonce) = id_token_claims.nonce.clone() {
                if let Some(nonce) = nonce {
                    if id_token_nonce != nonce {
                        return Err(TokenValidationError::Unknown);
                    }
                }
            }

            Ok(id_token_claims)
        },

        Err(err) => {
            log::error!("Errore nella validazione del token ID: {:?}", err);
            match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenValidationError::ExpiredSignature),
                _ => Err(TokenValidationError::Unknown)
            }
        }
    }
}

async fn fetch_jwks() -> Result<JwkSet, reqwest::Error> {
    let http_client = Client::new();

    let jwks = match http_client
        .get("http://localhost:8080/realms/fast-photo-hub/protocol/openid-connect/certs")
        .send()
        .await
    {
        Ok(response) => match response.json::<JwkSet>().await {
            Ok(jwks) => jwks,
            Err(err) => {
                log::error!("Errore nel parsing del JSON del JWK: {:?}", err);
                return Err(err);
            }
        },
        Err(err) => {
            log::error!("Errore nella richiesta HTTP per il JWK: {:?}", err);
            return Err(err);
        }
    };

    Ok(jwks)
}