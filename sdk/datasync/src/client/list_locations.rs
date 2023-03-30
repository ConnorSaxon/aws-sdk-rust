// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListLocations`](crate::client::fluent_builders::ListLocations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListLocations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListLocations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListLocations::set_max_results): <p>The maximum number of locations to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListLocations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListLocations::set_next_token): <p>An opaque string that indicates the position at which to begin the next list of locations.</p>
    ///   - [`filters(Vec<LocationFilter>)`](crate::client::fluent_builders::ListLocations::filters) / [`set_filters(Option<Vec<LocationFilter>>)`](crate::client::fluent_builders::ListLocations::set_filters): <p>You can use API filters to narrow down the list of resources returned by <code>ListLocations</code>. For example, to retrieve all tasks on a specific source location, you can use <code>ListLocations</code> with filter name <code>LocationType S3</code> and <code>Operator Equals</code>.</p>
                            /// - On success, responds with [`ListLocationsOutput`](crate::output::ListLocationsOutput) with field(s):
    ///   - [`locations(Option<Vec<LocationListEntry>>)`](crate::output::ListLocationsOutput::locations): <p>An array that contains a list of locations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListLocationsOutput::next_token): <p>An opaque string that indicates the position at which to begin returning the next list of locations.</p>
                            /// - On failure, responds with [`SdkError<ListLocationsError>`](crate::error::ListLocationsError)
    pub fn list_locations(&self) -> crate::client::fluent_builders::ListLocations {
                                crate::client::fluent_builders::ListLocations::new(self.handle.clone())
                            }
}

