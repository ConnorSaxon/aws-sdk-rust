// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListProfileObjects`](crate::client::fluent_builders::ListProfileObjects) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListProfileObjects::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListProfileObjects::set_next_token): <p>The pagination token from the previous call to ListProfileObjects.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListProfileObjects::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListProfileObjects::set_max_results): <p>The maximum number of objects returned per page.</p>
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::ListProfileObjects::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::ListProfileObjects::set_domain_name): <p>The unique name of the domain.</p>
    ///   - [`object_type_name(impl Into<String>)`](crate::client::fluent_builders::ListProfileObjects::object_type_name) / [`set_object_type_name(Option<String>)`](crate::client::fluent_builders::ListProfileObjects::set_object_type_name): <p>The name of the profile object type.</p>
    ///   - [`profile_id(impl Into<String>)`](crate::client::fluent_builders::ListProfileObjects::profile_id) / [`set_profile_id(Option<String>)`](crate::client::fluent_builders::ListProfileObjects::set_profile_id): <p>The unique identifier of a customer profile.</p>
    ///   - [`object_filter(ObjectFilter)`](crate::client::fluent_builders::ListProfileObjects::object_filter) / [`set_object_filter(Option<ObjectFilter>)`](crate::client::fluent_builders::ListProfileObjects::set_object_filter): <p>Applies a filter to the response to include profile objects with the specified index values. This filter is only supported for ObjectTypeName _asset, _case and _order.</p>
                            /// - On success, responds with [`ListProfileObjectsOutput`](crate::output::ListProfileObjectsOutput) with field(s):
    ///   - [`items(Option<Vec<ListProfileObjectsItem>>)`](crate::output::ListProfileObjectsOutput::items): <p>The list of ListProfileObject instances.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListProfileObjectsOutput::next_token): <p>The pagination token from the previous call to ListProfileObjects.</p>
                            /// - On failure, responds with [`SdkError<ListProfileObjectsError>`](crate::error::ListProfileObjectsError)
    pub fn list_profile_objects(&self) -> crate::client::fluent_builders::ListProfileObjects {
                                crate::client::fluent_builders::ListProfileObjects::new(self.handle.clone())
                            }
}

