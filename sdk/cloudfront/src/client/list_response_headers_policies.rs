// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResponseHeadersPolicies`](crate::client::fluent_builders::ListResponseHeadersPolicies) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`r#type(ResponseHeadersPolicyType)`](crate::client::fluent_builders::ListResponseHeadersPolicies::type) / [`set_type(Option<ResponseHeadersPolicyType>)`](crate::client::fluent_builders::ListResponseHeadersPolicies::set_type): <p>A filter to get only the specified kind of response headers policies. Valid values are:</p>  <ul>   <li> <p> <code>managed</code> – Gets only the managed policies created by Amazon Web Services.</p> </li>   <li> <p> <code>custom</code> – Gets only the custom policies created in your Amazon Web Services account.</p> </li>  </ul>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListResponseHeadersPolicies::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListResponseHeadersPolicies::set_marker): <p>Use this field when paginating results to indicate where to begin in your list of response headers policies. The response includes response headers policies in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListResponseHeadersPolicies::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListResponseHeadersPolicies::set_max_items): <p>The maximum number of response headers policies that you want to get in the response.</p>
                            /// - On success, responds with [`ListResponseHeadersPoliciesOutput`](crate::output::ListResponseHeadersPoliciesOutput) with field(s):
    ///   - [`response_headers_policy_list(Option<ResponseHeadersPolicyList>)`](crate::output::ListResponseHeadersPoliciesOutput::response_headers_policy_list): <p>A list of response headers policies.</p>
                            /// - On failure, responds with [`SdkError<ListResponseHeadersPoliciesError>`](crate::error::ListResponseHeadersPoliciesError)
    pub fn list_response_headers_policies(&self) -> crate::client::fluent_builders::ListResponseHeadersPolicies {
                                crate::client::fluent_builders::ListResponseHeadersPolicies::new(self.handle.clone())
                            }
}

