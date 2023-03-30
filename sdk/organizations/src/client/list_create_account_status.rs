// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCreateAccountStatus`](crate::client::fluent_builders::ListCreateAccountStatus) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCreateAccountStatus::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`states(Vec<CreateAccountState>)`](crate::client::fluent_builders::ListCreateAccountStatus::states) / [`set_states(Option<Vec<CreateAccountState>>)`](crate::client::fluent_builders::ListCreateAccountStatus::set_states): <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCreateAccountStatus::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCreateAccountStatus::set_next_token): <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCreateAccountStatus::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCreateAccountStatus::set_max_results): <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
                            /// - On success, responds with [`ListCreateAccountStatusOutput`](crate::output::ListCreateAccountStatusOutput) with field(s):
    ///   - [`create_account_statuses(Option<Vec<CreateAccountStatus>>)`](crate::output::ListCreateAccountStatusOutput::create_account_statuses): <p>A list of objects with details about the requests. Certain elements, such as the accountId number, are present in the output only after the account has been successfully created.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListCreateAccountStatusOutput::next_token): <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
                            /// - On failure, responds with [`SdkError<ListCreateAccountStatusError>`](crate::error::ListCreateAccountStatusError)
    pub fn list_create_account_status(&self) -> crate::client::fluent_builders::ListCreateAccountStatus {
                                crate::client::fluent_builders::ListCreateAccountStatus::new(self.handle.clone())
                            }
}

