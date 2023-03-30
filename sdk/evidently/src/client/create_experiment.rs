// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateExperiment`](crate::client::fluent_builders::CreateExperiment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project(impl Into<String>)`](crate::client::fluent_builders::CreateExperiment::project) / [`set_project(Option<String>)`](crate::client::fluent_builders::CreateExperiment::set_project): <p>The name or ARN of the project that you want to create the new experiment in.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateExperiment::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateExperiment::set_name): <p>A name for the new experiment.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateExperiment::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateExperiment::set_description): <p>An optional description of the experiment.</p>
    ///   - [`treatments(Vec<TreatmentConfig>)`](crate::client::fluent_builders::CreateExperiment::treatments) / [`set_treatments(Option<Vec<TreatmentConfig>>)`](crate::client::fluent_builders::CreateExperiment::set_treatments): <p>An array of structures that describe the configuration of each feature variation used in the experiment.</p>
    ///   - [`metric_goals(Vec<MetricGoalConfig>)`](crate::client::fluent_builders::CreateExperiment::metric_goals) / [`set_metric_goals(Option<Vec<MetricGoalConfig>>)`](crate::client::fluent_builders::CreateExperiment::set_metric_goals): <p>An array of structures that defines the metrics used for the experiment, and whether a higher or lower value for each metric is the goal.</p>
    ///   - [`randomization_salt(impl Into<String>)`](crate::client::fluent_builders::CreateExperiment::randomization_salt) / [`set_randomization_salt(Option<String>)`](crate::client::fluent_builders::CreateExperiment::set_randomization_salt): <p>When Evidently assigns a particular user session to an experiment, it must use a randomization ID to determine which variation the user session is served. This randomization ID is a combination of the entity ID and <code>randomizationSalt</code>. If you omit <code>randomizationSalt</code>, Evidently uses the experiment name as the <code>randomizationSalt</code>.</p>
    ///   - [`sampling_rate(i64)`](crate::client::fluent_builders::CreateExperiment::sampling_rate) / [`set_sampling_rate(Option<i64>)`](crate::client::fluent_builders::CreateExperiment::set_sampling_rate): <p>The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent. The available audience is the total audience minus the audience that you have allocated to overrides or current launches of this feature.</p>  <p>This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the available audience.</p>
    ///   - [`online_ab_config(OnlineAbConfig)`](crate::client::fluent_builders::CreateExperiment::online_ab_config) / [`set_online_ab_config(Option<OnlineAbConfig>)`](crate::client::fluent_builders::CreateExperiment::set_online_ab_config): <p>A structure that contains the configuration of which variation to use as the "control" version. tThe "control" version is used for comparison with other variations. This structure also specifies how much experiment traffic is allocated to each variation.</p>
    ///   - [`segment(impl Into<String>)`](crate::client::fluent_builders::CreateExperiment::segment) / [`set_segment(Option<String>)`](crate::client::fluent_builders::CreateExperiment::set_segment): <p>Specifies an audience <i>segment</i> to use in the experiment. When a segment is used in an experiment, only user sessions that match the segment pattern are used in the experiment.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateExperiment::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateExperiment::set_tags): <p>Assigns one or more tags (key-value pairs) to the experiment.</p>  <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>  <p>Tags don't have any semantic meaning to Amazon Web Services and are interpreted strictly as strings of characters.</p>  <p>You can associate as many as 50 tags with an experiment.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
                            /// - On success, responds with [`CreateExperimentOutput`](crate::output::CreateExperimentOutput) with field(s):
    ///   - [`experiment(Option<Experiment>)`](crate::output::CreateExperimentOutput::experiment): <p>A structure containing the configuration details of the experiment that you created.</p>
                            /// - On failure, responds with [`SdkError<CreateExperimentError>`](crate::error::CreateExperimentError)
    pub fn create_experiment(&self) -> crate::client::fluent_builders::CreateExperiment {
                                crate::client::fluent_builders::CreateExperiment::new(self.handle.clone())
                            }
}

