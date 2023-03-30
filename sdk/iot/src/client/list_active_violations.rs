// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListActiveViolations`](crate::client::fluent_builders::ListActiveViolations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListActiveViolations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`thing_name(impl Into<String>)`](crate::client::fluent_builders::ListActiveViolations::thing_name) / [`set_thing_name(Option<String>)`](crate::client::fluent_builders::ListActiveViolations::set_thing_name): <p>The name of the thing whose active violations are listed.</p>
    ///   - [`security_profile_name(impl Into<String>)`](crate::client::fluent_builders::ListActiveViolations::security_profile_name) / [`set_security_profile_name(Option<String>)`](crate::client::fluent_builders::ListActiveViolations::set_security_profile_name): <p>The name of the Device Defender security profile for which violations are listed.</p>
    ///   - [`behavior_criteria_type(BehaviorCriteriaType)`](crate::client::fluent_builders::ListActiveViolations::behavior_criteria_type) / [`set_behavior_criteria_type(Option<BehaviorCriteriaType>)`](crate::client::fluent_builders::ListActiveViolations::set_behavior_criteria_type): <p> The criteria for a behavior. </p>
    ///   - [`list_suppressed_alerts(bool)`](crate::client::fluent_builders::ListActiveViolations::list_suppressed_alerts) / [`set_list_suppressed_alerts(Option<bool>)`](crate::client::fluent_builders::ListActiveViolations::set_list_suppressed_alerts): <p> A list of all suppressed alerts. </p>
    ///   - [`verification_state(VerificationState)`](crate::client::fluent_builders::ListActiveViolations::verification_state) / [`set_verification_state(Option<VerificationState>)`](crate::client::fluent_builders::ListActiveViolations::set_verification_state): <p>The verification state of the violation (detect alarm).</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListActiveViolations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListActiveViolations::set_next_token): <p>The token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListActiveViolations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListActiveViolations::set_max_results): <p>The maximum number of results to return at one time.</p>
                            /// - On success, responds with [`ListActiveViolationsOutput`](crate::output::ListActiveViolationsOutput) with field(s):
    ///   - [`active_violations(Option<Vec<ActiveViolation>>)`](crate::output::ListActiveViolationsOutput::active_violations): <p>The list of active violations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListActiveViolationsOutput::next_token): <p>A token that can be used to retrieve the next set of results, or <code>null</code> if there are no additional results.</p>
                            /// - On failure, responds with [`SdkError<ListActiveViolationsError>`](crate::error::ListActiveViolationsError)
    pub fn list_active_violations(&self) -> crate::client::fluent_builders::ListActiveViolations {
                                crate::client::fluent_builders::ListActiveViolations::new(self.handle.clone())
                            }
}

