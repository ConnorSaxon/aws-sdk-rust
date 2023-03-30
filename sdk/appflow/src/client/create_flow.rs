// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFlow`](crate::client::fluent_builders::CreateFlow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`flow_name(impl Into<String>)`](crate::client::fluent_builders::CreateFlow::flow_name) / [`set_flow_name(Option<String>)`](crate::client::fluent_builders::CreateFlow::set_flow_name): <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateFlow::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateFlow::set_description): <p> A description of the flow you want to create. </p>
    ///   - [`kms_arn(impl Into<String>)`](crate::client::fluent_builders::CreateFlow::kms_arn) / [`set_kms_arn(Option<String>)`](crate::client::fluent_builders::CreateFlow::set_kms_arn): <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    ///   - [`trigger_config(TriggerConfig)`](crate::client::fluent_builders::CreateFlow::trigger_config) / [`set_trigger_config(Option<TriggerConfig>)`](crate::client::fluent_builders::CreateFlow::set_trigger_config): <p> The trigger settings that determine how and when the flow runs. </p>
    ///   - [`source_flow_config(SourceFlowConfig)`](crate::client::fluent_builders::CreateFlow::source_flow_config) / [`set_source_flow_config(Option<SourceFlowConfig>)`](crate::client::fluent_builders::CreateFlow::set_source_flow_config): <p> The configuration that controls how Amazon AppFlow retrieves data from the source connector. </p>
    ///   - [`destination_flow_config_list(Vec<DestinationFlowConfig>)`](crate::client::fluent_builders::CreateFlow::destination_flow_config_list) / [`set_destination_flow_config_list(Option<Vec<DestinationFlowConfig>>)`](crate::client::fluent_builders::CreateFlow::set_destination_flow_config_list): <p> The configuration that controls how Amazon AppFlow places data in the destination connector. </p>
    ///   - [`tasks(Vec<Task>)`](crate::client::fluent_builders::CreateFlow::tasks) / [`set_tasks(Option<Vec<Task>>)`](crate::client::fluent_builders::CreateFlow::set_tasks): <p> A list of tasks that Amazon AppFlow performs while transferring the data in the flow run. </p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateFlow::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateFlow::set_tags): <p> The tags used to organize, track, or control access for your flow. </p>
    ///   - [`metadata_catalog_config(MetadataCatalogConfig)`](crate::client::fluent_builders::CreateFlow::metadata_catalog_config) / [`set_metadata_catalog_config(Option<MetadataCatalogConfig>)`](crate::client::fluent_builders::CreateFlow::set_metadata_catalog_config): <p>Specifies the configuration that Amazon AppFlow uses when it catalogs the data that's transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.</p>
                            /// - On success, responds with [`CreateFlowOutput`](crate::output::CreateFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::output::CreateFlowOutput::flow_arn): <p> The flow's Amazon Resource Name (ARN). </p>
    ///   - [`flow_status(Option<FlowStatus>)`](crate::output::CreateFlowOutput::flow_status): <p> Indicates the current status of the flow. </p>
                            /// - On failure, responds with [`SdkError<CreateFlowError>`](crate::error::CreateFlowError)
    pub fn create_flow(&self) -> crate::client::fluent_builders::CreateFlow {
                                crate::client::fluent_builders::CreateFlow::new(self.handle.clone())
                            }
}

