// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateAlert`](crate::client::fluent_builders::UpdateAlert) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`alert_arn(impl Into<String>)`](crate::client::fluent_builders::UpdateAlert::alert_arn) / [`set_alert_arn(Option<String>)`](crate::client::fluent_builders::UpdateAlert::set_alert_arn): <p>The ARN of the alert to update.</p>
    ///   - [`alert_description(impl Into<String>)`](crate::client::fluent_builders::UpdateAlert::alert_description) / [`set_alert_description(Option<String>)`](crate::client::fluent_builders::UpdateAlert::set_alert_description): <p>A description of the alert.</p>
    ///   - [`alert_sensitivity_threshold(i32)`](crate::client::fluent_builders::UpdateAlert::alert_sensitivity_threshold) / [`set_alert_sensitivity_threshold(i32)`](crate::client::fluent_builders::UpdateAlert::set_alert_sensitivity_threshold): <p>An integer from 0 to 100 specifying the alert sensitivity threshold.</p>
    ///   - [`action(Action)`](crate::client::fluent_builders::UpdateAlert::action) / [`set_action(Option<Action>)`](crate::client::fluent_builders::UpdateAlert::set_action): <p>Action that will be triggered when there is an alert.</p>
    ///   - [`alert_filters(AlertFilters)`](crate::client::fluent_builders::UpdateAlert::alert_filters) / [`set_alert_filters(Option<AlertFilters>)`](crate::client::fluent_builders::UpdateAlert::set_alert_filters): <p>The configuration of the alert filters, containing MetricList and DimensionFilterList.</p>
                            /// - On success, responds with [`UpdateAlertOutput`](crate::output::UpdateAlertOutput) with field(s):
    ///   - [`alert_arn(Option<String>)`](crate::output::UpdateAlertOutput::alert_arn): <p>The ARN of the updated alert.</p>
                            /// - On failure, responds with [`SdkError<UpdateAlertError>`](crate::error::UpdateAlertError)
    pub fn update_alert(&self) -> crate::client::fluent_builders::UpdateAlert {
                                crate::client::fluent_builders::UpdateAlert::new(self.handle.clone())
                            }
}

