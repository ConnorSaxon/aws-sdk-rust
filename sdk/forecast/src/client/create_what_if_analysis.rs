// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateWhatIfAnalysis`](crate::client::fluent_builders::CreateWhatIfAnalysis) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`what_if_analysis_name(impl Into<String>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::what_if_analysis_name) / [`set_what_if_analysis_name(Option<String>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::set_what_if_analysis_name): <p>The name of the what-if analysis. Each name must be unique.</p>
    ///   - [`forecast_arn(impl Into<String>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::forecast_arn) / [`set_forecast_arn(Option<String>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::set_forecast_arn): <p>The Amazon Resource Name (ARN) of the baseline forecast.</p>
    ///   - [`time_series_selector(TimeSeriesSelector)`](crate::client::fluent_builders::CreateWhatIfAnalysis::time_series_selector) / [`set_time_series_selector(Option<TimeSeriesSelector>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::set_time_series_selector): <p>Defines the set of time series that are used in the what-if analysis with a <code>TimeSeriesIdentifiers</code> object. What-if analyses are performed only for the time series in this object.</p>  <p>The <code>TimeSeriesIdentifiers</code> object needs the following information:</p>  <ul>   <li> <p> <code>DataSource</code> </p> </li>   <li> <p> <code>Format</code> </p> </li>   <li> <p> <code>Schema</code> </p> </li>  </ul>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateWhatIfAnalysis::set_tags): <p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p>
                            /// - On success, responds with [`CreateWhatIfAnalysisOutput`](crate::output::CreateWhatIfAnalysisOutput) with field(s):
    ///   - [`what_if_analysis_arn(Option<String>)`](crate::output::CreateWhatIfAnalysisOutput::what_if_analysis_arn): <p>The Amazon Resource Name (ARN) of the what-if analysis.</p>
                            /// - On failure, responds with [`SdkError<CreateWhatIfAnalysisError>`](crate::error::CreateWhatIfAnalysisError)
    pub fn create_what_if_analysis(&self) -> crate::client::fluent_builders::CreateWhatIfAnalysis {
                                crate::client::fluent_builders::CreateWhatIfAnalysis::new(self.handle.clone())
                            }
}

