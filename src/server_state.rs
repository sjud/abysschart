use super::*;
use std::{
    collections::HashMap,
    sync::Arc,
};
use sha2::Sha256;
use hmac::Hmac;
#[derive(Debug,Clone,PartialEq,Default)]
pub struct GameState{
    pub characters:HashMap<uuid::Uuid,windows::CharacterData>,
}

#[derive(Debug,Clone)]
pub struct ServerState{
    pub game_state:GameState,
    pub key:Arc<Hmac<Sha256>>,
}

impl ServerState{
    pub fn new(key:Arc<Hmac<Sha256>>) -> Self {
        Self{
            key,
            game_state:GameState::default(),
        }
    }
}