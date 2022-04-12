use actix_web::{dev, HttpResponse};
use awc::ClientResponse;

pub trait IntoHttpResponse {
  fn into_http_response(self) -> HttpResponse;

  fn into_wrapped_http_response<E>(self) -> Result<HttpResponse, E>
  where
    Self: Sized,
  {
    Ok(self.into_http_response())
  }
}

impl IntoHttpResponse
  for ClientResponse<dev::Decompress<dev::Payload>>
{
  fn into_http_response(self) -> HttpResponse {
    let mut response = HttpResponse::build(self.status());

    self.headers().iter().for_each(|(k, v)| {
      response.insert_header((k, v.clone()));
    });

    // TODO: other stuff than header and status (e.g. extensions or
    // stuff like that)

    response.streaming(self)
  }
}

pub mod util {
  use actix_web::{get, web, HttpResponse, error};
  use actix_web::web::Data;
  use awc::Client;

  use super::IntoHttpResponse;

  pub fn google_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Data::new(Client::default())).service(google_proxy);
  }

  #[get("/{url:.*}")]
  pub async fn google_proxy(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
  ) -> actix_web::Result<HttpResponse> {
    let url = format!(
      "https://www.google.com/{}",
      path.into_inner().0
    );

    client.get(&url).send().await.map_err(|e| error::ErrorInternalServerError(e))?.into_wrapped_http_response()
  }
}
