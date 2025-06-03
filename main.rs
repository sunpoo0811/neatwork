mod util;

use casdoor_rust_sdk::{AuthService, CasdoorConfig};
use casdoor_rust_sdk::{CasdoorUser, UserService};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::tokio::task;
use rocket::{Request, Response};
use util::abs_path;

#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TokenUser {
    token: String,
    user: CasdoorUser,
}

#[get("/login")]
fn login() -> Result<Json<String>, String> {
    let path = abs_path("conf.toml").map_err(|err| {
        let error_msg = format!("Error getting the absolute path of conf.toml: {:?}", err);
        eprintln!("{}", &error_msg);
        error_msg
    })?;

    let conf = CasdoorConfig::from_toml(&path).map_err(|err| {
        let error_msg = format!("Error parsing the configuration file: {:?}", err);
        eprintln!("{}", &error_msg);
        error_msg
    })?;

    let auth_service = AuthService::new(&conf);
    // 下方函数中的地址并非用于redirect_url，而是在redirect_url（也就是casdoor前端登录地址）登录成功后跳转的地址（也就是函数中地址拼接code码）
    // let redirect_url = auth_service.get_signin_url("http://localhost:8080/callback".to_string());
    let redirect_url = auth_service.get_signin_url("http://localhost:8080/callback".to_string());
    println!("登录重定向redirect_url:{}", &redirect_url);
    Ok(Json(redirect_url))
}

#[get("/signup")]
fn signup() -> Json<String> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let auth_service = AuthService::new(&conf);
    let redirect_url = auth_service.get_signup_url_enable_password();
    Json(redirect_url)
}

#[get("/auth/<code>")]
async fn callback(code: String) -> Result<Json<TokenUser>, String> {
    println!("前端发送使用code:{}换取token", &code);
    let user_result = task::spawn_blocking(move || {
        let conf_path = abs_path("conf.toml").map_err(|_| {
            let err_msg = "Cannot find conf.toml".to_string();
            eprintln!("abs_path() error: {}", err_msg);
            err_msg
        })?;
        let conf_str = conf_path.as_str();
        let conf = CasdoorConfig::from_toml(conf_str).map_err(|_| {
            let err_msg = "Failed to parse TOML config".to_string();
            eprintln!("from_toml() error: {}", err_msg);
            err_msg
        })?;

        let auth_service = AuthService::new(&conf);
        let token = auth_service.get_auth_token(code).map_err(|e| {
            let err_msg = e.to_string();
            eprintln!("get_auth_token() error: {}", err_msg);
            err_msg
        })?;
        println!("换取到的token: {}", token);
        let user = auth_service
            .parse_jwt_token(token.to_string())
            .map_err(|e| {
                let err_msg = e.to_string();
                eprintln!("parse_jwt_token() error: {}", err_msg);
                err_msg
            })?;
        println!("解析token后得到的用户: {:?}", user);
        Ok(TokenUser { token, user })
    })
    .await
    .map_err(|_| {
        let err_msg = "Failed to process in spawn_blocking".to_string();
        eprintln!("{}", err_msg);
        err_msg
    })?;
    println!("User: {:?}", user_result);

    match user_result {
        Ok(TokenUser { token, user }) => Ok(Json(TokenUser { token, user })),
        Err(e) => Err(e),
    }
}

#[get("/logout")]
fn logout() -> Redirect {
    Redirect::to("/")
}

#[get("/user/count/<is_online>")]
async fn user_count(is_online: String) -> Json<i64> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);

    let count = user_service.get_user_count(is_online).await.unwrap();
    Json(count)
}

#[get("/user/<name>")]
async fn get_user(name: String) -> Json<CasdoorUser> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);

    let user = user_service.get_user(name).await.unwrap();
    Json(user)
}

#[get("/user/list")]
async fn get_user_list() -> Json<Vec<CasdoorUser>> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);

    let users = user_service.get_users().await.unwrap();
    Json(users)
}

#[post("/user/delete", data = "<user>")]
async fn delete_user(user: Json<CasdoorUser>) -> Json<u16> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);

    let code = user_service.delete_user(user.0).await.unwrap();
    Json(code.as_u16())
}

#[post("/user/add", data = "<user>")]
async fn add_user(user: Json<CasdoorUser>) -> Json<u16> {
    let conf = CasdoorConfig::from_toml(abs_path("conf.toml").unwrap().as_str()).unwrap();
    let user_service = UserService::new(&conf);

    let code = user_service.add_user(user.0).await.unwrap();
    Json(code.as_u16())
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/api",
        routes![
            login,
            signup,
            callback,
            logout,
            user_count,
            get_user,
            get_user_list,
            delete_user,
            add_user,
        ],
    )
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
