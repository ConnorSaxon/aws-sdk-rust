// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssumeRoleWithSAML`](crate::client::fluent_builders::AssumeRoleWithSAML) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_role_arn): <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    ///   - [`principal_arn(impl Into<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::principal_arn) / [`set_principal_arn(Option<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_principal_arn): <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    ///   - [`saml_assertion(impl Into<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::saml_assertion) / [`set_saml_assertion(Option<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_saml_assertion): <p>The base64 encoded SAML authentication response provided by the IdP.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    ///   - [`policy_arns(Vec<PolicyDescriptorType>)`](crate::client::fluent_builders::AssumeRoleWithSAML::policy_arns) / [`set_policy_arns(Option<Vec<PolicyDescriptorType>>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_policy_arns): <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>  <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>   <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>  </note>  <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    ///   - [`policy(impl Into<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::policy) / [`set_policy(Option<String>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_policy): <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>  <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>  <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>   <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>  </note>
    ///   - [`duration_seconds(i32)`](crate::client::fluent_builders::AssumeRoleWithSAML::duration_seconds) / [`set_duration_seconds(Option<i32>)`](crate::client::fluent_builders::AssumeRoleWithSAML::set_duration_seconds): <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>  <p>By default, the value is set to <code>3600</code> seconds. </p> <note>   <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>  </note>
                            /// - On success, responds with [`AssumeRoleWithSamlOutput`](crate::output::AssumeRoleWithSamlOutput) with field(s):
    ///   - [`credentials(Option<Credentials>)`](crate::output::AssumeRoleWithSamlOutput::credentials): <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>   <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>  </note>
    ///   - [`assumed_role_user(Option<AssumedRoleUser>)`](crate::output::AssumeRoleWithSamlOutput::assumed_role_user): <p>The identifiers for the temporary security credentials that the operation returns.</p>
    ///   - [`packed_policy_size(Option<i32>)`](crate::output::AssumeRoleWithSamlOutput::packed_policy_size): <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    ///   - [`subject(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::subject): <p>The value of the <code>NameID</code> element in the <code>Subject</code> element of the SAML assertion.</p>
    ///   - [`subject_type(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::subject_type): <p> The format of the name ID, as defined by the <code>Format</code> attribute in the <code>NameID</code> element of the SAML assertion. Typical examples of the format are <code>transient</code> or <code>persistent</code>. </p>  <p> If the format includes the prefix <code>urn:oasis:names:tc:SAML:2.0:nameid-format</code>, that prefix is removed. For example, <code>urn:oasis:names:tc:SAML:2.0:nameid-format:transient</code> is returned as <code>transient</code>. If the format includes any other prefix, the format is returned with no modifications.</p>
    ///   - [`issuer(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::issuer): <p>The value of the <code>Issuer</code> element of the SAML assertion.</p>
    ///   - [`audience(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::audience): <p> The value of the <code>Recipient</code> attribute of the <code>SubjectConfirmationData</code> element of the SAML assertion. </p>
    ///   - [`name_qualifier(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::name_qualifier): <p>A hash value based on the concatenation of the following:</p>  <ul>   <li> <p>The <code>Issuer</code> response value.</p> </li>   <li> <p>The Amazon Web Services account ID.</p> </li>   <li> <p>The friendly name (the last part of the ARN) of the SAML provider in IAM.</p> </li>  </ul>  <p>The combination of <code>NameQualifier</code> and <code>Subject</code> can be used to uniquely identify a federated user.</p>  <p>The following pseudocode shows how the hash value is calculated:</p>  <p> <code>BASE64 ( SHA1 ( "https://example.com/saml" + "123456789012" + "/MySAMLIdP" ) )</code> </p>
    ///   - [`source_identity(Option<String>)`](crate::output::AssumeRoleWithSamlOutput::source_identity): <p>The value in the <code>SourceIdentity</code> attribute in the SAML assertion. </p>  <p>You can require users to set a source identity value when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. That way, actions that are taken with the role are associated with that user. After the source identity is set, the value cannot be changed. It is present in the request for all actions that are taken by the role and persists across <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_terms-and-concepts#iam-term-role-chaining">chained role</a> sessions. You can configure your SAML identity provider to use an attribute associated with your users, like user name or email, as the source identity when calling <code>AssumeRoleWithSAML</code>. You do this by adding an attribute to the SAML assertion. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>  <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
                            /// - On failure, responds with [`SdkError<AssumeRoleWithSAMLError>`](crate::error::AssumeRoleWithSAMLError)
    pub fn assume_role_with_saml(&self) -> crate::client::fluent_builders::AssumeRoleWithSAML {
                                crate::client::fluent_builders::AssumeRoleWithSAML::new(self.handle.clone())
                            }
}

