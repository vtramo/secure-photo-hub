use serde::Deserialize;

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
