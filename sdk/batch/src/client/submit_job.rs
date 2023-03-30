// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SubmitJob`](crate::client::fluent_builders::SubmitJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_name(impl Into<String>)`](crate::client::fluent_builders::SubmitJob::job_name) / [`set_job_name(Option<String>)`](crate::client::fluent_builders::SubmitJob::set_job_name): <p>The name of the job. It can be up to 128 letters long. The first character must be alphanumeric, can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    ///   - [`job_queue(impl Into<String>)`](crate::client::fluent_builders::SubmitJob::job_queue) / [`set_job_queue(Option<String>)`](crate::client::fluent_builders::SubmitJob::set_job_queue): <p>The job queue where the job is submitted. You can specify either the name or the Amazon Resource Name (ARN) of the queue.</p>
    ///   - [`share_identifier(impl Into<String>)`](crate::client::fluent_builders::SubmitJob::share_identifier) / [`set_share_identifier(Option<String>)`](crate::client::fluent_builders::SubmitJob::set_share_identifier): <p>The share identifier for the job. If the job queue doesn't have a scheduling policy, then this parameter must not be specified. If the job queue has a scheduling policy, then this parameter must be specified.</p>
    ///   - [`scheduling_priority_override(i32)`](crate::client::fluent_builders::SubmitJob::scheduling_priority_override) / [`set_scheduling_priority_override(Option<i32>)`](crate::client::fluent_builders::SubmitJob::set_scheduling_priority_override): <p>The scheduling priority for the job. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority. This overrides any scheduling priority in the job definition.</p>  <p>The minimum supported value is 0 and the maximum supported value is 9999.</p>
    ///   - [`array_properties(ArrayProperties)`](crate::client::fluent_builders::SubmitJob::array_properties) / [`set_array_properties(Option<ArrayProperties>)`](crate::client::fluent_builders::SubmitJob::set_array_properties): <p>The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000. If you specify array properties for a job, it becomes an array job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/array_jobs.html">Array Jobs</a> in the <i>Batch User Guide</i>.</p>
    ///   - [`depends_on(Vec<JobDependency>)`](crate::client::fluent_builders::SubmitJob::depends_on) / [`set_depends_on(Option<Vec<JobDependency>>)`](crate::client::fluent_builders::SubmitJob::set_depends_on): <p>A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a <code>SEQUENTIAL</code> type dependency without specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an <code>N_TO_N</code> type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each dependency to complete before it can begin.</p>
    ///   - [`job_definition(impl Into<String>)`](crate::client::fluent_builders::SubmitJob::job_definition) / [`set_job_definition(Option<String>)`](crate::client::fluent_builders::SubmitJob::set_job_definition): <p>The job definition used by this job. This value can be one of <code>name</code>, <code>name:revision</code>, or the Amazon Resource Name (ARN) for the job definition. If <code>name</code> is specified without a revision then the latest active revision is used.</p>
    ///   - [`parameters(HashMap<String, String>)`](crate::client::fluent_builders::SubmitJob::parameters) / [`set_parameters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SubmitJob::set_parameters): <p>Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    ///   - [`container_overrides(ContainerOverrides)`](crate::client::fluent_builders::SubmitJob::container_overrides) / [`set_container_overrides(Option<ContainerOverrides>)`](crate::client::fluent_builders::SubmitJob::set_container_overrides): <p>An object with various properties that override the defaults for the job definition that specify the name of a container in the specified job definition and the overrides it should receive. You can override the default command for a container, which is specified in the job definition or the Docker image, with a <code>command</code> override. You can also override existing environment variables on a container or add new environment variables to it with an <code>environment</code> override.</p>
    ///   - [`node_overrides(NodeOverrides)`](crate::client::fluent_builders::SubmitJob::node_overrides) / [`set_node_overrides(Option<NodeOverrides>)`](crate::client::fluent_builders::SubmitJob::set_node_overrides): <p>A list of node overrides in JSON format that specify the node range to target and the container overrides for that node range.</p> <note>   <p>This parameter isn't applicable to jobs that are running on Fargate resources; use <code>containerOverrides</code> instead.</p>  </note>
    ///   - [`retry_strategy(RetryStrategy)`](crate::client::fluent_builders::SubmitJob::retry_strategy) / [`set_retry_strategy(Option<RetryStrategy>)`](crate::client::fluent_builders::SubmitJob::set_retry_strategy): <p>The retry strategy to use for failed jobs from this <code>SubmitJob</code> operation. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.</p>
    ///   - [`propagate_tags(bool)`](crate::client::fluent_builders::SubmitJob::propagate_tags) / [`set_propagate_tags(Option<bool>)`](crate::client::fluent_builders::SubmitJob::set_propagate_tags): <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags aren't propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state. When specified, this overrides the tag propagation setting in the job definition.</p>
    ///   - [`timeout(JobTimeout)`](crate::client::fluent_builders::SubmitJob::timeout) / [`set_timeout(Option<JobTimeout>)`](crate::client::fluent_builders::SubmitJob::set_timeout): <p>The timeout configuration for this <code>SubmitJob</code> operation. You can specify a timeout duration after which Batch terminates your jobs if they haven't finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. This configuration overrides any timeout configuration specified in the job definition. For array jobs, child jobs have the same timeout configuration as the parent job. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/job_timeouts.html">Job Timeouts</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::SubmitJob::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::SubmitJob::set_tags): <p>The tags that you apply to the job request to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> in <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`eks_properties_override(EksPropertiesOverride)`](crate::client::fluent_builders::SubmitJob::eks_properties_override) / [`set_eks_properties_override(Option<EksPropertiesOverride>)`](crate::client::fluent_builders::SubmitJob::set_eks_properties_override): <p>An object that can only be specified for jobs that are run on Amazon EKS resources with various properties that override defaults for the job definition.</p>
                            /// - On success, responds with [`SubmitJobOutput`](crate::output::SubmitJobOutput) with field(s):
    ///   - [`job_arn(Option<String>)`](crate::output::SubmitJobOutput::job_arn): <p>The Amazon Resource Name (ARN) for the job.</p>
    ///   - [`job_name(Option<String>)`](crate::output::SubmitJobOutput::job_name): <p>The name of the job.</p>
    ///   - [`job_id(Option<String>)`](crate::output::SubmitJobOutput::job_id): <p>The unique identifier for the job.</p>
                            /// - On failure, responds with [`SdkError<SubmitJobError>`](crate::error::SubmitJobError)
    pub fn submit_job(&self) -> crate::client::fluent_builders::SubmitJob {
                                crate::client::fluent_builders::SubmitJob::new(self.handle.clone())
                            }
}

