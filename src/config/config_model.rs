#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub frontend: Frontend,
    pub database: Database,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
    pub path_prefix: String,
}

#[derive(Debug, Clone)]
pub struct Frontend {
    pub development_url: String,
    pub production_url: String,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct PatientsSecret {
    pub secret: String,
    pub refresh_secret: String,
}

#[derive(Debug, Clone)]
pub struct DoctorsSecret {
    pub secret: String,
    pub refresh_secret: String,
}
