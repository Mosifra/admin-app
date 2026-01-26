use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUniversityResponse {
    pub success: bool,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompanyResponse {
    pub success: bool,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUniversitiesResponse {
    pub universities: Vec<University>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchCompaniesResponse {
    pub companies: Vec<Company>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    pub id: String,
    pub login: String,
    pub mail: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub login: String,
    pub mail: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUniversityResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCompanyResponse {
    pub success: bool,
}
