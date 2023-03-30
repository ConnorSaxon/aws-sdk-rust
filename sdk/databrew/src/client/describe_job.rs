// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeJob`](crate::client::fluent_builders::DescribeJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DescribeJob::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DescribeJob::set_name): <p>The name of the job to be described.</p>
                            /// - On success, responds with [`DescribeJobOutput`](crate::output::DescribeJobOutput) with field(s):
    ///   - [`create_date(Option<DateTime>)`](crate::output::DescribeJobOutput::create_date): <p>The date and time that the job was created.</p>
    ///   - [`created_by(Option<String>)`](crate::output::DescribeJobOutput::created_by): <p>The identifier (user name) of the user associated with the creation of the job.</p>
    ///   - [`dataset_name(Option<String>)`](crate::output::DescribeJobOutput::dataset_name): <p>The dataset that the job acts upon.</p>
    ///   - [`encryption_key_arn(Option<String>)`](crate::output::DescribeJobOutput::encryption_key_arn): <p>The Amazon Resource Name (ARN) of an encryption key that is used to protect the job.</p>
    ///   - [`encryption_mode(Option<EncryptionMode>)`](crate::output::DescribeJobOutput::encryption_mode): <p>The encryption mode for the job, which can be one of the following:</p>  <ul>   <li> <p> <code>SSE-KMS</code> - Server-side encryption with keys managed by KMS.</p> </li>   <li> <p> <code>SSE-S3</code> - Server-side encryption with keys managed by Amazon S3.</p> </li>  </ul>
    ///   - [`name(Option<String>)`](crate::output::DescribeJobOutput::name): <p>The name of the job.</p>
    ///   - [`r#type(Option<JobType>)`](crate::output::DescribeJobOutput::type): <p>The job type, which must be one of the following:</p>  <ul>   <li> <p> <code>PROFILE</code> - The job analyzes the dataset to determine its size, data types, data distribution, and more.</p> </li>   <li> <p> <code>RECIPE</code> - The job applies one or more transformations to a dataset.</p> </li>  </ul>
    ///   - [`last_modified_by(Option<String>)`](crate::output::DescribeJobOutput::last_modified_by): <p>The identifier (user name) of the user who last modified the job.</p>
    ///   - [`last_modified_date(Option<DateTime>)`](crate::output::DescribeJobOutput::last_modified_date): <p>The date and time that the job was last modified.</p>
    ///   - [`log_subscription(Option<LogSubscription>)`](crate::output::DescribeJobOutput::log_subscription): <p>Indicates whether Amazon CloudWatch logging is enabled for this job.</p>
    ///   - [`max_capacity(i32)`](crate::output::DescribeJobOutput::max_capacity): <p>The maximum number of compute nodes that DataBrew can consume when the job processes data.</p>
    ///   - [`max_retries(i32)`](crate::output::DescribeJobOutput::max_retries): <p>The maximum number of times to retry the job after a job run fails.</p>
    ///   - [`outputs(Option<Vec<Output>>)`](crate::output::DescribeJobOutput::outputs): <p>One or more artifacts that represent the output from running the job.</p>
    ///   - [`data_catalog_outputs(Option<Vec<DataCatalogOutput>>)`](crate::output::DescribeJobOutput::data_catalog_outputs): <p>One or more artifacts that represent the Glue Data Catalog output from running the job.</p>
    ///   - [`database_outputs(Option<Vec<DatabaseOutput>>)`](crate::output::DescribeJobOutput::database_outputs): <p>Represents a list of JDBC database output objects which defines the output destination for a DataBrew recipe job to write into.</p>
    ///   - [`project_name(Option<String>)`](crate::output::DescribeJobOutput::project_name): <p>The DataBrew project associated with this job.</p>
    ///   - [`profile_configuration(Option<ProfileConfiguration>)`](crate::output::DescribeJobOutput::profile_configuration): <p>Configuration for profile jobs. Used to select columns, do evaluations, and override default parameters of evaluations. When configuration is null, the profile job will run with default settings.</p>
    ///   - [`validation_configurations(Option<Vec<ValidationConfiguration>>)`](crate::output::DescribeJobOutput::validation_configurations): <p>List of validation configurations that are applied to the profile job.</p>
    ///   - [`recipe_reference(Option<RecipeReference>)`](crate::output::DescribeJobOutput::recipe_reference): <p>Represents the name and version of a DataBrew recipe.</p>
    ///   - [`resource_arn(Option<String>)`](crate::output::DescribeJobOutput::resource_arn): <p>The Amazon Resource Name (ARN) of the job.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribeJobOutput::role_arn): <p>The ARN of the Identity and Access Management (IAM) role to be assumed when DataBrew runs the job.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribeJobOutput::tags): <p>Metadata tags associated with this job.</p>
    ///   - [`timeout(i32)`](crate::output::DescribeJobOutput::timeout): <p>The job's timeout in minutes. A job that attempts to run longer than this timeout period ends with a status of <code>TIMEOUT</code>.</p>
    ///   - [`job_sample(Option<JobSample>)`](crate::output::DescribeJobOutput::job_sample): <p>Sample configuration for profile jobs only. Determines the number of rows on which the profile job will be executed.</p>
                            /// - On failure, responds with [`SdkError<DescribeJobError>`](crate::error::DescribeJobError)
    pub fn describe_job(&self) -> crate::client::fluent_builders::DescribeJob {
                                crate::client::fluent_builders::DescribeJob::new(self.handle.clone())
                            }
}

