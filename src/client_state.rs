use super::*;


#[derive(Debug,Clone,PartialEq,Default)]
pub struct ClientState{
    pub user_info: user_info::UserInfo,
    pub user_msg: user_msg::UserMsg,
    pub show_windows:windows::ShowWindows,
    pub game_data:GameData,
    pub background_img:body::BackgroundImage,
}

#[derive(Debug,Clone,PartialEq,Default)]
pub struct GameData{
    pub character_data:Option<windows::CharacterData>,
}

