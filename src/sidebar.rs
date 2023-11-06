use super::*;


#[component]
pub fn SideBar() -> impl IntoView{
    view!{
        <div>
            <SideBarButton button_type=SideBarButtonType::Settings/>
            <SideBarButton button_type=SideBarButtonType::Character/>
            <SideBarButton button_type=SideBarButtonType::Ship/>
        </div>
    }
}

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
pub enum SideBarButtonType{
    Character,
    Settings,
    Ship,
}

#[component]
pub fn SideBarButton(button_type:SideBarButtonType) -> impl IntoView{
    let image_url = match button_type {
        SideBarButtonType::Character => "/sidebar_icons/generic_character.png",
        SideBarButtonType::Settings => "/sidebar_icons/settings.png",
        SideBarButtonType::Ship => "/sidebar_icons/generic_spaceship.png",
    };
    let bg = expect_context::<RwBackgroundImage>();

    view!{
        <button class="h-12 w-12"  on:click=move|_| {
            bg.0.write_only()(BackgroundImage::None);
        }  >
        <img src=image_url class="hover:opacity-50" />
        </button>
    }
}