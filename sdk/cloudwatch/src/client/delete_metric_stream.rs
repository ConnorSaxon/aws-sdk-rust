// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteMetricStream`](crate::client::fluent_builders::DeleteMetricStream) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeleteMetricStream::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeleteMetricStream::set_name): <p>The name of the metric stream to delete.</p>
                            /// - On success, responds with [`DeleteMetricStreamOutput`](crate::output::DeleteMetricStreamOutput)
                            /// - On failure, responds with [`SdkError<DeleteMetricStreamError>`](crate::error::DeleteMetricStreamError)
    pub fn delete_metric_stream(&self) -> crate::client::fluent_builders::DeleteMetricStream {
                                crate::client::fluent_builders::DeleteMetricStream::new(self.handle.clone())
                            }
}

