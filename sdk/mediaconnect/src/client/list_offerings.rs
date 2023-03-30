// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListOfferings`](crate::client::fluent_builders::ListOfferings) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListOfferings::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListOfferings::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListOfferings::set_max_results): The maximum number of results to return per API request. For example, you submit a ListOfferings request with MaxResults set at 5. Although 20 items match your request, the service returns no more than the first 5 items. (The service also returns a NextToken value that you can use to fetch the next batch of results.) The service might return fewer results than the MaxResults value. If MaxResults is not included in the request, the service defaults to pagination with a maximum of 10 results per page.
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListOfferings::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListOfferings::set_next_token): The token that identifies which batch of results that you want to see. For example, you submit a ListOfferings request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListOfferings request a second time and specify the NextToken value.
                            /// - On success, responds with [`ListOfferingsOutput`](crate::output::ListOfferingsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListOfferingsOutput::next_token): The token that identifies which batch of results that you want to see. For example, you submit a ListOfferings request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListOfferings request a second time and specify the NextToken value.
    ///   - [`offerings(Option<Vec<Offering>>)`](crate::output::ListOfferingsOutput::offerings): A list of offerings that are available to this account in the current AWS Region.
                            /// - On failure, responds with [`SdkError<ListOfferingsError>`](crate::error::ListOfferingsError)
    pub fn list_offerings(&self) -> crate::client::fluent_builders::ListOfferings {
                                crate::client::fluent_builders::ListOfferings::new(self.handle.clone())
                            }
}

