// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateWorkload`](crate::client::fluent_builders::CreateWorkload) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`workload_name(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::workload_name) / [`set_workload_name(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_workload_name): <p>The name of the workload.</p>  <p>The name must be unique within an account within an Amazon Web Services Region. Spaces and capitalization are ignored when checking for uniqueness.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_description): <p>The description for the workload.</p>
    ///   - [`environment(WorkloadEnvironment)`](crate::client::fluent_builders::CreateWorkload::environment) / [`set_environment(Option<WorkloadEnvironment>)`](crate::client::fluent_builders::CreateWorkload::set_environment): <p>The environment for the workload.</p>
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_account_ids): <p>The list of Amazon Web Services account IDs associated with the workload.</p>
    ///   - [`aws_regions(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::aws_regions) / [`set_aws_regions(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_aws_regions): <p>The list of Amazon Web Services Regions associated with the workload, for example, <code>us-east-2</code>, or <code>ca-central-1</code>.</p>
    ///   - [`non_aws_regions(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::non_aws_regions) / [`set_non_aws_regions(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_non_aws_regions): <p> The list of non-Amazon Web Services Regions associated with the workload.</p>
    ///   - [`pillar_priorities(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::pillar_priorities) / [`set_pillar_priorities(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_pillar_priorities): <p>The priorities of the pillars, which are used to order items in the improvement plan. Each pillar is represented by its <code>PillarReviewSummary$PillarId</code>.</p>
    ///   - [`architectural_design(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::architectural_design) / [`set_architectural_design(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_architectural_design): <p>The URL of the architectural design for the workload.</p>
    ///   - [`review_owner(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::review_owner) / [`set_review_owner(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_review_owner): <p>The review owner of the workload. The name, email address, or identifier for the primary group or individual that owns the workload review process.</p>
    ///   - [`industry_type(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::industry_type) / [`set_industry_type(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_industry_type): <p>The industry type for the workload.</p>  <p>If specified, must be one of the following:</p>  <ul>   <li> <p> <code>Agriculture</code> </p> </li>   <li> <p> <code>Automobile</code> </p> </li>   <li> <p> <code>Defense</code> </p> </li>   <li> <p> <code>Design and Engineering</code> </p> </li>   <li> <p> <code>Digital Advertising</code> </p> </li>   <li> <p> <code>Education</code> </p> </li>   <li> <p> <code>Environmental Protection</code> </p> </li>   <li> <p> <code>Financial Services</code> </p> </li>   <li> <p> <code>Gaming</code> </p> </li>   <li> <p> <code>General Public Services</code> </p> </li>   <li> <p> <code>Healthcare</code> </p> </li>   <li> <p> <code>Hospitality</code> </p> </li>   <li> <p> <code>InfoTech</code> </p> </li>   <li> <p> <code>Justice and Public Safety</code> </p> </li>   <li> <p> <code>Life Sciences</code> </p> </li>   <li> <p> <code>Manufacturing</code> </p> </li>   <li> <p> <code>Media &amp; Entertainment</code> </p> </li>   <li> <p> <code>Mining &amp; Resources</code> </p> </li>   <li> <p> <code>Oil &amp; Gas</code> </p> </li>   <li> <p> <code>Power &amp; Utilities</code> </p> </li>   <li> <p> <code>Professional Services</code> </p> </li>   <li> <p> <code>Real Estate &amp; Construction</code> </p> </li>   <li> <p> <code>Retail &amp; Wholesale</code> </p> </li>   <li> <p> <code>Social Protection</code> </p> </li>   <li> <p> <code>Telecommunications</code> </p> </li>   <li> <p> <code>Travel, Transportation &amp; Logistics</code> </p> </li>   <li> <p> <code>Other</code> </p> </li>  </ul>
    ///   - [`industry(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::industry) / [`set_industry(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_industry): <p>The industry for the workload.</p>
    ///   - [`lenses(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::lenses) / [`set_lenses(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_lenses): <p>The list of lenses associated with the workload. Each lens is identified by its <code>LensSummary$LensAlias</code>.</p>
    ///   - [`notes(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::notes) / [`set_notes(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_notes): <p>The notes associated with the workload.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::CreateWorkload::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::CreateWorkload::set_client_request_token): <p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p>  <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after it has completed successfully, the result of the original request is returned. </p> <important>   <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p>  </important>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateWorkload::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateWorkload::set_tags): <p>The tags to be associated with the workload.</p>
    ///   - [`discovery_config(WorkloadDiscoveryConfig)`](crate::client::fluent_builders::CreateWorkload::discovery_config) / [`set_discovery_config(Option<WorkloadDiscoveryConfig>)`](crate::client::fluent_builders::CreateWorkload::set_discovery_config): <p>Well-Architected discovery configuration settings associated to the workload.</p>
    ///   - [`applications(Vec<String>)`](crate::client::fluent_builders::CreateWorkload::applications) / [`set_applications(Option<Vec<String>>)`](crate::client::fluent_builders::CreateWorkload::set_applications): <p>List of AppRegistry application ARNs associated to the workload.</p>
                            /// - On success, responds with [`CreateWorkloadOutput`](crate::output::CreateWorkloadOutput) with field(s):
    ///   - [`workload_id(Option<String>)`](crate::output::CreateWorkloadOutput::workload_id): <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    ///   - [`workload_arn(Option<String>)`](crate::output::CreateWorkloadOutput::workload_arn): <p>The ARN for the workload.</p>
                            /// - On failure, responds with [`SdkError<CreateWorkloadError>`](crate::error::CreateWorkloadError)
    pub fn create_workload(&self) -> crate::client::fluent_builders::CreateWorkload {
                                crate::client::fluent_builders::CreateWorkload::new(self.handle.clone())
                            }
}

