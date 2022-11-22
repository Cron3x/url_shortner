mod sqlite_lib;
mod gen_url;

#[macro_use] extern crate rocket;
use rocket::{serde::{json::Json, Deserialize, Serialize}, response::{Redirect, content::RawHtml}};
use sqlite_lib::Url;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UrlSchemetic{
    url: String
}

#[post("/gen", data = "<url>")]
fn gen(url: Json<UrlSchemetic>) -> Json<String>{
    
    let long_url = url.0.url;

    let short_url = gen_url::gen_url_32(&long_url);
    let url_struct = Url{
        long_url: long_url,
        short_url: short_url.clone()
    };
    let db = sqlite_lib::DBLib::new("urls.db", "urls");
    match db.write(url_struct) {
        Ok(_) => return Json(short_url),
        Err(e) => return Json(e.to_string())
    };
}

#[get("/<url>")]
fn index(url:String) -> Redirect{
    let db = sqlite_lib::DBLib::new("urls.db", "urls");
    let urls  = match db.read(){
        Ok(u) => u,
        Err(e) => panic!("paniced {}",e),
    };
    for i in urls {
        if i.short_url == url{
            return Redirect::to(i.long_url)
        }
    }
    Redirect::to(uri!("/url_error"))
}

#[get("/url_error")]
fn url_error() -> RawHtml<&'static str>{

    RawHtml("there is no such url!")
}

#[launch]
fn rocket() -> _{
    rocket::build().mount("/", routes![index,gen,url_error])
}