use rocket::{get, http::Status};

#[get("/health_check")]
pub async fn health_check() -> Status {
    Status::Ok
}
