use crate::api::api::{create_company_request, create_university_request};

#[tauri::command]
async fn create_university(
    login: String,
    mail: String,
    name: String,
) -> Result<String, reqwest::Error> {
    let body = create_university_request(login, mail, name).await?;
    Ok(body.password)
}
#[tauri::command]
async fn create_company(
    login: String,
    mail: String,
    name: String,
) -> Result<String, reqwest::Error> {
    let body = create_company_request(login, mail, name).await?;
    Ok(body.password)
}
