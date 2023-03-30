// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResourceTags`](crate::client::fluent_builders::ListResourceTags) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListResourceTags::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::client::fluent_builders::ListResourceTags::key_id) / [`set_key_id(Option<String>)`](crate::client::fluent_builders::ListResourceTags::set_key_id): <p>Gets tags on the specified KMS key.</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListResourceTags::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListResourceTags::set_limit): <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>  <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListResourceTags::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListResourceTags::set_marker): <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>  <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p>
                            /// - On success, responds with [`ListResourceTagsOutput`](crate::output::ListResourceTagsOutput) with field(s):
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::ListResourceTagsOutput::tags): <p>A list of tags. Each tag consists of a tag key and a tag value.</p> <note>   <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>  </note>
    ///   - [`next_marker(Option<String>)`](crate::output::ListResourceTagsOutput::next_marker): <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>  <p>Do not assume or infer any information from this value.</p>
    ///   - [`truncated(bool)`](crate::output::ListResourceTagsOutput::truncated): <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
                            /// - On failure, responds with [`SdkError<ListResourceTagsError>`](crate::error::ListResourceTagsError)
    pub fn list_resource_tags(&self) -> crate::client::fluent_builders::ListResourceTags {
                                crate::client::fluent_builders::ListResourceTags::new(self.handle.clone())
                            }
}

