// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPredictors`](crate::client::fluent_builders::ListPredictors) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPredictors::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPredictors::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPredictors::set_next_token): <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPredictors::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPredictors::set_max_results): <p>The number of items to return in the response.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListPredictors::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListPredictors::set_filters): <p>An array of filters. For each filter, you provide a condition and a match statement. The condition is either <code>IS</code> or <code>IS_NOT</code>, which specifies whether to include or exclude the predictors that match the statement from the list, respectively. The match statement consists of a key and a value.</p>  <p> <b>Filter properties</b> </p>  <ul>   <li> <p> <code>Condition</code> - The condition to apply. Valid values are <code>IS</code> and <code>IS_NOT</code>. To include the predictors that match the statement, specify <code>IS</code>. To exclude matching predictors, specify <code>IS_NOT</code>.</p> </li>   <li> <p> <code>Key</code> - The name of the parameter to filter on. Valid values are <code>DatasetGroupArn</code> and <code>Status</code>.</p> </li>   <li> <p> <code>Value</code> - The value to match.</p> </li>  </ul>  <p>For example, to list all predictors whose status is ACTIVE, you would specify:</p>  <p> <code>"Filters": [ { "Condition": "IS", "Key": "Status", "Value": "ACTIVE" } ]</code> </p>
                            /// - On success, responds with [`ListPredictorsOutput`](crate::output::ListPredictorsOutput) with field(s):
    ///   - [`predictors(Option<Vec<PredictorSummary>>)`](crate::output::ListPredictorsOutput::predictors): <p>An array of objects that summarize each predictor's properties.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPredictorsOutput::next_token): <p>If the response is truncated, Amazon Forecast returns this token. To retrieve the next set of results, use the token in the next request.</p>
                            /// - On failure, responds with [`SdkError<ListPredictorsError>`](crate::error::ListPredictorsError)
    pub fn list_predictors(&self) -> crate::client::fluent_builders::ListPredictors {
                                crate::client::fluent_builders::ListPredictors::new(self.handle.clone())
                            }
}

