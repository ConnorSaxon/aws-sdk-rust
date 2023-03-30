// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateSigninDelegateGroupsWithAccount`](crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount::set_account_id): <p>The Amazon Chime account ID.</p>
    ///   - [`signin_delegate_groups(Vec<SigninDelegateGroup>)`](crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount::signin_delegate_groups) / [`set_signin_delegate_groups(Option<Vec<SigninDelegateGroup>>)`](crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount::set_signin_delegate_groups): <p>The sign-in delegate groups.</p>
                            /// - On success, responds with [`AssociateSigninDelegateGroupsWithAccountOutput`](crate::output::AssociateSigninDelegateGroupsWithAccountOutput)
                            /// - On failure, responds with [`SdkError<AssociateSigninDelegateGroupsWithAccountError>`](crate::error::AssociateSigninDelegateGroupsWithAccountError)
    pub fn associate_signin_delegate_groups_with_account(&self) -> crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount {
                                crate::client::fluent_builders::AssociateSigninDelegateGroupsWithAccount::new(self.handle.clone())
                            }
}

