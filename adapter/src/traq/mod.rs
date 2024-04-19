use derive_new::new;

pub mod message;
pub mod stamp;

#[derive(new)]
pub struct TraqRepositoryImpl {
    access_token: String,
}
