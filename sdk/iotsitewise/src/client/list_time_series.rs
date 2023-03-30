// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTimeSeries`](crate::client::fluent_builders::ListTimeSeries) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTimeSeries::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTimeSeries::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTimeSeries::set_next_token): <p>The token to be used for the next set of paginated results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListTimeSeries::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListTimeSeries::set_max_results): <p>The maximum number of results to return for each paginated request.</p>
    ///   - [`asset_id(impl Into<String>)`](crate::client::fluent_builders::ListTimeSeries::asset_id) / [`set_asset_id(Option<String>)`](crate::client::fluent_builders::ListTimeSeries::set_asset_id): <p>The ID of the asset in which the asset property was created.</p>
    ///   - [`alias_prefix(impl Into<String>)`](crate::client::fluent_builders::ListTimeSeries::alias_prefix) / [`set_alias_prefix(Option<String>)`](crate::client::fluent_builders::ListTimeSeries::set_alias_prefix): <p>The alias prefix of the time series.</p>
    ///   - [`time_series_type(ListTimeSeriesType)`](crate::client::fluent_builders::ListTimeSeries::time_series_type) / [`set_time_series_type(Option<ListTimeSeriesType>)`](crate::client::fluent_builders::ListTimeSeries::set_time_series_type): <p>The type of the time series. The time series type can be one of the following values:</p>  <ul>   <li> <p> <code>ASSOCIATED</code> – The time series is associated with an asset property.</p> </li>   <li> <p> <code>DISASSOCIATED</code> – The time series isn't associated with any asset property.</p> </li>  </ul>
                            /// - On success, responds with [`ListTimeSeriesOutput`](crate::output::ListTimeSeriesOutput) with field(s):
    ///   - [`time_series_summaries(Option<Vec<TimeSeriesSummary>>)`](crate::output::ListTimeSeriesOutput::time_series_summaries): <p>One or more time series summaries to list.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTimeSeriesOutput::next_token): <p>The token for the next set of results, or null if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListTimeSeriesError>`](crate::error::ListTimeSeriesError)
    pub fn list_time_series(&self) -> crate::client::fluent_builders::ListTimeSeries {
                                crate::client::fluent_builders::ListTimeSeries::new(self.handle.clone())
                            }
}

