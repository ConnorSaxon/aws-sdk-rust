// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`InitiateVaultLock`](crate::client::fluent_builders::InitiateVaultLock) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::InitiateVaultLock::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::InitiateVaultLock::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID. This value must match the AWS account ID associated with the credentials used to sign the request. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon Glacier uses the AWS account ID associated with the credentials used to sign the request. If you specify your account ID, do not include any hyphens ('-') in the ID.</p>
    ///   - [`vault_name(impl Into<String>)`](crate::client::fluent_builders::InitiateVaultLock::vault_name) / [`set_vault_name(Option<String>)`](crate::client::fluent_builders::InitiateVaultLock::set_vault_name): <p>The name of the vault.</p>
    ///   - [`policy(VaultLockPolicy)`](crate::client::fluent_builders::InitiateVaultLock::policy) / [`set_policy(Option<VaultLockPolicy>)`](crate::client::fluent_builders::InitiateVaultLock::set_policy): <p>The vault lock policy as a JSON string, which uses "\" as an escape character.</p>
                            /// - On success, responds with [`InitiateVaultLockOutput`](crate::output::InitiateVaultLockOutput) with field(s):
    ///   - [`lock_id(Option<String>)`](crate::output::InitiateVaultLockOutput::lock_id): <p>The lock ID, which is used to complete the vault locking process.</p>
                            /// - On failure, responds with [`SdkError<InitiateVaultLockError>`](crate::error::InitiateVaultLockError)
    pub fn initiate_vault_lock(&self) -> crate::client::fluent_builders::InitiateVaultLock {
                                crate::client::fluent_builders::InitiateVaultLock::new(self.handle.clone())
                            }
}

