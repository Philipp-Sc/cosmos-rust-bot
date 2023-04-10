use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, get, App, Error, HttpServer, HttpResponse, web};
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
use actix_web::error::UrlGenerationError;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde_json::json;


use cosmos_rust_interface::utils::entry::*;
use cosmos_rust_interface::utils::entry::db::query::socket::client_send_query_request;

const QUERY_SOCKET: &str = "./tmp/cosmos_rust_bot_query_socket";


async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    // Check if the request matches the file route
    let is_file_request = req.path().starts_with("/static/");

    // If the request is a file request, skip the authentication check
    if is_file_request {
        Ok(req)
    }else {
    // For any other request the user needs to pass the authentication check
        if let (Some(Ok(token)), Ok(user_hash)) = (credentials.password().map(|pw| pw.parse::<u64>()), credentials.user_id().parse::<u64>()) {
            let request: UserQuery = UserQuery {
                query_part: QueryPart::AuthQueryPart(AuthQueryPart { token, user_hash }),
                settings_part: SettingsPart {
                    subscribe: None,
                    unsubscribe: None,
                    register: None,
                    user_hash: Some(user_hash)
                }
            };

            if let Ok(CosmosRustServerValue::Notification(n)) = client_send_query_request(QUERY_SOCKET, request) {
                log::info!("Notification: {:?}", &n);

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
}


async fn verify(_req: HttpRequest, _basic_auth: BasicAuth) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

async fn translate(_req: HttpRequest, info: web::Path<(String, String, String)>) -> actix_web::Result<HttpResponse> {
    let (lang, blockchain, id) = info.into_inner();

    // where is the dirty work done?

    // Now I just need to ask Cosmos-Rust-Bot to fetch a translation. CRB may or may not need to query GPT.
    // On Okay, a json file is returned, that by then should now exist.

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

async fn file(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let filepath = req.match_info().query("filepath");
    let ext = req.match_info().query("ext");

    // Check if the file extension is allowed
    if ext == "json" || ext == "html" {
        Ok(NamedFile::open(format!("./tmp/public/{}.{}", filepath, ext))?)
    } else {
        Err(UrlGenerationError::ResourceNotFound.into())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validator);
        App::new()
            //.wrap(auth) // to enable auth for all services
            .wrap(Cors::permissive())
            .service(
                web::resource("/verify")
                    .wrap(auth.clone())
                    .route(web::get().to(verify)),
            )
            .service(
                web::resource("/translate/{lang}/{blockchain}/{id}")
                    //.wrap(auth)
                    .route(web::get().to(translate)),
            )
            .service(
                web::resource("/static/{filepath:.*}.{ext:html|json}")
                    //.wrap(auth)
                    .route(web::get().to(file)),
            )
        })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
