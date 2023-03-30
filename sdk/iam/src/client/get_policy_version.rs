// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPolicyVersion`](crate::client::fluent_builders::GetPolicyVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`policy_arn(impl Into<String>)`](crate::client::fluent_builders::GetPolicyVersion::policy_arn) / [`set_policy_arn(Option<String>)`](crate::client::fluent_builders::GetPolicyVersion::set_policy_arn): <p>The Amazon Resource Name (ARN) of the managed policy that you want information about.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`version_id(impl Into<String>)`](crate::client::fluent_builders::GetPolicyVersion::version_id) / [`set_version_id(Option<String>)`](crate::client::fluent_builders::GetPolicyVersion::set_version_id): <p>Identifies the policy version to retrieve.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that consists of the lowercase letter 'v' followed by one or two digits, and optionally followed by a period '.' and a string of letters and digits.</p>
                            /// - On success, responds with [`GetPolicyVersionOutput`](crate::output::GetPolicyVersionOutput) with field(s):
    ///   - [`policy_version(Option<PolicyVersion>)`](crate::output::GetPolicyVersionOutput::policy_version): <p>A structure containing details about the policy version.</p>
                            /// - On failure, responds with [`SdkError<GetPolicyVersionError>`](crate::error::GetPolicyVersionError)
    pub fn get_policy_version(&self) -> crate::client::fluent_builders::GetPolicyVersion {
                                crate::client::fluent_builders::GetPolicyVersion::new(self.handle.clone())
                            }
}

