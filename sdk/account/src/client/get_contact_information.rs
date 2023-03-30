// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetContactInformation`](crate::client::fluent_builders::GetContactInformation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetContactInformation::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetContactInformation::set_account_id): <p>Specifies the 12-digit account ID number of the Amazon Web Services account that you want to access or modify with this operation. If you don't specify this parameter, it defaults to the Amazon Web Services account of the identity used to call the operation. To use this parameter, the caller must be an identity in the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">organization's management account</a> or a delegated administrator account. The specified account ID must also be a member account in the same organization. The organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features enabled</a>, and the organization must have <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-trusted-access.html">trusted access</a> enabled for the Account Management service, and optionally a <a href="https://docs.aws.amazon.com/organizations/latest/userguide/using-orgs-delegated-admin.html">delegated admin</a> account assigned.</p> <note>   <p>The management account can't specify its own <code>AccountId</code>. It must call the operation in standalone context by not including the <code>AccountId</code> parameter.</p>  </note>  <p>To call this operation on an account that is not a member of an organization, don't specify this parameter. Instead, call the operation using an identity belonging to the account whose contacts you wish to retrieve or modify.</p>
                            /// - On success, responds with [`GetContactInformationOutput`](crate::output::GetContactInformationOutput) with field(s):
    ///   - [`contact_information(Option<ContactInformation>)`](crate::output::GetContactInformationOutput::contact_information): <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
                            /// - On failure, responds with [`SdkError<GetContactInformationError>`](crate::error::GetContactInformationError)
    pub fn get_contact_information(&self) -> crate::client::fluent_builders::GetContactInformation {
                                crate::client::fluent_builders::GetContactInformation::new(self.handle.clone())
                            }
}

