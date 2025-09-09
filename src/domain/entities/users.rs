pub struct User {
    pub hospital_number: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDatetime,
    pub updated_at: NaiveDatetime,
    pub deleted_at: Option<NaiveDatetime>
}

pub struct RegisterUserEntity {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub role: String,
}

pub struct EditUserEntity {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub password_hash: String,
    pub role: String,
}