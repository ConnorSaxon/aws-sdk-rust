// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSuppressedDestinations`](crate::client::fluent_builders::ListSuppressedDestinations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSuppressedDestinations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`reasons(Vec<SuppressionListReason>)`](crate::client::fluent_builders::ListSuppressedDestinations::reasons) / [`set_reasons(Option<Vec<SuppressionListReason>>)`](crate::client::fluent_builders::ListSuppressedDestinations::set_reasons): <p>The factors that caused the email address to be added to .</p>
    ///   - [`start_date(DateTime)`](crate::client::fluent_builders::ListSuppressedDestinations::start_date) / [`set_start_date(Option<DateTime>)`](crate::client::fluent_builders::ListSuppressedDestinations::set_start_date): <p>Used to filter the list of suppressed email destinations so that it only includes addresses that were added to the list after a specific date.</p>
    ///   - [`end_date(DateTime)`](crate::client::fluent_builders::ListSuppressedDestinations::end_date) / [`set_end_date(Option<DateTime>)`](crate::client::fluent_builders::ListSuppressedDestinations::set_end_date): <p>Used to filter the list of suppressed email destinations so that it only includes addresses that were added to the list before a specific date.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSuppressedDestinations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSuppressedDestinations::set_next_token): <p>A token returned from a previous call to <code>ListSuppressedDestinations</code> to indicate the position in the list of suppressed email addresses.</p>
    ///   - [`page_size(i32)`](crate::client::fluent_builders::ListSuppressedDestinations::page_size) / [`set_page_size(Option<i32>)`](crate::client::fluent_builders::ListSuppressedDestinations::set_page_size): <p>The number of results to show in a single call to <code>ListSuppressedDestinations</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
                            /// - On success, responds with [`ListSuppressedDestinationsOutput`](crate::output::ListSuppressedDestinationsOutput) with field(s):
    ///   - [`suppressed_destination_summaries(Option<Vec<SuppressedDestinationSummary>>)`](crate::output::ListSuppressedDestinationsOutput::suppressed_destination_summaries): <p>A list of summaries, each containing a summary for a suppressed email destination.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSuppressedDestinationsOutput::next_token): <p>A token that indicates that there are additional email addresses on the suppression list for your account. To view additional suppressed addresses, issue another request to <code>ListSuppressedDestinations</code>, and pass this token in the <code>NextToken</code> parameter.</p>
                            /// - On failure, responds with [`SdkError<ListSuppressedDestinationsError>`](crate::error::ListSuppressedDestinationsError)
    pub fn list_suppressed_destinations(&self) -> crate::client::fluent_builders::ListSuppressedDestinations {
                                crate::client::fluent_builders::ListSuppressedDestinations::new(self.handle.clone())
                            }
}

