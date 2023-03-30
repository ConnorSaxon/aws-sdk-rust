// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePhoneNumber`](crate::client::fluent_builders::DescribePhoneNumber) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`phone_number_id(impl Into<String>)`](crate::client::fluent_builders::DescribePhoneNumber::phone_number_id) / [`set_phone_number_id(Option<String>)`](crate::client::fluent_builders::DescribePhoneNumber::set_phone_number_id): <p>A unique identifier for the phone number.</p>
                            /// - On success, responds with [`DescribePhoneNumberOutput`](crate::output::DescribePhoneNumberOutput) with field(s):
    ///   - [`claimed_phone_number_summary(Option<ClaimedPhoneNumberSummary>)`](crate::output::DescribePhoneNumberOutput::claimed_phone_number_summary): <p>Information about a phone number that's been claimed to your Amazon Connect instance or traffic distribution group.</p>
                            /// - On failure, responds with [`SdkError<DescribePhoneNumberError>`](crate::error::DescribePhoneNumberError)
    pub fn describe_phone_number(&self) -> crate::client::fluent_builders::DescribePhoneNumber {
                                crate::client::fluent_builders::DescribePhoneNumber::new(self.handle.clone())
                            }
}

