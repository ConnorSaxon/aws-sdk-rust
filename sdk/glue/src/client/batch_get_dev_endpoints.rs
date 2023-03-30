// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchGetDevEndpoints`](crate::client::fluent_builders::BatchGetDevEndpoints) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dev_endpoint_names(Vec<String>)`](crate::client::fluent_builders::BatchGetDevEndpoints::dev_endpoint_names) / [`set_dev_endpoint_names(Option<Vec<String>>)`](crate::client::fluent_builders::BatchGetDevEndpoints::set_dev_endpoint_names): <p>The list of <code>DevEndpoint</code> names, which might be the names returned from the <code>ListDevEndpoint</code> operation.</p>
                            /// - On success, responds with [`BatchGetDevEndpointsOutput`](crate::output::BatchGetDevEndpointsOutput) with field(s):
    ///   - [`dev_endpoints(Option<Vec<DevEndpoint>>)`](crate::output::BatchGetDevEndpointsOutput::dev_endpoints): <p>A list of <code>DevEndpoint</code> definitions.</p>
    ///   - [`dev_endpoints_not_found(Option<Vec<String>>)`](crate::output::BatchGetDevEndpointsOutput::dev_endpoints_not_found): <p>A list of <code>DevEndpoints</code> not found.</p>
                            /// - On failure, responds with [`SdkError<BatchGetDevEndpointsError>`](crate::error::BatchGetDevEndpointsError)
    pub fn batch_get_dev_endpoints(&self) -> crate::client::fluent_builders::BatchGetDevEndpoints {
                                crate::client::fluent_builders::BatchGetDevEndpoints::new(self.handle.clone())
                            }
}

