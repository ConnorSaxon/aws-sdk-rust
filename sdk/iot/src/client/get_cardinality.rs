// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCardinality`](crate::client::fluent_builders::GetCardinality) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_name(impl Into<String>)`](crate::client::fluent_builders::GetCardinality::index_name) / [`set_index_name(Option<String>)`](crate::client::fluent_builders::GetCardinality::set_index_name): <p>The name of the index to search.</p>
    ///   - [`query_string(impl Into<String>)`](crate::client::fluent_builders::GetCardinality::query_string) / [`set_query_string(Option<String>)`](crate::client::fluent_builders::GetCardinality::set_query_string): <p>The search query string.</p>
    ///   - [`aggregation_field(impl Into<String>)`](crate::client::fluent_builders::GetCardinality::aggregation_field) / [`set_aggregation_field(Option<String>)`](crate::client::fluent_builders::GetCardinality::set_aggregation_field): <p>The field to aggregate.</p>
    ///   - [`query_version(impl Into<String>)`](crate::client::fluent_builders::GetCardinality::query_version) / [`set_query_version(Option<String>)`](crate::client::fluent_builders::GetCardinality::set_query_version): <p>The query version.</p>
                            /// - On success, responds with [`GetCardinalityOutput`](crate::output::GetCardinalityOutput) with field(s):
    ///   - [`cardinality(i32)`](crate::output::GetCardinalityOutput::cardinality): <p>The approximate count of unique values that match the query.</p>
                            /// - On failure, responds with [`SdkError<GetCardinalityError>`](crate::error::GetCardinalityError)
    pub fn get_cardinality(&self) -> crate::client::fluent_builders::GetCardinality {
                                crate::client::fluent_builders::GetCardinality::new(self.handle.clone())
                            }
}

