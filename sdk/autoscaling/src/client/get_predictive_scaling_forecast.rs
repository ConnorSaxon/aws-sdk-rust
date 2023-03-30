// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPredictiveScalingForecast`](crate::client::fluent_builders::GetPredictiveScalingForecast) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`auto_scaling_group_name(impl Into<String>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::auto_scaling_group_name) / [`set_auto_scaling_group_name(Option<String>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::set_auto_scaling_group_name): <p>The name of the Auto Scaling group.</p>
    ///   - [`policy_name(impl Into<String>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::policy_name) / [`set_policy_name(Option<String>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::set_policy_name): <p>The name of the policy.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::GetPredictiveScalingForecast::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::set_start_time): <p>The inclusive start time of the time range for the forecast data to get. At most, the date and time can be one year before the current date and time.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::GetPredictiveScalingForecast::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::GetPredictiveScalingForecast::set_end_time): <p>The exclusive end time of the time range for the forecast data to get. The maximum time duration between the start and end time is 30 days. </p>  <p>Although this parameter can accept a date and time that is more than two days in the future, the availability of forecast data has limits. Amazon EC2 Auto Scaling only issues forecasts for periods of two days in advance.</p>
                            /// - On success, responds with [`GetPredictiveScalingForecastOutput`](crate::output::GetPredictiveScalingForecastOutput) with field(s):
    ///   - [`load_forecast(Option<Vec<LoadForecast>>)`](crate::output::GetPredictiveScalingForecastOutput::load_forecast): <p>The load forecast.</p>
    ///   - [`capacity_forecast(Option<CapacityForecast>)`](crate::output::GetPredictiveScalingForecastOutput::capacity_forecast): <p>The capacity forecast.</p>
    ///   - [`update_time(Option<DateTime>)`](crate::output::GetPredictiveScalingForecastOutput::update_time): <p>The time the forecast was made.</p>
                            /// - On failure, responds with [`SdkError<GetPredictiveScalingForecastError>`](crate::error::GetPredictiveScalingForecastError)
    pub fn get_predictive_scaling_forecast(&self) -> crate::client::fluent_builders::GetPredictiveScalingForecast {
                                crate::client::fluent_builders::GetPredictiveScalingForecast::new(self.handle.clone())
                            }
}

