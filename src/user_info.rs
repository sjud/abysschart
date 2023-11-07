use std::cmp::Ordering;

use super::*;

#[derive(Debug, Clone, PartialEq,Default,Serialize,Deserialize)]
pub enum PlatformRole{
    Admin,
    Subscriber,
    #[default]
    FreePlayer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrdPlatformRole(pub PlatformRole);
impl PartialOrd for OrdPlatformRole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = match self.0 {
            PlatformRole::FreePlayer => 0,
            PlatformRole::Subscriber => 1,
            PlatformRole::Admin => 2,
        };
        let other_val = match other.0 {
            PlatformRole::FreePlayer => 0,
            PlatformRole::Subscriber => 1,
            PlatformRole::Admin => 2,
        };
        if self_val < other_val {
            Some(Ordering::Less)
        } else if self_val > other_val {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug, Clone, PartialEq,Default, Serialize, Deserialize)]
pub struct Auth {
    /// Current user's id.
    pub user_id: uuid::Uuid,
    /// The role with the platform.
    pub platform_role: PlatformRole,
}


cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        use axum::{
            extract::FromRequestParts,
            http::{
                StatusCode,
                request::Parts,
            },
        };
        use jwt::VerifyWithKey;
        use axum::RequestPartsExt;
        use axum_extra::{extract::cookie::CookieJar };
        #[async_trait::async_trait]
        impl FromRequestParts<server_state::ServerState> for Option<Auth> {
            type Rejection = (StatusCode,String);

            async fn from_request_parts(
                parts: &mut Parts,
                state: &server_state::ServerState
            ) -> Result<Self,Self::Rejection> {
                let jar: CookieJar = parts.extract::<CookieJar>().await
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR,format!("{:?}",err)))?;

                let jwt = jar.get("jwt").map(|cookie|cookie.value());


                if let Some(jwt) = jwt {
                    let key = state.key.as_ref();
                    let auth : Auth = jwt.verify_with_key(key)
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR,format!("{:?}",err)))?;
                    Ok(Some(auth))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct UserInfo {
    /// the deserialized jwt rust struct
    pub inner: Option<Auth>,
}



impl UserInfo {
    pub fn add_auth(&mut self, inner: Auth) {
        self.inner = Some(inner);
    }

    /// Creates a new user info with jwt updates.
    pub fn add_jwt(&mut self, jwt: &str) -> Result<(), String> {
        use jwt::{Header, Token};
        let token: Token<Header, Auth, _> =
            Token::parse_unverified(jwt).map_err(|err| format!("{err:?}"))?;
        let auth: Auth = token.claims().clone();
        self.inner = Some(auth);
        Ok(())
    }

    pub fn is_logged_in(&self) -> bool {
        self.inner.is_some()
    }
    /// If the current user is logged in, return the user's id.
    pub fn current_user_id(&self) -> Option<uuid::Uuid> {
        self.inner.as_ref().map(|inner| inner.user_id)
    }
}

