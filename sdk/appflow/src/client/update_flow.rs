// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateFlow`](crate::client::fluent_builders::UpdateFlow) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`flow_name(impl Into<String>)`](crate::client::fluent_builders::UpdateFlow::flow_name) / [`set_flow_name(Option<String>)`](crate::client::fluent_builders::UpdateFlow::set_flow_name): <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateFlow::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateFlow::set_description): <p> A description of the flow. </p>
    ///   - [`trigger_config(TriggerConfig)`](crate::client::fluent_builders::UpdateFlow::trigger_config) / [`set_trigger_config(Option<TriggerConfig>)`](crate::client::fluent_builders::UpdateFlow::set_trigger_config): <p> The trigger settings that determine how and when the flow runs. </p>
    ///   - [`source_flow_config(SourceFlowConfig)`](crate::client::fluent_builders::UpdateFlow::source_flow_config) / [`set_source_flow_config(Option<SourceFlowConfig>)`](crate::client::fluent_builders::UpdateFlow::set_source_flow_config): <p> Contains information about the configuration of the source connector used in the flow. </p>
    ///   - [`destination_flow_config_list(Vec<DestinationFlowConfig>)`](crate::client::fluent_builders::UpdateFlow::destination_flow_config_list) / [`set_destination_flow_config_list(Option<Vec<DestinationFlowConfig>>)`](crate::client::fluent_builders::UpdateFlow::set_destination_flow_config_list): <p> The configuration that controls how Amazon AppFlow transfers data to the destination connector. </p>
    ///   - [`tasks(Vec<Task>)`](crate::client::fluent_builders::UpdateFlow::tasks) / [`set_tasks(Option<Vec<Task>>)`](crate::client::fluent_builders::UpdateFlow::set_tasks): <p> A list of tasks that Amazon AppFlow performs while transferring the data in the flow run. </p>
    ///   - [`metadata_catalog_config(MetadataCatalogConfig)`](crate::client::fluent_builders::UpdateFlow::metadata_catalog_config) / [`set_metadata_catalog_config(Option<MetadataCatalogConfig>)`](crate::client::fluent_builders::UpdateFlow::set_metadata_catalog_config): <p>Specifies the configuration that Amazon AppFlow uses when it catalogs the data that's transferred by the associated flow. When Amazon AppFlow catalogs the data from a flow, it stores metadata in a data catalog.</p>
                            /// - On success, responds with [`UpdateFlowOutput`](crate::output::UpdateFlowOutput) with field(s):
    ///   - [`flow_status(Option<FlowStatus>)`](crate::output::UpdateFlowOutput::flow_status): <p>Indicates the current status of the flow. </p>
                            /// - On failure, responds with [`SdkError<UpdateFlowError>`](crate::error::UpdateFlowError)
    pub fn update_flow(&self) -> crate::client::fluent_builders::UpdateFlow {
                                crate::client::fluent_builders::UpdateFlow::new(self.handle.clone())
                            }
}

