// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRecipeJob`](crate::client::fluent_builders::CreateRecipeJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dataset_name(impl Into<String>)`](crate::client::fluent_builders::CreateRecipeJob::dataset_name) / [`set_dataset_name(Option<String>)`](crate::client::fluent_builders::CreateRecipeJob::set_dataset_name): <p>The name of the dataset that this job processes.</p>
    ///   - [`encryption_key_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRecipeJob::encryption_key_arn) / [`set_encryption_key_arn(Option<String>)`](crate::client::fluent_builders::CreateRecipeJob::set_encryption_key_arn): <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    ///   - [`encryption_mode(EncryptionMode)`](crate::client::fluent_builders::CreateRecipeJob::encryption_mode) / [`set_encryption_mode(Option<EncryptionMode>)`](crate::client::fluent_builders::CreateRecipeJob::set_encryption_mode): <p>The encryption mode for the job, which can be one of the following:</p>  <ul>   <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>   <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>  </ul>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateRecipeJob::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateRecipeJob::set_name): <p>A unique name for the job. Valid characters are alphanumeric (A-Z, a-z, 0-9), hyphen (-), period (.), and space.</p>
    ///   - [`log_subscription(LogSubscription)`](crate::client::fluent_builders::CreateRecipeJob::log_subscription) / [`set_log_subscription(Option<LogSubscription>)`](crate::client::fluent_builders::CreateRecipeJob::set_log_subscription): <p>Enables or disables Amazon CloudWatch logging for the job. If logging is enabled, CloudWatch writes one log stream for each job run.</p>
    ///   - [`max_capacity(i32)`](crate::client::fluent_builders::CreateRecipeJob::max_capacity) / [`set_max_capacity(i32)`](crate::client::fluent_builders::CreateRecipeJob::set_max_capacity): <p>The maximum number of nodes that DataBrew can consume when the job processes data.</p>
    ///   - [`max_retries(i32)`](crate::client::fluent_builders::CreateRecipeJob::max_retries) / [`set_max_retries(i32)`](crate::client::fluent_builders::CreateRecipeJob::set_max_retries): <p>The maximum number of times to retry the job after a job run fails.</p>
    ///   - [`outputs(Vec<Output>)`](crate::client::fluent_builders::CreateRecipeJob::outputs) / [`set_outputs(Option<Vec<Output>>)`](crate::client::fluent_builders::CreateRecipeJob::set_outputs): <p>One or more artifacts that represent the output from running the job.</p>
    ///   - [`data_catalog_outputs(Vec<DataCatalogOutput>)`](crate::client::fluent_builders::CreateRecipeJob::data_catalog_outputs) / [`set_data_catalog_outputs(Option<Vec<DataCatalogOutput>>)`](crate::client::fluent_builders::CreateRecipeJob::set_data_catalog_outputs): <p>One or more artifacts that represent the Glue Data Catalog output from running the job.</p>
    ///   - [`database_outputs(Vec<DatabaseOutput>)`](crate::client::fluent_builders::CreateRecipeJob::database_outputs) / [`set_database_outputs(Option<Vec<DatabaseOutput>>)`](crate::client::fluent_builders::CreateRecipeJob::set_database_outputs): <p>Represents a list of JDBC database output objects which defines the output destination for a DataBrew recipe job to write to. </p>
    ///   - [`project_name(impl Into<String>)`](crate::client::fluent_builders::CreateRecipeJob::project_name) / [`set_project_name(Option<String>)`](crate::client::fluent_builders::CreateRecipeJob::set_project_name): <p>Either the name of an existing project, or a combination of a recipe and a dataset to associate with the recipe.</p>
    ///   - [`recipe_reference(RecipeReference)`](crate::client::fluent_builders::CreateRecipeJob::recipe_reference) / [`set_recipe_reference(Option<RecipeReference>)`](crate::client::fluent_builders::CreateRecipeJob::set_recipe_reference): <p>Represents the name and version of a DataBrew recipe.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRecipeJob::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateRecipeJob::set_role_arn): <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateRecipeJob::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateRecipeJob::set_tags): <p>Metadata tags to apply to this job.</p>
    ///   - [`timeout(i32)`](crate::client::fluent_builders::CreateRecipeJob::timeout) / [`set_timeout(i32)`](crate::client::fluent_builders::CreateRecipeJob::set_timeout): <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
                            /// - On success, responds with [`CreateRecipeJobOutput`](crate::output::CreateRecipeJobOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateRecipeJobOutput::name): <p>The name of the job that you created.</p>
                            /// - On failure, responds with [`SdkError<CreateRecipeJobError>`](crate::error::CreateRecipeJobError)
    pub fn create_recipe_job(&self) -> crate::client::fluent_builders::CreateRecipeJob {
                                crate::client::fluent_builders::CreateRecipeJob::new(self.handle.clone())
                            }
}

