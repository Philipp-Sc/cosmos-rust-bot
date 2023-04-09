use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, get, App, Error, HttpServer, HttpResponse};
use actix_web_httpauth::extractors::basic::{BasicAuth};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::middleware::Logger;

use actix_web_httpauth::extractors::{AuthenticationError, bearer};
use actix_web_httpauth::headers::www_authenticate::basic::Basic;
use actix_web::HttpRequest;

use qstring::QString;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use reqwest::Client;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;

use actix_files::NamedFile;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;


use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::entry::db::query::socket::client_send_query_request;

const QUERY_SOCKET: &str = "./tmp/cosmos_rust_bot_query_socket";

/*
async fn validator(
    req: ServiceRequest,
    bearer_auth: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // the configured app_data config or a new config.
    let config = req.app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .realm("Restricted area")
        .scope("cosmos-rust-server");

    let request: UserQuery = UserQuery{ query_part: QueryPart::AuthQueryPart(AuthQueryPart{ token: bearer_auth.token().parse::<u64>().unwrap(), user_hash: 0u64 }), settings_part: SettingsPart {
        subscribe: None,
        unsubscribe: None,
        register: None,
        user_hash: Some(0u64)
    } };
    let response = client_send_query_request(QUERY_SOCKET,request).unwrap();
    if bearer_auth.token() == "0" || req.path().starts_with("/static") {
        Ok(req)
    } else {
        Err((AuthenticationError::from(config).into(), req))
    }
}*/

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    if let (Some(Ok(token)),Ok(user_hash)) = (credentials.password().map(|pw| pw.parse::<u64>()),credentials.user_id().parse::<u64>()){
        let request: UserQuery = UserQuery{ query_part: QueryPart::AuthQueryPart(AuthQueryPart{ token, user_hash }), settings_part: SettingsPart {
            subscribe: None,
            unsubscribe: None,
            register: None,
            user_hash: Some(user_hash)
        } };

        if let Ok(CosmosRustServerValue::Notification(n)) = client_send_query_request(QUERY_SOCKET,request) {

            let mut is_authorized = false;

            match n.query.query_part {
                QueryPart::AuthQueryPart(_) => {
                    for i in 0..n.entries.len() {
                        match &n.entries[i] {
                            CosmosRustBotValue::Authorization(auth) => {
                                is_authorized = auth.is_authorized;
                            }
                            _ => {
                                is_authorized = false;
                            }
                        }
                        break;
                    }
                },
                _ => {
                    is_authorized = false;
                }
            }

            if is_authorized {
                return Ok(req);
            }
        }
    }
    Err((Error::from(AuthenticationError::new(Basic::default())), req))
}


#[get("/verify")]
async fn verify(_req: HttpRequest, _bearer_auth: BearerAuth) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

/*
#[get("/subscriptions")]
async fn subscriptions(req: HttpRequest, bearer_auth: BearerAuth) -> actix_web::Result<NamedFile> {

    // use auth token, to request

    let qs = QString::from(req.query_string());
    if let Some(question_parameter) = (qs.get("question")) {

    }
    Ok(NamedFile::open("dummy.png")?)
}

#[get("/static/{filename:.+\\.json}")]
async fn file_exists(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let filename = req.match_info().query("filename");
    let file_path = format!("./static/{}", filename);
    NamedFile::open(&file_path)?;

    let response_body = json!({
        "success": true,
        "message": "File retrieved successfully",
        "filename": filename
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response_body))
}
*/

/*
#[get("/static/{filename:.*}")]
async fn file(req: HttpRequest, _bearer_auth: BearerAuth) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(format!("./static/{}",req.match_info().query("filename")))?)
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            /*.app_data(
                bearer::Config::default()
                    .realm("Restricted area")
                    .scope("cosmos-rust-server"),
            )
            .wrap(HttpAuthentication::bearer(validator))
            */
            .wrap(Logger::default())
            .wrap(HttpAuthentication::basic(validator))
            // ensure the CORS middleware is wrapped around the httpauth middleware so it is able to
            // add headers to error responses
            .wrap(Cors::permissive())
            //.service(file_exists)
            .service(verify)
        //.service(subscriptions)
        //.service(file)
    })
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}
