// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateJob`](crate::client::fluent_builders::CreateJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`details(RequestDetails)`](crate::client::fluent_builders::CreateJob::details) / [`set_details(Option<RequestDetails>)`](crate::client::fluent_builders::CreateJob::set_details): <p>The details for the CreateJob request.</p>
    ///   - [`r#type(Type)`](crate::client::fluent_builders::CreateJob::type) / [`set_type(Option<Type>)`](crate::client::fluent_builders::CreateJob::set_type): <p>The type of job to be created.</p>
                            /// - On success, responds with [`CreateJobOutput`](crate::output::CreateJobOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateJobOutput::arn): <p>The ARN for the job.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::CreateJobOutput::created_at): <p>The date and time that the job was created, in ISO 8601 format.</p>
    ///   - [`details(Option<ResponseDetails>)`](crate::output::CreateJobOutput::details): <p>Details about the job.</p>
    ///   - [`errors(Option<Vec<JobError>>)`](crate::output::CreateJobOutput::errors): <p>The errors associated with jobs.</p>
    ///   - [`id(Option<String>)`](crate::output::CreateJobOutput::id): <p>The unique identifier for the job.</p>
    ///   - [`state(Option<State>)`](crate::output::CreateJobOutput::state): <p>The state of the job.</p>
    ///   - [`r#type(Option<Type>)`](crate::output::CreateJobOutput::type): <p>The job type.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::output::CreateJobOutput::updated_at): <p>The date and time that the job was last updated, in ISO 8601 format.</p>
                            /// - On failure, responds with [`SdkError<CreateJobError>`](crate::error::CreateJobError)
    pub fn create_job(&self) -> crate::client::fluent_builders::CreateJob {
                                crate::client::fluent_builders::CreateJob::new(self.handle.clone())
                            }
}

