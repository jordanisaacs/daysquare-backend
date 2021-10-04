#[derive(Debug, Deserialize)]
pub struct Service {
    #[validate(min=1, max=50)]
    name: String,
    #[validate(url)]
    home_url: String,
    #[validate(url)]
    api_url: String,
    #[validate(min=1, max=150)]
    description: String
}
