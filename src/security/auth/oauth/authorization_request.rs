use std::fmt::{Display, Formatter};

use base64::Engine;
use reqwest::Url;
use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OAuthAuthorizationRequestState {
    state: String,
    nonce: String,
    code_verifier: String,
    code_verifier_digest: String,
}

impl OAuthAuthorizationRequestState {
    pub fn random() -> Self {
        let state = Self::generate_state();
        let (code_verifier, code_verifier_digest) = Self::generate_code_verifier();
        let nonce = Self::generate_nonce();
        Self::new(&state, &nonce, &code_verifier, &code_verifier_digest)
    }

    pub fn new(state: &str, nonce: &str, code_verifier: &str, code_verifier_digest: &str) -> Self {
        Self {
            state: state.to_string(),
            nonce: nonce.to_string(),
            code_verifier: code_verifier.to_string(),
            code_verifier_digest: code_verifier_digest.to_string(),
        }
    }

    fn generate_state() -> String {
        let mut buf = [0u8; 64];
        let system_random = ring::rand::SystemRandom::new();
        system_random.fill(&mut buf).expect("Should be ok");
        base64::prelude::BASE64_URL_SAFE.encode(buf)
    }

    fn generate_code_verifier() -> (String, String) {
        let mut buf = [0u8; 64];
        let system_random = ring::rand::SystemRandom::new();
        system_random.fill(&mut buf).expect("Should be ok");
        let code_verifier = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(buf);
        let code_verifier_digest =
            ring::digest::digest(&ring::digest::SHA256, code_verifier.as_bytes());
        let code_verifier_digest =
            base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(code_verifier_digest.as_ref());
        (code_verifier, code_verifier_digest)
    }

    fn generate_nonce() -> String {
        let mut buf = [0u8; 64];
        let system_random = ring::rand::SystemRandom::new();
        system_random.fill(&mut buf).expect("Should be ok");
        base64::prelude::BASE64_URL_SAFE.encode(buf)
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn code_verifier(&self) -> &str {
        &self.code_verifier
    }

    pub fn code_verifier_digest(&self) -> &str {
        &self.code_verifier_digest
    }
}

#[derive(Debug)]
pub struct OAuthSecureAuthorizationRequest {
    authorization_endpoint: Url,
    client_id: String,
    redirect_uri: Url,
    state: OAuthAuthorizationRequestState,
    response_type: OAuthResponseType,
    scope: Vec<String>,
}

impl OAuthSecureAuthorizationRequest {
    const CLIENT_ID_PARAM: &'static str = "client_id";
    const REDIRECT_URI_PARAM: &'static str = "redirect_uri";
    const STATE_PARAM: &'static str = "state";
    const RESPONSE_TYPE_PARAM: &'static str = "response_type";
    const SCOPE_PARAM: &'static str = "scope";
    const CODE_CHALLENGE_PARAM: &'static str = "code_challenge";
    const CODE_CHALLENGE_METHOD_PARAM: &'static str = "code_challenge_method";
    const CODE_CHALLENGE_S256_METHOD: &'static str = "S256";
    const NONCE_PARAM: &'static str = "nonce";

    pub fn code_flow(
        authorization_endpoint: &Url,
        client_id: &str,
        scope: &Vec<String>,
        redirect_uri: &Url,
    ) -> Self {
        Self {
            authorization_endpoint: authorization_endpoint.clone(),
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.clone(),
            state: OAuthAuthorizationRequestState::random(),
            response_type: OAuthResponseType::Code,
            scope: scope.clone(),
        }
    }

    pub fn state(&self) -> &OAuthAuthorizationRequestState {
        &self.state
    }
}

impl From<&OAuthSecureAuthorizationRequest> for Url {
    fn from(authorization_request: &OAuthSecureAuthorizationRequest) -> Self {
        let mut authorization_endpoint = authorization_request.authorization_endpoint.clone();

        authorization_endpoint
            .query_pairs_mut()
            .append_pair(
                OAuthSecureAuthorizationRequest::CLIENT_ID_PARAM,
                &authorization_request.client_id,
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::REDIRECT_URI_PARAM,
                &authorization_request.redirect_uri.to_string(),
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::STATE_PARAM,
                &authorization_request.state.state,
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::RESPONSE_TYPE_PARAM,
                &authorization_request.response_type.to_string(),
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::SCOPE_PARAM,
                &authorization_request.scope.join(" "),
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::CODE_CHALLENGE_PARAM,
                &authorization_request.state.code_verifier_digest,
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::CODE_CHALLENGE_METHOD_PARAM,
                OAuthSecureAuthorizationRequest::CODE_CHALLENGE_S256_METHOD,
            )
            .append_pair(
                OAuthSecureAuthorizationRequest::NONCE_PARAM,
                &authorization_request.state.nonce,
            );

        authorization_endpoint
    }
}

#[derive(Debug)]
pub enum OAuthResponseType {
    Code,
}

impl Display for OAuthResponseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthResponseType::Code => f.write_str("code"),
        }
    }
}
