use super::*;

#[derive(Copy,Clone,Debug,PartialEq,Default)]
pub enum BackgroundImage{
    #[default]
    Homepage,
    None,
}

impl BackgroundImage{
    /// Will output a line of css "background-image: url(...);"
    pub fn to_css(&self) -> &'static str {
        match &self {
            Self::Homepage => "background-image: url(/backgrounds/homepage.png)",
            Self::None => {""},

        }
       }
}


#[island]
pub fn SetBody() -> impl IntoView {
    let state: RwSignal<ClientState> = expect_context::<RwSignal<ClientState>>();
    let read_bg = create_read_slice(
        state,
        |state| state.background_img.clone(),
    );

    view!{
        {move || {
            let body_style = move || format!("{};background-repeat: repeat;",read_bg().to_css());
            view!{
                <Body attr:style=body_style()/>
            }
        }
    }
    }
}