pub trait UsersRepository {
    async register(&self, register_user_entity: RegisterUserEntity) -> Result<i32>;
    async fn find_by_hospital_number(&self, hospital_number: i32) -> Result<UserEntity>
    async fn find_by_hospital_numbers(&self, hospital_numbers: Vec<i32>) -> Result<Vec<UserEntity>>
}