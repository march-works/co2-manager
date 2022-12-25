use self::user::UserRepository;

pub mod user;

pub trait Repositories {
    type UserRepo: UserRepository;

    fn user_repository(&self) -> &Self::UserRepo;
}
