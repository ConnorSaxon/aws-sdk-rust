// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPartnerAccount`](crate::client::fluent_builders::GetPartnerAccount) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`partner_account_id(impl Into<String>)`](crate::client::fluent_builders::GetPartnerAccount::partner_account_id) / [`set_partner_account_id(Option<String>)`](crate::client::fluent_builders::GetPartnerAccount::set_partner_account_id): <p>The partner account ID to disassociate from the AWS account.</p>
    ///   - [`partner_type(PartnerType)`](crate::client::fluent_builders::GetPartnerAccount::partner_type) / [`set_partner_type(Option<PartnerType>)`](crate::client::fluent_builders::GetPartnerAccount::set_partner_type): <p>The partner type.</p>
                            /// - On success, responds with [`GetPartnerAccountOutput`](crate::output::GetPartnerAccountOutput) with field(s):
    ///   - [`sidewalk(Option<SidewalkAccountInfoWithFingerprint>)`](crate::output::GetPartnerAccountOutput::sidewalk): <p>The Sidewalk account credentials.</p>
    ///   - [`account_linked(bool)`](crate::output::GetPartnerAccountOutput::account_linked): <p>Whether the partner account is linked to the AWS account.</p>
                            /// - On failure, responds with [`SdkError<GetPartnerAccountError>`](crate::error::GetPartnerAccountError)
    pub fn get_partner_account(&self) -> crate::client::fluent_builders::GetPartnerAccount {
                                crate::client::fluent_builders::GetPartnerAccount::new(self.handle.clone())
                            }
}

