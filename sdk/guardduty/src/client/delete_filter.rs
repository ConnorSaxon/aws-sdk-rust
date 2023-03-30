// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFilter`](crate::client::fluent_builders::DeleteFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::DeleteFilter::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::DeleteFilter::set_detector_id): <p>The unique ID of the detector that the filter is associated with.</p>
    ///   - [`filter_name(impl Into<String>)`](crate::client::fluent_builders::DeleteFilter::filter_name) / [`set_filter_name(Option<String>)`](crate::client::fluent_builders::DeleteFilter::set_filter_name): <p>The name of the filter that you want to delete.</p>
                            /// - On success, responds with [`DeleteFilterOutput`](crate::output::DeleteFilterOutput)
                            /// - On failure, responds with [`SdkError<DeleteFilterError>`](crate::error::DeleteFilterError)
    pub fn delete_filter(&self) -> crate::client::fluent_builders::DeleteFilter {
                                crate::client::fluent_builders::DeleteFilter::new(self.handle.clone())
                            }
}

