// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListComponents`](crate::client::fluent_builders::ListComponents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListComponents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`owner(Ownership)`](crate::client::fluent_builders::ListComponents::owner) / [`set_owner(Option<Ownership>)`](crate::client::fluent_builders::ListComponents::set_owner): <p>Filters results based on the type of owner for the component. By default, this request returns a list of components that your account owns. To see results for other types of owners, you can specify components that Amazon manages, third party components, or components that other accounts have shared with you.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListComponents::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListComponents::set_filters): <p>Use the following filters to streamline results:</p>  <ul>   <li> <p> <code>description</code> </p> </li>   <li> <p> <code>name</code> </p> </li>   <li> <p> <code>platform</code> </p> </li>   <li> <p> <code>supportedOsVersion</code> </p> </li>   <li> <p> <code>type</code> </p> </li>   <li> <p> <code>version</code> </p> </li>  </ul>
    ///   - [`by_name(bool)`](crate::client::fluent_builders::ListComponents::by_name) / [`set_by_name(bool)`](crate::client::fluent_builders::ListComponents::set_by_name): <p>Returns the list of components for the specified name.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListComponents::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListComponents::set_max_results): <p>The maximum items to return in a request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListComponents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListComponents::set_next_token): <p>A token to specify where to start paginating. This is the NextToken from a previously truncated response.</p>
                            /// - On success, responds with [`ListComponentsOutput`](crate::output::ListComponentsOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::ListComponentsOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`component_version_list(Option<Vec<ComponentVersion>>)`](crate::output::ListComponentsOutput::component_version_list): <p>The list of component semantic versions.</p> <note>   <p>The semantic version has four nodes: <major>    .    <minor>     .     <patch>      /      <build>       . You can assign values for the first three, and can filter on all of them.      </build>     </patch>    </minor>   </major></p>  </note>
    ///   - [`next_token(Option<String>)`](crate::output::ListComponentsOutput::next_token): <p>The next token used for paginated responses. When this is not empty, there are additional elements that the service has not included in this request. Use this token with the next request to retrieve additional objects.</p>
                            /// - On failure, responds with [`SdkError<ListComponentsError>`](crate::error::ListComponentsError)
    pub fn list_components(&self) -> crate::client::fluent_builders::ListComponents {
                                crate::client::fluent_builders::ListComponents::new(self.handle.clone())
                            }
}

