use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IdTokenClaims {
    acr: String,
    at_hash: String,
    aud: String,
    auth_time: Option<u64>,
    azp: String,
    email: Option<String>,
    email_verified: bool,
    exp: u64,
    family_name: Option<String>,
    given_name: Option<String>,
    iat: u64,
    iss: String,
    jti: String,
    name: Option<String>,
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

    pub fn auth_time(&self) -> Option<u64> {
        self.auth_time.clone()
    }

    pub fn azp(&self) -> &str {
        &self.azp
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn exp(&self) -> u64 {
        self.exp
    }

    pub fn family_name(&self) -> Option<String> {
        self.family_name.clone()
    }

    pub fn given_name(&self) -> Option<String> {
        self.given_name.clone()
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

    pub fn name(&self) -> Option<String> {
        self.name.clone()
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
