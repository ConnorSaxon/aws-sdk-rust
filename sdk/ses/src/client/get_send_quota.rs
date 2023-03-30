// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetSendQuota`](crate::client::fluent_builders::GetSendQuota) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::GetSendQuota::send) it.
                            /// - On success, responds with [`GetSendQuotaOutput`](crate::output::GetSendQuotaOutput) with field(s):
    ///   - [`max24_hour_send(f64)`](crate::output::GetSendQuotaOutput::max24_hour_send): <p>The maximum number of emails the user is allowed to send in a 24-hour interval. A value of -1 signifies an unlimited quota.</p>
    ///   - [`max_send_rate(f64)`](crate::output::GetSendQuotaOutput::max_send_rate): <p>The maximum number of emails that Amazon SES can accept from the user's account per second.</p> <note>   <p>The rate at which Amazon SES accepts the user's messages might be less than the maximum send rate.</p>  </note>
    ///   - [`sent_last24_hours(f64)`](crate::output::GetSendQuotaOutput::sent_last24_hours): <p>The number of emails sent during the previous 24 hours.</p>
                            /// - On failure, responds with [`SdkError<GetSendQuotaError>`](crate::error::GetSendQuotaError)
    pub fn get_send_quota(&self) -> crate::client::fluent_builders::GetSendQuota {
                                crate::client::fluent_builders::GetSendQuota::new(self.handle.clone())
                            }
}

