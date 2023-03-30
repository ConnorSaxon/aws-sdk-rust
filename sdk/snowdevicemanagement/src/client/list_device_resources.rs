// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDeviceResources`](crate::client::fluent_builders::ListDeviceResources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDeviceResources::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`managed_device_id(impl Into<String>)`](crate::client::fluent_builders::ListDeviceResources::managed_device_id) / [`set_managed_device_id(Option<String>)`](crate::client::fluent_builders::ListDeviceResources::set_managed_device_id): <p>The ID of the managed device that you are listing the resources of.</p>
    ///   - [`r#type(impl Into<String>)`](crate::client::fluent_builders::ListDeviceResources::type) / [`set_type(Option<String>)`](crate::client::fluent_builders::ListDeviceResources::set_type): <p>A structure used to filter the results by type of resource.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDeviceResources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDeviceResources::set_max_results): <p>The maximum number of resources per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDeviceResources::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDeviceResources::set_next_token): <p>A pagination token to continue to the next page of results.</p>
                            /// - On success, responds with [`ListDeviceResourcesOutput`](crate::output::ListDeviceResourcesOutput) with field(s):
    ///   - [`resources(Option<Vec<ResourceSummary>>)`](crate::output::ListDeviceResourcesOutput::resources): <p>A structure defining the resource's type, Amazon Resource Name (ARN), and ID.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDeviceResourcesOutput::next_token): <p>A pagination token to continue to the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListDeviceResourcesError>`](crate::error::ListDeviceResourcesError)
    pub fn list_device_resources(&self) -> crate::client::fluent_builders::ListDeviceResources {
                                crate::client::fluent_builders::ListDeviceResources::new(self.handle.clone())
                            }
}

