use super::*;


#[server(PostLogin)]
pub async fn post_login(
    email: String,
    password: String,
    remember: Option<bool>,
) -> Result<(String, user_info::Auth), ServerFnError> {
    let state: server_state::ServerState = use_context::<server_state::ServerState>()
        .ok_or(ServerFnErrorErr::ServerError("No server state".to_string()))?;
    let key = state.key.as_ref();
  

        use jwt::SignWithKey;

        let auth = user_info::Auth::default();

        let jwt = auth
            .clone()
            .sign_with_key(key)
            .map_err(|err| backend_utils::handle_server_err(err, "Server Error: Authorization.".to_string()))?;

        use axum_extra::extract::cookie::*;
        use time::{Duration, OffsetDateTime};

        let cookie = Cookie::build("jwt", jwt.clone())
            .secure(true)
            // .http_only(true)
            .path("/")
            .same_site(SameSite::None)
            .max_age(Duration::weeks(2));

        let cookie = if remember == Some(true) {
            cookie.expires(
                OffsetDateTime::now_utc()
                    .checked_add(Duration::weeks(2))
                    .ok_or(ServerFnErrorErr::ServerError("Time error.".to_string()))?,
            )
        } else {
            cookie
        };

        let cookie = cookie.finish();
        

        Ok((cookie.to_string(), auth))
}

#[island]
pub fn Login() -> impl IntoView {
    let state: RwSignal<ClientState> = expect_context::<RwSignal<ClientState>>();
    let login = create_server_action::<PostLogin>();
    let jwt_resp = login.value().read_only();
    let msg_set = create_write_slice( state, |state, val| state.user_msg = val);
    let add_auth = create_write_slice(state,|state,auth|state.user_info.add_auth(auth));
    create_effect( move |_| {
        if let Some(Ok((jwt, auth))) = jwt_resp() {
            add_auth(auth);
            use wasm_bindgen::JsCast;
            use web_sys::HtmlDocument;
            document()
                .dyn_into::<HtmlDocument>()
                .unwrap()
                .set_cookie(&jwt)
                .unwrap();
        } else if let Some(Err(ServerFnError::ServerError(err))) = jwt_resp() {
            msg_set(user_msg::UserMsg {
                theme: user_msg::MsgTheme::Red,
                header: "Login Error".to_string(),
                body: format!("{}", err),
            });
        }
    });

 

    view!{
        <div class="rounded-lg w-[17rem] m-auto p-4 sm:mt-32"  >
        <ActionForm action=login>
        <div class="mt-2">
        <label for="email" class="text-slate-50" >{"Email"}</label>
        </div>
        <div>
        <input class="rounded"
        type="email" name="email" id="email" placeholder="Email"/>
        </div>


        <div class="mt-2">
        <label for="password" class="text-slate-50" >{"Password"}</label>
        </div>
        <div>
        <input class="rounded"
        type="password" name="password" id="password" placeholder="Password" />
        </div>

        <div  class="mt-4">
        //<A href="/forgot_login">
            <span class="text-slate-50 underline hover:text-green-200">{"Forgot your password?"}</span>
        //</A>
        </div>


        <div class="mt-4">
        <label for="remember" class="text-slate-50" >{"Remember Login"}</label>
         <input class="ml-4" type="checkbox" id="remember" name="remember" value="true" />
        </div>

        <div class="mt-4">
        <input type="submit" class="text-slate-50 underline hover:text-green-200 text-2xl" value="Login"/>
        </div>
        </ActionForm>
        <div class="mt-4">
        //<A href="/register">
            <span class="text-slate-50 hover:text-green-200 underline">{"Register a new account instead?"}</span>
        //</A>
        </div>
        </div>
    }
}