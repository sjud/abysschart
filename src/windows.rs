use super::*;

#[derive(Clone,Debug,Default,PartialEq)]
pub struct ShowWindows{
    pub character:bool,
    pub settings:bool,
    pub ship:bool,
}



#[island]
pub fn WindowAggregator() -> impl IntoView {
    let state: RwSignal<ClientState> = expect_context::<RwSignal<ClientState>>();
    let read_windows = create_read_slice(
        state,
        |state| state.show_windows.clone(),
    );

    view!{
            <Show
            when = move || read_windows().character
            fallback = || view!{}
            >
            <CharacterWindow/>
            </Show>
            <Show
            when = move || read_windows().settings
            fallback = || view!{}
            >
            <SettingsWindow/>
            </Show>
            <Show
            when = move || read_windows().ship
            fallback = || view!{}
            >
            <ShipWindow/>
            </Show>
        }
}




#[derive(Debug,PartialEq,Clone)]
pub struct CharacterData{
    pub character_id:uuid::Uuid,
    pub skill_points:usize,
    pub name:String,
    pub account_id:uuid::Uuid,
    pub creation_ts:time::PrimitiveDateTime,
    pub skill_sheet: SkillSheet,
}

#[derive(Debug,PartialEq,Clone)]
pub struct SkillSheet{
    pub field_1:u8,
    pub field_2:u8,
    pub field_3:u8,
    pub field_4:u8,
    pub field_5:u8,
}

#[component]
pub fn CharacterWindow() -> impl IntoView {
    let state = expect_context::<RwSignal<ClientState>>();
    let character_data = create_read_slice(state,
        |state| state.game_data.character_data.clone()
    );
    
    view!{
        <div class="w-[80vw] h-[70vh] bg-gray-700">
            <img src="/sidebar_icons/generic_character.png" class="w-40 h-40"/>
            <p class="text-white">{ move || character_data().map(|character|character.name).unwrap_or("UnNamed?".to_string())}</p>            
        </div>
    }

}

#[component]
pub fn SettingsWindow() -> impl IntoView {

}

#[component]
pub fn ShipWindow() -> impl IntoView {

}