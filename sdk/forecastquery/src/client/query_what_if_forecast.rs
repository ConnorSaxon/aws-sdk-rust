// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`QueryWhatIfForecast`](crate::client::fluent_builders::QueryWhatIfForecast) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`what_if_forecast_arn(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::what_if_forecast_arn) / [`set_what_if_forecast_arn(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_what_if_forecast_arn): <p>The Amazon Resource Name (ARN) of the what-if forecast to query.</p>
    ///   - [`start_date(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::start_date) / [`set_start_date(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_start_date): <p>The start date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    ///   - [`end_date(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::end_date) / [`set_end_date(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_end_date): <p>The end date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    ///   - [`filters(HashMap<String, String>)`](crate::client::fluent_builders::QueryWhatIfForecast::filters) / [`set_filters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_filters): <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>  <p> <code>{"item_id" : "client_21"}</code> </p>  <p>To get the full what-if forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateWhatIfForecastExport.html">CreateForecastExportJob</a> operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_next_token): <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
                            /// - On success, responds with [`QueryWhatIfForecastOutput`](crate::output::QueryWhatIfForecastOutput) with field(s):
    ///   - [`forecast(Option<Forecast>)`](crate::output::QueryWhatIfForecastOutput::forecast): <p>Provides information about a forecast. Returned as part of the <code>QueryForecast</code> response.</p>
                            /// - On failure, responds with [`SdkError<QueryWhatIfForecastError>`](crate::error::QueryWhatIfForecastError)
    pub fn query_what_if_forecast(&self) -> crate::client::fluent_builders::QueryWhatIfForecast {
                                crate::client::fluent_builders::QueryWhatIfForecast::new(self.handle.clone())
                            }
}

