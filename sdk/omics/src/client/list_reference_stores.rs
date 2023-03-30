// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListReferenceStores`](crate::client::fluent_builders::ListReferenceStores) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListReferenceStores::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListReferenceStores::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListReferenceStores::set_max_results): <p>The maximum number of stores to return in one page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListReferenceStores::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListReferenceStores::set_next_token): <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    ///   - [`filter(ReferenceStoreFilter)`](crate::client::fluent_builders::ListReferenceStores::filter) / [`set_filter(Option<ReferenceStoreFilter>)`](crate::client::fluent_builders::ListReferenceStores::set_filter): <p>A filter to apply to the list.</p>
                            /// - On success, responds with [`ListReferenceStoresOutput`](crate::output::ListReferenceStoresOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListReferenceStoresOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    ///   - [`reference_stores(Option<Vec<ReferenceStoreDetail>>)`](crate::output::ListReferenceStoresOutput::reference_stores): <p>A list of reference stores.</p>
                            /// - On failure, responds with [`SdkError<ListReferenceStoresError>`](crate::error::ListReferenceStoresError)
    pub fn list_reference_stores(&self) -> crate::client::fluent_builders::ListReferenceStores {
                                crate::client::fluent_builders::ListReferenceStores::new(self.handle.clone())
                            }
}

