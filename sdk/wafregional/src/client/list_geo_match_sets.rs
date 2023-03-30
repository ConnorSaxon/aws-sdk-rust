// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListGeoMatchSets`](crate::client::fluent_builders::ListGeoMatchSets) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_marker(impl Into<String>)`](crate::client::fluent_builders::ListGeoMatchSets::next_marker) / [`set_next_marker(Option<String>)`](crate::client::fluent_builders::ListGeoMatchSets::set_next_marker): <p>If you specify a value for <code>Limit</code> and you have more <code>GeoMatchSet</code>s than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>GeoMatchSet</code> objects. For the second and subsequent <code>ListGeoMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>GeoMatchSet</code> objects.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListGeoMatchSets::limit) / [`set_limit(i32)`](crate::client::fluent_builders::ListGeoMatchSets::set_limit): <p>Specifies the number of <code>GeoMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>GeoMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>GeoMatchSet</code> objects.</p>
                            /// - On success, responds with [`ListGeoMatchSetsOutput`](crate::output::ListGeoMatchSetsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListGeoMatchSetsOutput::next_marker): <p>If you have more <code>GeoMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>GeoMatchSet</code> objects, submit another <code>ListGeoMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    ///   - [`geo_match_sets(Option<Vec<GeoMatchSetSummary>>)`](crate::output::ListGeoMatchSetsOutput::geo_match_sets): <p>An array of <code>GeoMatchSetSummary</code> objects.</p>
                            /// - On failure, responds with [`SdkError<ListGeoMatchSetsError>`](crate::error::ListGeoMatchSetsError)
    pub fn list_geo_match_sets(&self) -> crate::client::fluent_builders::ListGeoMatchSets {
                                crate::client::fluent_builders::ListGeoMatchSets::new(self.handle.clone())
                            }
}

