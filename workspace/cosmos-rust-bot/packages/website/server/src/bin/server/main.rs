use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, get, App, Error, HttpServer}; 
use actix_web_httpauth::extractors::basic::{BasicAuth};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::middleware::Logger; 
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::basic::Basic;
use actix_web::HttpRequest;

use qstring::QString;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use reqwest::Client; 
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;

use std::io::Cursor;
type CustomResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use actix_files::NamedFile;


use any_ascii::any_ascii;

const PATH_TO_IMAGES: &'static str = "./dalle";
const PATH_TO_CARDS: &'static str = "./gpt3";
const OPENAI_API_KEY: &'static str = "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";


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Url {
    url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct OpenAIImagesGenerations {
    pub created: f64,
    pub data: Vec<Url>,
}

/*
{
    "created": 1589478378,
    "data": [
        {
        "url": "https://..."
        },
        {
        "url": "https://..."
        }
    ]
}
*/
  

      

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // TODO: connect to database and check if user exists and hashed password is correct.
    eprintln!("{:?}", credentials);
    if credentials.user_id() == "librelearning" && credentials.password() == Some("123") {
        Ok(req)
    }else{
        Err((Error::from(AuthenticationError::new(Basic::default())), req)) 
    }
}


#[get("/gpt3")]
async fn index(req: HttpRequest, auth: BasicAuth) -> actix_web::Result<NamedFile> {
   
    let qs = QString::from(req.query_string());
    if let (Some(question_parameter)) = (qs.get("question")) {

        let question = any_ascii(&question_parameter.to_lowercase()); 

	    let prompt_bias = r#"as "Digital art" themed "Fallout 2" in "Vintage" style by "Leonardo da Vinci"."#;

        let query = format!("\"{}\" {}",&question,&prompt_bias);

        let hashed_query = calculate_hash(&query);

        let already_exists = std::path::Path::new(&format!("{}/0_{}.json",PATH_TO_CARDS,&hashed_query)).exists();

        if already_exists { 
            return Ok(NamedFile::open(format!("{}/0_{}.json",PATH_TO_CARDS,&hashed_query))?);
        }

        if query.len() < 2*256 { // about the size of two max length tweets

            let json_data = serde_json::json!({
                "model": "text-davinci-002",
                "prompt": "Say this is a test",
                "max_tokens": 6,
                "temperature": 0,
                "top_p": 1,
                "n": 1,
                "stop": "\n"
              });

            let client = Client::new();
            let url = "https://api.openai.com/v1/completions";
            let response = client.post(url)
            .bearer_auth(OPENAI_API_KEY)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(json_data.to_string())
            .send().await;
            let response = if let Ok(resp) = response {
				resp.json::<OpenAIImagesGenerations>().await.ok()
	    		   }else{
	        		None
	    	           };


            let json_response = response.as_ref();
            eprintln!("{:?}", json_response);

            // write to file. if result parsing complete and correct.

            if success {
                return Ok(NamedFile::open(format!("{}/0_{}.png",PATH_TO_IMAGES,&hashed_query))?);
            } 
        }
    }
    Ok(NamedFile::open("dummy.png")?)
}

 


#[get("/dalle")]
async fn index(req: HttpRequest, auth: BasicAuth) -> actix_web::Result<NamedFile> {
   
    let qs = QString::from(req.query_string());
    if let (Some(context_parameter),Some(label_parameter)) = (qs.get("context"),qs.get("label")) {

        let context = any_ascii(&context_parameter.to_lowercase());
        let label = any_ascii(&label_parameter.to_lowercase()); 

	// (digital art, the original, Youtube Thumbnail) 
        // (digital art, emotional, memorable, high quality)
        // "Summary", "Fallout 2", "Vintage", "Digital art", "..." 
        // " as "Digital art" themed "Fallout 2" in "Vintage" style by "Leonardo da Vinci".

	// fallout new vegas
        // star wars
        // etc..

	let prompt_bias = r#"as "Digital art" themed "Fallout 2" in "Vintage" style by "Leonardo da Vinci"."#;

        let queries = vec![
            format!("\"{} ({})\" {}",&context,&label,&prompt_bias),
            format!("\"{} ({})\" {}",&label,&context,&prompt_bias)
            ];
        let hashed_queries = vec![
            calculate_hash(&queries[0]),
            calculate_hash(&queries[1])
            ];

        for i in 0..hashed_queries.len() {
            let already_exists = std::path::Path::new(&format!("{}/0_{}.png",PATH_TO_IMAGES,&hashed_queries[i])).exists();

            if already_exists { 
                return Ok(NamedFile::open(format!("{}/0_{}.png",PATH_TO_IMAGES,&hashed_queries[i]))?);
            }
        }
        let query = &queries[0];
        let hashed_query = hashed_queries[0];

        if query.len() < 2*256 { // about the size of two max length tweets

            let json_data = serde_json::json!({
                "prompt" :  query,
                "n" : 1,
                "size" : "256x256", 
            });

            let client = Client::new();
            let url = "https://api.openai.com/v1/images/generations";
            let response = client.post(url)
            .bearer_auth(OPENAI_API_KEY)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(json_data.to_string())
            .send().await;
            let response = if let Ok(resp) = response {
				resp.json::<OpenAIImagesGenerations>().await.ok()
	    		   }else{
	        		None
	    	           };


            let json_response = response.as_ref();
            eprintln!("{:?}", json_response);

            let mut success = true;
            if let Some(ref images_generations) = response {
                for i in 0..images_generations.data.len() {
                    if let Err(_) = fetch_url(images_generations.data[i].url.to_owned(),format!("{}/{}_{}.png",PATH_TO_IMAGES,i,hashed_query)).await {
                        success = false;
                    }
                }
            }else{
                success = false;
            }

            if success {
                return Ok(NamedFile::open(format!("{}/0_{}.png",PATH_TO_IMAGES,&hashed_query))?);
            } 
        }
    }
    Ok(NamedFile::open("dummy.png")?)
}

async fn fetch_url(url: String, file_name: String) -> CustomResult<()> {
    let response = reqwest::get(url).await?;
    let mut f = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut f)?;
    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[get("/static/{filename:.*}")]
async fn file(req: HttpRequest) -> actix_web::Result<NamedFile> { 
    Ok(NamedFile::open(format!("./static/{}",req.match_info().query("filename")))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let auth = HttpAuthentication::basic(validator);
        App::new() 
            // ensure the CORS middleware is wrapped around the httpauth middleware so it is able to
            // add headers to error responses 
            .wrap(Logger::default())
            .wrap(auth) 
            .wrap(Cors::permissive())
            .service(index)
            .service(file)
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
