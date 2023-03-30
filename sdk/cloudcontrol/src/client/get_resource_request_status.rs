// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetResourceRequestStatus`](crate::client::fluent_builders::GetResourceRequestStatus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`request_token(impl Into<String>)`](crate::client::fluent_builders::GetResourceRequestStatus::request_token) / [`set_request_token(Option<String>)`](crate::client::fluent_builders::GetResourceRequestStatus::set_request_token): <p>A unique token used to track the progress of the resource operation request.</p>  <p>Request tokens are included in the <code>ProgressEvent</code> type returned by a resource operation request.</p>
                            /// - On success, responds with [`GetResourceRequestStatusOutput`](crate::output::GetResourceRequestStatusOutput) with field(s):
    ///   - [`progress_event(Option<ProgressEvent>)`](crate::output::GetResourceRequestStatusOutput::progress_event): <p>Represents the current status of the resource operation request.</p>
                            /// - On failure, responds with [`SdkError<GetResourceRequestStatusError>`](crate::error::GetResourceRequestStatusError)
    pub fn get_resource_request_status(&self) -> crate::client::fluent_builders::GetResourceRequestStatus {
                                crate::client::fluent_builders::GetResourceRequestStatus::new(self.handle.clone())
                            }
}

