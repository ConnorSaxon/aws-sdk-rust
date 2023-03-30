// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateJob`](crate::client::fluent_builders::CreateJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateJob::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateJob::set_client_token): <p>A unique token that guarantees that the call to this API is idempotent.</p>
    ///   - [`algorithm_specification(AlgorithmSpecification)`](crate::client::fluent_builders::CreateJob::algorithm_specification) / [`set_algorithm_specification(Option<AlgorithmSpecification>)`](crate::client::fluent_builders::CreateJob::set_algorithm_specification): <p>Definition of the Amazon Braket job to be created. Specifies the container image the job uses and information about the Python scripts used for entry and training.</p>
    ///   - [`input_data_config(Vec<InputFileConfig>)`](crate::client::fluent_builders::CreateJob::input_data_config) / [`set_input_data_config(Option<Vec<InputFileConfig>>)`](crate::client::fluent_builders::CreateJob::set_input_data_config): <p>A list of parameters that specify the name and type of input data and where it is located.</p>
    ///   - [`output_data_config(JobOutputDataConfig)`](crate::client::fluent_builders::CreateJob::output_data_config) / [`set_output_data_config(Option<JobOutputDataConfig>)`](crate::client::fluent_builders::CreateJob::set_output_data_config): <p>The path to the S3 location where you want to store job artifacts and the encryption key used to store them.</p>
    ///   - [`checkpoint_config(JobCheckpointConfig)`](crate::client::fluent_builders::CreateJob::checkpoint_config) / [`set_checkpoint_config(Option<JobCheckpointConfig>)`](crate::client::fluent_builders::CreateJob::set_checkpoint_config): <p>Information about the output locations for job checkpoint data.</p>
    ///   - [`job_name(impl Into<String>)`](crate::client::fluent_builders::CreateJob::job_name) / [`set_job_name(Option<String>)`](crate::client::fluent_builders::CreateJob::set_job_name): <p>The name of the Amazon Braket job.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateJob::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreateJob::set_role_arn): <p>The Amazon Resource Name (ARN) of an IAM role that Amazon Braket can assume to perform tasks on behalf of a user. It can access user resources, run an Amazon Braket job container on behalf of user, and output resources to the users' s3 buckets.</p>
    ///   - [`stopping_condition(JobStoppingCondition)`](crate::client::fluent_builders::CreateJob::stopping_condition) / [`set_stopping_condition(Option<JobStoppingCondition>)`](crate::client::fluent_builders::CreateJob::set_stopping_condition): <p> The user-defined criteria that specifies when a job stops running.</p>
    ///   - [`instance_config(InstanceConfig)`](crate::client::fluent_builders::CreateJob::instance_config) / [`set_instance_config(Option<InstanceConfig>)`](crate::client::fluent_builders::CreateJob::set_instance_config): <p>Configuration of the resource instances to use while running the hybrid job on Amazon Braket.</p>
    ///   - [`hyper_parameters(HashMap<String, String>)`](crate::client::fluent_builders::CreateJob::hyper_parameters) / [`set_hyper_parameters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateJob::set_hyper_parameters): <p>Algorithm-specific parameters used by an Amazon Braket job that influence the quality of the training job. The values are set with a string of JSON key:value pairs, where the key is the name of the hyperparameter and the value is the value of th hyperparameter.</p>
    ///   - [`device_config(DeviceConfig)`](crate::client::fluent_builders::CreateJob::device_config) / [`set_device_config(Option<DeviceConfig>)`](crate::client::fluent_builders::CreateJob::set_device_config): <p>The quantum processing unit (QPU) or simulator used to create an Amazon Braket job.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateJob::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateJob::set_tags): <p>A tag object that consists of a key and an optional value, used to manage metadata for Amazon Braket resources.</p>
                            /// - On success, responds with [`CreateJobOutput`](crate::output::CreateJobOutput) with field(s):
    ///   - [`job_arn(Option<String>)`](crate::output::CreateJobOutput::job_arn): <p>The ARN of the Amazon Braket job created.</p>
                            /// - On failure, responds with [`SdkError<CreateJobError>`](crate::error::CreateJobError)
    pub fn create_job(&self) -> crate::client::fluent_builders::CreateJob {
                                crate::client::fluent_builders::CreateJob::new(self.handle.clone())
                            }
}

