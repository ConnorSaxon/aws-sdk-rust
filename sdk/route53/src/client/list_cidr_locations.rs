// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCidrLocations`](crate::client::fluent_builders::ListCidrLocations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCidrLocations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`collection_id(impl Into<String>)`](crate::client::fluent_builders::ListCidrLocations::collection_id) / [`set_collection_id(Option<String>)`](crate::client::fluent_builders::ListCidrLocations::set_collection_id): <p>The CIDR collection ID.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCidrLocations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCidrLocations::set_next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>  <p>If no value is provided, the listing of results starts from the beginning.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCidrLocations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCidrLocations::set_max_results): <p>The maximum number of CIDR collection locations to return in the response.</p>
                            /// - On success, responds with [`ListCidrLocationsOutput`](crate::output::ListCidrLocationsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListCidrLocationsOutput::next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>  <p>If no value is provided, the listing of results starts from the beginning.</p>
    ///   - [`cidr_locations(Option<Vec<LocationSummary>>)`](crate::output::ListCidrLocationsOutput::cidr_locations): <p>A complex type that contains information about the list of CIDR locations.</p>
                            /// - On failure, responds with [`SdkError<ListCidrLocationsError>`](crate::error::ListCidrLocationsError)
    pub fn list_cidr_locations(&self) -> crate::client::fluent_builders::ListCidrLocations {
                                crate::client::fluent_builders::ListCidrLocations::new(self.handle.clone())
                            }
}

