use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::vo::{VoUser, Token};

const TOKEN_KEY: &[u8] = "token_key".as_bytes();

#[derive(Deserialize, Serialize)]
struct Claims {
    pub user: VoUser,
    pub is_refresh: bool,
    pub exp: i64,
}

fn gen_token(user: &VoUser, is_refresh: bool) -> String {
    let now = Utc::now().timestamp();
    let claims = Claims {
        user: user.clone(),
        is_refresh,
        exp: if is_refresh {
            now + 60 * 60 * 24 * 7
        } else {
            now + 60 * 60
        },
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TOKEN_KEY),
    )
    .unwrap()
}

pub fn gen_user_token(user: VoUser) -> Token {
    let access_token = gen_token(&user, false);
    // let refresh_token = gen_token(&user, true);
    Token::new(access_token, user)
}

pub fn validate_token(token: &str) -> Option<VoUser> {
    if token.is_empty() {
        return None;
    }
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(TOKEN_KEY),
        &Validation::default(),
    ) {
        Ok(c) if !c.claims.is_refresh => Some(c.claims.user),
        _ => None,
    }
}

pub fn validate_refresh_token(token: &str) -> Option<VoUser> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(TOKEN_KEY),
        &Validation::default(),
    ) {
        Ok(c) if c.claims.is_refresh => Some(c.claims.user),
        _ => None,
    }
}
