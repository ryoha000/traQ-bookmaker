use derive_new::new;

pub mod message;

#[derive(new)]
pub struct TraqRepositoryImpl {
    access_token: String,
}
