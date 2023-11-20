use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::validate_request;
use crate::v1::resources::moderation::{ModerationParameters, ModerationResponse};

pub struct Moderations<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Given a input text, outputs if the model classifies it as violating OpenAI's content policy.
    pub fn moderations(&self) -> Moderations {
        Moderations { client: self }
    }
}

impl Moderations<'_> {
    /// Classifies if text violates OpenAI's Content Policy
    pub async fn create(
        &self,
        parameters: ModerationParameters,
    ) -> Result<ModerationResponse, APIError> {
        let response = self.client.post("/moderations", &parameters).await?;

        let value = validate_request(response).await?;

        let moderation_response: ModerationResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(moderation_response)
    }
}
