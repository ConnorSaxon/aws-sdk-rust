// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendBonus`](crate::client::fluent_builders::SendBonus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`worker_id(impl Into<String>)`](crate::client::fluent_builders::SendBonus::worker_id) / [`set_worker_id(Option<String>)`](crate::client::fluent_builders::SendBonus::set_worker_id): <p>The ID of the Worker being paid the bonus.</p>
    ///   - [`bonus_amount(impl Into<String>)`](crate::client::fluent_builders::SendBonus::bonus_amount) / [`set_bonus_amount(Option<String>)`](crate::client::fluent_builders::SendBonus::set_bonus_amount): <p> The Bonus amount is a US Dollar amount specified using a string (for example, "5" represents $5.00 USD and "101.42" represents $101.42 USD). Do not include currency symbols or currency codes. </p>
    ///   - [`assignment_id(impl Into<String>)`](crate::client::fluent_builders::SendBonus::assignment_id) / [`set_assignment_id(Option<String>)`](crate::client::fluent_builders::SendBonus::set_assignment_id): <p>The ID of the assignment for which this bonus is paid.</p>
    ///   - [`reason(impl Into<String>)`](crate::client::fluent_builders::SendBonus::reason) / [`set_reason(Option<String>)`](crate::client::fluent_builders::SendBonus::set_reason): <p>A message that explains the reason for the bonus payment. The Worker receiving the bonus can see this message.</p>
    ///   - [`unique_request_token(impl Into<String>)`](crate::client::fluent_builders::SendBonus::unique_request_token) / [`set_unique_request_token(Option<String>)`](crate::client::fluent_builders::SendBonus::set_unique_request_token): <p>A unique identifier for this request, which allows you to retry the call on error without granting multiple bonuses. This is useful in cases such as network timeouts where it is unclear whether or not the call succeeded on the server. If the bonus already exists in the system from a previous call using the same UniqueRequestToken, subsequent calls will return an error with a message containing the request ID.</p>
                            /// - On success, responds with [`SendBonusOutput`](crate::output::SendBonusOutput)
                            /// - On failure, responds with [`SdkError<SendBonusError>`](crate::error::SendBonusError)
    pub fn send_bonus(&self) -> crate::client::fluent_builders::SendBonus {
                                crate::client::fluent_builders::SendBonus::new(self.handle.clone())
                            }
}

