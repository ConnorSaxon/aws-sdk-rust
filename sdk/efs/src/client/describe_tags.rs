// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeTags`](crate::client::fluent_builders::DescribeTags) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeTags::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_items(i32)`](crate::client::fluent_builders::DescribeTags::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::DescribeTags::set_max_items): <p>(Optional) The maximum number of file system tags to return in the response. Currently, this number is automatically set to 100, and other values are ignored. The response is paginated at 100 per page if you have more than 100 tags.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::DescribeTags::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::DescribeTags::set_marker): <p>(Optional) An opaque pagination token returned from a previous <code>DescribeTags</code> operation (String). If present, it specifies to continue the list from where the previous call left off.</p>
    ///   - [`file_system_id(impl Into<String>)`](crate::client::fluent_builders::DescribeTags::file_system_id) / [`set_file_system_id(Option<String>)`](crate::client::fluent_builders::DescribeTags::set_file_system_id): <p>The ID of the file system whose tag set you want to retrieve.</p>
                            /// - On success, responds with [`DescribeTagsOutput`](crate::output::DescribeTagsOutput) with field(s):
    ///   - [`marker(Option<String>)`](crate::output::DescribeTagsOutput::marker): <p>If the request included a <code>Marker</code>, the response returns that value in this field.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::DescribeTagsOutput::tags): <p>Returns tags associated with the file system as an array of <code>Tag</code> objects. </p>
    ///   - [`next_marker(Option<String>)`](crate::output::DescribeTagsOutput::next_marker): <p>If a value is present, there are more tags to return. In a subsequent request, you can provide the value of <code>NextMarker</code> as the value of the <code>Marker</code> parameter in your next request to retrieve the next set of tags.</p>
                            /// - On failure, responds with [`SdkError<DescribeTagsError>`](crate::error::DescribeTagsError)
    #[deprecated(note = "Use ListTagsForResource.")]
    pub fn describe_tags(&self) -> crate::client::fluent_builders::DescribeTags {
                                crate::client::fluent_builders::DescribeTags::new(self.handle.clone())
                            }
}

