use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::helpers::format_response;
use crate::v1::resources::fine_tuning::CreateFineTuningJobParameters;
use crate::v1::resources::fine_tuning::FineTuningJob;
use crate::v1::resources::fine_tuning::ListFineTuningJobEventsResponse;
use crate::v1::resources::fine_tuning::ListFineTuningJobsResponse;
use crate::v1::resources::shared::SimpleListParameters;
use serde_json::Value;

pub struct FineTuning<'a> {
    pub client: &'a Client,
}

impl Client {
    /// Manage fine-tuning jobs to tailor a model to your specific training data.
    pub fn fine_tuning(&self) -> FineTuning {
        FineTuning { client: self }
    }
}

impl FineTuning<'_> {
    /// Creates a job that fine-tunes a specified model from a given dataset.
    pub async fn create(
        &self,
        parameters: CreateFineTuningJobParameters,
    ) -> Result<FineTuningJob, APIError> {
        let response = self.client.post("/fine_tuning/jobs", &parameters).await?;

        let fine_tuning_job_response: FineTuningJob = format_response(response)?;

        Ok(fine_tuning_job_response)
    }

    /// List your organization's fine-tuning jobs.
    pub async fn list(
        &self,
        query: Option<SimpleListParameters>,
    ) -> Result<ListFineTuningJobsResponse, APIError> {
        let response = self
            .client
            .get_with_query("/fine_tuning/jobs", &query)
            .await?;

        let list_fine_tuning_jobs_response: ListFineTuningJobsResponse = format_response(response)?;

        Ok(list_fine_tuning_jobs_response)
    }

    /// Get info about a fine-tuning job.
    pub async fn retrieve(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let response = self
            .client
            .get(format!("/fine_tuning/jobs/{id}").as_str())
            .await?;

        let fine_tuning_job_response: FineTuningJob = format_response(response)?;

        Ok(fine_tuning_job_response)
    }

    /// Immediately cancel a fine-tune job.
    pub async fn cancel(&self, id: &str) -> Result<FineTuningJob, APIError> {
        let response = self
            .client
            .post(
                format!("/fine_tuning/jobs/{id}/cancel").as_str(),
                &Value::Null,
            )
            .await?;

        let fine_tuning_job_response: FineTuningJob = format_response(response)?;

        Ok(fine_tuning_job_response)
    }

    /// Get status updates for a fine-tuning job.
    pub async fn list_job_events(
        &self,
        id: &str,
        query: Option<SimpleListParameters>,
    ) -> Result<ListFineTuningJobEventsResponse, APIError> {
        let response = self
            .client
            .get_with_query(format!("/fine_tuning/jobs/{id}/events").as_str(), &query)
            .await?;

        let list_fine_tuning_job_events_response: ListFineTuningJobEventsResponse =
            format_response(response)?;

        Ok(list_fine_tuning_job_events_response)
    }
}
