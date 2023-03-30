// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTagsForResource::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): <p>The Amazon Resource Name (ARN) for the resource that you want to list tags for.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListTagsForResource::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListTagsForResource::set_max_results): <p>The maximum number of tags that you want to return in the response to a <code>ListTagsForResource</code> request. If you don't specify a value for <code>MaxResults</code>, Resolver returns up to 100 tags.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_next_token): <p>For the first <code>ListTagsForResource</code> request, omit this value.</p>  <p>If you have more than <code>MaxResults</code> tags, you can submit another <code>ListTagsForResource</code> request to get the next group of tags for the resource. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
                            /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::ListTagsForResourceOutput::tags): <p>The tags that are associated with the resource that you specified in the <code>ListTagsForResource</code> request.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTagsForResourceOutput::next_token): <p>If more than <code>MaxResults</code> tags match the specified criteria, you can submit another <code>ListTagsForResource</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response. </p>
                            /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                                crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                            }
}

