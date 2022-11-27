use bstr::B;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use pam::Authenticator;
use sha2::Sha256;
use std::{collections::BTreeMap, env, time::SystemTime};

use super::models::Credentials;

fn check_credentials(creds: &Credentials) -> Result<(), String> {
    match Authenticator::with_password("login") {
        Ok(mut auth) => {
            auth.get_handler()
                .set_credentials(creds.login.clone(), creds.pass.clone());
            auth.authenticate().map(|_| ()).map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn load_key() -> Result<Hmac<Sha256>, String> {
    let secret = env::var("REMOUSE_SECRET").unwrap_or("DEFAULT".to_string());
    Hmac::<Sha256>::new_from_slice(B(&secret)).map_err(|e| e.to_string())
}

fn set_exp(epoch: u64) {
    env::set_var("REMOUSE_SECRET", "DEFAULT");
    env::set_var("REMOUSE_SECRET_EXP", epoch.to_string());
}

fn current_epoch() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn load_exp() -> Result<u64, String> {
    env::var("REMOUSE_SECRET_EXP")
        .map(|s| u64::from_str_radix(&s, 10).map_err(|e| e.to_string()))
        .unwrap_or_else(|_| {
            let epoch = current_epoch() + 1800;
            set_exp(epoch);
            Ok(epoch)
        })
}

fn generate_token(login: &str) -> Result<String, String> {
    let mut claims: BTreeMap<String, String> = BTreeMap::new();
    let key = load_key()?;
    let exp = load_exp()?;
    claims.insert("login".to_string(), login.to_string());
    claims.insert("exp".to_string(), exp.to_string());
    claims.sign_with_key(&key).map_err(|e| e.to_string())
}

fn read_claims(token: String) -> Result<BTreeMap<String, String>, String> {
    let key = load_key()?;
    let claims: BTreeMap<String, String> =
        token.verify_with_key(&key).map_err(|e| e.to_string())?;
    Ok(claims)
}

fn validate_claims<'a>(
    exp: &'a String,
    claims: &'a BTreeMap<String, String>,
) -> Result<&'a BTreeMap<String, String>, String> {
    let exp_epoch = u64::from_str_radix(exp, 10).map_err(|e| e.to_string())?;
    let cur_epoch = current_epoch();
    if exp_epoch >= cur_epoch {
        Ok(claims)
    } else {
        Err("Session Expired".to_string())
    }
}

pub fn login(creds: &Credentials) -> Result<String, String> {
    check_credentials(&creds)?;
    generate_token(&creds.login)
}

pub fn verify(token: String) -> Result<BTreeMap<String, String>, String> {
    let claims = read_claims(token)?;
    if let Some(exp) = claims.get("exp") {
        validate_claims(exp, &claims).cloned()
    } else {
        Err("Malformed Token".to_string())
    }
}

pub fn refresh(token: String) -> Result<String, String> {
    let claims = read_claims(token)?;
    if let Some(login) = claims.get("login") {
        generate_token(login)
    } else {
        Err("Malformed Token".to_string())
    }
}
