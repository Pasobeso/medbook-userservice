use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{
    config::config_loader::{get_doctors_secret_env, get_patients_secret_env},
    domain::{
        entities::users::UserEntity, repositories::users::UsersRepository,
        value_objects::roles::Roles,
    },
    infrastructure::{
        argon2_hashing,
        jwt_authentication::{
            self,
            authentication_model::LoginModel,
            jwt_model::{self, Claims, Passport},
        },
        postgres::schema::users,
    },
};

pub struct AuthenticationUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    users_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self {
        Self { users_repository }
    }

    pub async fn patients_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_patients_secret_env()?;
        let patient = self
            .users_repository
            .find_by_id(login_model.hospital_number)
            .await?;

        let patient_role_str = Roles::Patient.to_string();

        let is_patient = patient.role.iter().any(|r| r == &patient_role_str);

        if !is_patient {
            return Err(anyhow::anyhow!("User is not a patient"));
        }

        let original_password = patient.password;
        let login_password = login_model.password;

        if !argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        };

        let access_token_claims = Claims {
            sub: patient.id.to_string(),
            role: jwt_model::Roles::Patient,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: patient.id.to_string(),
            role: jwt_model::Roles::Patient,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn patients_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_patients_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: jwt_model::Roles::Patient,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: jwt_model::Roles::Patient,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn doctors_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_doctors_secret_env()?;

        let doctor = self
            .users_repository
            .find_by_id(login_model.hospital_number)
            .await?;

        let doctor_role_str = Roles::Doctor.to_string();
        let is_doctor = doctor.role.iter().any(|r| r == &doctor_role_str);

        if !is_doctor {
            return Err(anyhow::anyhow!("User is not a doctor"));
        }

        let original_password = doctor.password;
        let login_password = login_model.password;

        if !argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        };

        let access_token_claims = Claims {
            sub: doctor.id.to_string(),
            role: jwt_model::Roles::Doctor,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: doctor.id.to_string(),
            role: jwt_model::Roles::Doctor,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn doctors_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_doctors_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: jwt_model::Roles::Doctor,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: jwt_model::Roles::Doctor,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn get_me(&self, hospital_id: i32) -> Result<UserEntity> {
        Ok(self.users_repository.find_by_id(hospital_id).await?)
    }
}
