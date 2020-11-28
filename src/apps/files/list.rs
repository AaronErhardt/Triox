use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, Error, HttpResponse};

use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};

use super::QueryPath;
use crate::jwt;
use crate::AppState;

/// File list returned by the `list` and `list_root` services as JSON
#[derive(serde::Serialize)]
struct Response {
    files: Vec<String>,
    dirs: Vec<String>,
}

/// Service for listing files via an API
#[get("/app/files/list")]
pub async fn list(
    app_state: web::Data<AppState>,
    jwt: jwt::JWT,
    web::Query(query_path): web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    let claims = jwt::extract_claims(&jwt.0, &app_state.config.jwt.secret).await?;

    let full_path = super::resolve_path(claims.id, &query_path.path);

    let mut dir: ReadDir = fs::read_dir(&full_path)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut entries: Vec<DirEntry> = Vec::new();

    loop {
        let new_entry = dir.next_entry().await;
        if let Ok(Some(entry)) = new_entry {
            entries.push(entry);
        } else {
            break;
        }
    }

    let mut files: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();

    for entry in entries {
        match entry
            .file_type()
            .await
            .map_err(ErrorInternalServerError)?
            .is_file()
        {
            true => files.push(entry.file_name().into_string().map_err(|err| {
                ErrorInternalServerError(
                    err.to_str()
                        .unwrap_or("String conversion failed")
                        .to_owned(),
                )
            })?),
            false => dirs.push(entry.file_name().into_string().map_err(|err| {
                ErrorInternalServerError(
                    err.to_str()
                        .unwrap_or("String conversion failed")
                        .to_owned(),
                )
            })?),
        }
    }

    Ok(HttpResponse::Ok().json(Response { dirs, files }))
}
