// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListObjectPolicies`](crate::client::fluent_builders::ListObjectPolicies) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListObjectPolicies::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl Into<String>)`](crate::client::fluent_builders::ListObjectPolicies::directory_arn) / [`set_directory_arn(Option<String>)`](crate::client::fluent_builders::ListObjectPolicies::set_directory_arn): <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where objects reside. For more information, see <code>arns</code>.</p>
    ///   - [`object_reference(ObjectReference)`](crate::client::fluent_builders::ListObjectPolicies::object_reference) / [`set_object_reference(Option<ObjectReference>)`](crate::client::fluent_builders::ListObjectPolicies::set_object_reference): <p>Reference that identifies the object for which policies will be listed.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListObjectPolicies::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListObjectPolicies::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListObjectPolicies::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListObjectPolicies::set_max_results): <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    ///   - [`consistency_level(ConsistencyLevel)`](crate::client::fluent_builders::ListObjectPolicies::consistency_level) / [`set_consistency_level(Option<ConsistencyLevel>)`](crate::client::fluent_builders::ListObjectPolicies::set_consistency_level): <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
                            /// - On success, responds with [`ListObjectPoliciesOutput`](crate::output::ListObjectPoliciesOutput) with field(s):
    ///   - [`attached_policy_ids(Option<Vec<String>>)`](crate::output::ListObjectPoliciesOutput::attached_policy_ids): <p>A list of policy <code>ObjectIdentifiers</code>, that are attached to the object.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListObjectPoliciesOutput::next_token): <p>The pagination token.</p>
                            /// - On failure, responds with [`SdkError<ListObjectPoliciesError>`](crate::error::ListObjectPoliciesError)
    pub fn list_object_policies(&self) -> crate::client::fluent_builders::ListObjectPolicies {
                                crate::client::fluent_builders::ListObjectPolicies::new(self.handle.clone())
                            }
}

