use super::*;


#[component]
pub fn SideBar() -> impl IntoView{
    view!{
        <div class="top-0 fixed bg-slate-500 flex flex-col space-y-0 space-x-0 justify-start">
            <SideBarButton button_type=SideBarButtonType::Settings/>
            <SideBarButton button_type=SideBarButtonType::Character/>
            <SideBarButton button_type=SideBarButtonType::Ship/>
        </div>
    }
}

#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq)]
pub enum SideBarButtonType{
    Character,
    Settings,
    Ship,
}

#[island]
pub fn SideBarButton(button_type:SideBarButtonType) -> impl IntoView {
    let state = expect_context::<RwSignal<ClientState>>();
    let flip_windows = create_write_slice(
        state,
        move |state,_| match button_type {
            SideBarButtonType::Character => state.show_windows.character=!state.show_windows.character,
            SideBarButtonType::Settings =>state.show_windows.settings=!state.show_windows.settings,
            SideBarButtonType::Ship => state.show_windows.ship=!state.show_windows.ship,       
        }
    );
    let image_url = match button_type {
        SideBarButtonType::Character => "/sidebar_icons/generic_character.png",
        SideBarButtonType::Settings => "/sidebar_icons/settings.png",
        SideBarButtonType::Ship => "/sidebar_icons/generic_spaceship.png",
    };
    

    view!{
        <div class="m-0 p-0">
        <img src=image_url class="hover:opacity-50 w-12" on:click=move |_| flip_windows(())/>
        </div>
    }
}