// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRealtimeEndpoint`](crate::client::fluent_builders::CreateRealtimeEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`ml_model_id(impl Into<String>)`](crate::client::fluent_builders::CreateRealtimeEndpoint::ml_model_id) / [`set_ml_model_id(Option<String>)`](crate::client::fluent_builders::CreateRealtimeEndpoint::set_ml_model_id): <p>The ID assigned to the <code>MLModel</code> during creation.</p>
                            /// - On success, responds with [`CreateRealtimeEndpointOutput`](crate::output::CreateRealtimeEndpointOutput) with field(s):
    ///   - [`ml_model_id(Option<String>)`](crate::output::CreateRealtimeEndpointOutput::ml_model_id): <p>A user-supplied ID that uniquely identifies the <code>MLModel</code>. This value should be identical to the value of the <code>MLModelId</code> in the request.</p>
    ///   - [`realtime_endpoint_info(Option<RealtimeEndpointInfo>)`](crate::output::CreateRealtimeEndpointOutput::realtime_endpoint_info): <p>The endpoint information of the <code>MLModel</code> </p>
                            /// - On failure, responds with [`SdkError<CreateRealtimeEndpointError>`](crate::error::CreateRealtimeEndpointError)
    pub fn create_realtime_endpoint(&self) -> crate::client::fluent_builders::CreateRealtimeEndpoint {
                                crate::client::fluent_builders::CreateRealtimeEndpoint::new(self.handle.clone())
                            }
}

