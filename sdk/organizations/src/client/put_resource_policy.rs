// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutResourcePolicy`](crate::client::fluent_builders::PutResourcePolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`content(impl Into<String>)`](crate::client::fluent_builders::PutResourcePolicy::content) / [`set_content(Option<String>)`](crate::client::fluent_builders::PutResourcePolicy::set_content): <p>If provided, the new content for the resource policy. The text must be correctly formatted JSON that complies with the syntax for the resource policy's type. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>Organizations User Guide.</i> </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::PutResourcePolicy::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::PutResourcePolicy::set_tags): <p>Updates the list of tags that you want to attach to the newly-created resource policy. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can't set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations resources</a> in the Organizations User Guide.</p> <note>   <p>Calls with tags apply to the initial creation of the resource policy, otherwise an exception is thrown. If any one of the tags is invalid or if you exceed the allowed number of tags for the resource policy, then the entire request fails and the resource policy is not created. </p>  </note>
                            /// - On success, responds with [`PutResourcePolicyOutput`](crate::output::PutResourcePolicyOutput) with field(s):
    ///   - [`resource_policy(Option<ResourcePolicy>)`](crate::output::PutResourcePolicyOutput::resource_policy): <p>A structure that contains details about the resource policy.</p>
                            /// - On failure, responds with [`SdkError<PutResourcePolicyError>`](crate::error::PutResourcePolicyError)
    pub fn put_resource_policy(&self) -> crate::client::fluent_builders::PutResourcePolicy {
                                crate::client::fluent_builders::PutResourcePolicy::new(self.handle.clone())
                            }
}

