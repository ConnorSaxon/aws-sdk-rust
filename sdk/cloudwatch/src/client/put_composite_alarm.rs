// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutCompositeAlarm`](crate::client::fluent_builders::PutCompositeAlarm) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`actions_enabled(bool)`](crate::client::fluent_builders::PutCompositeAlarm::actions_enabled) / [`set_actions_enabled(Option<bool>)`](crate::client::fluent_builders::PutCompositeAlarm::set_actions_enabled): <p>Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. The default is <code>TRUE</code>.</p>
    ///   - [`alarm_actions(Vec<String>)`](crate::client::fluent_builders::PutCompositeAlarm::alarm_actions) / [`set_alarm_actions(Option<Vec<String>>)`](crate::client::fluent_builders::PutCompositeAlarm::set_alarm_actions): <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>  <p>Valid Values: <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> | <code>arn:aws:ssm:<i>region</i>:<i>account-id</i>:opsitem:<i>severity</i> </code> </p>
    ///   - [`alarm_description(impl Into<String>)`](crate::client::fluent_builders::PutCompositeAlarm::alarm_description) / [`set_alarm_description(Option<String>)`](crate::client::fluent_builders::PutCompositeAlarm::set_alarm_description): <p>The description for the composite alarm.</p>
    ///   - [`alarm_name(impl Into<String>)`](crate::client::fluent_builders::PutCompositeAlarm::alarm_name) / [`set_alarm_name(Option<String>)`](crate::client::fluent_builders::PutCompositeAlarm::set_alarm_name): <p>The name for the composite alarm. This name must be unique within the Region.</p>
    ///   - [`alarm_rule(impl Into<String>)`](crate::client::fluent_builders::PutCompositeAlarm::alarm_rule) / [`set_alarm_rule(Option<String>)`](crate::client::fluent_builders::PutCompositeAlarm::set_alarm_rule): <p>An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For each alarm that you reference, you designate a function that specifies whether that alarm needs to be in ALARM state, OK state, or INSUFFICIENT_DATA state. You can use operators (AND, OR and NOT) to combine multiple functions in a single expression. You can use parenthesis to logically group the functions in your expression.</p>  <p>You can use either alarm names or ARNs to reference the other alarms that are to be evaluated.</p>  <p>Functions can include the following:</p>  <ul>   <li> <p> <code>ALARM("<i>alarm-name</i> or <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in ALARM state.</p> </li>   <li> <p> <code>OK("<i>alarm-name</i> or <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in OK state.</p> </li>   <li> <p> <code>INSUFFICIENT_DATA("<i>alarm-name</i> or <i>alarm-ARN</i>")</code> is TRUE if the named alarm is in INSUFFICIENT_DATA state.</p> </li>   <li> <p> <code>TRUE</code> always evaluates to TRUE.</p> </li>   <li> <p> <code>FALSE</code> always evaluates to FALSE.</p> </li>  </ul>  <p>TRUE and FALSE are useful for testing a complex <code>AlarmRule</code> structure, and for testing your alarm actions.</p>  <p>Alarm names specified in <code>AlarmRule</code> can be surrounded with double-quotes ("), but do not have to be.</p>  <p>The following are some examples of <code>AlarmRule</code>:</p>  <ul>   <li> <p> <code>ALARM(CPUUtilizationTooHigh) AND ALARM(DiskReadOpsTooHigh)</code> specifies that the composite alarm goes into ALARM state only if both CPUUtilizationTooHigh and DiskReadOpsTooHigh alarms are in ALARM state.</p> </li>   <li> <p> <code>ALARM(CPUUtilizationTooHigh) AND NOT ALARM(DeploymentInProgress)</code> specifies that the alarm goes to ALARM state if CPUUtilizationTooHigh is in ALARM state and DeploymentInProgress is not in ALARM state. This example reduces alarm noise during a known deployment window.</p> </li>   <li> <p> <code>(ALARM(CPUUtilizationTooHigh) OR ALARM(DiskReadOpsTooHigh)) AND OK(NetworkOutTooHigh)</code> goes into ALARM state if CPUUtilizationTooHigh OR DiskReadOpsTooHigh is in ALARM state, and if NetworkOutTooHigh is in OK state. This provides another example of using a composite alarm to prevent noise. This rule ensures that you are not notified with an alarm action on high CPU or disk usage if a known network problem is also occurring.</p> </li>  </ul>  <p>The <code>AlarmRule</code> can specify as many as 100 "children" alarms. The <code>AlarmRule</code> expression can have as many as 500 elements. Elements are child alarms, TRUE or FALSE statements, and parentheses.</p>
    ///   - [`insufficient_data_actions(Vec<String>)`](crate::client::fluent_builders::PutCompositeAlarm::insufficient_data_actions) / [`set_insufficient_data_actions(Option<Vec<String>>)`](crate::client::fluent_builders::PutCompositeAlarm::set_insufficient_data_actions): <p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>  <p>Valid Values: <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> </p>
    ///   - [`ok_actions(Vec<String>)`](crate::client::fluent_builders::PutCompositeAlarm::ok_actions) / [`set_ok_actions(Option<Vec<String>>)`](crate::client::fluent_builders::PutCompositeAlarm::set_ok_actions): <p>The actions to execute when this alarm transitions to an <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>  <p>Valid Values: <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::PutCompositeAlarm::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::PutCompositeAlarm::set_tags): <p>A list of key-value pairs to associate with the composite alarm. You can associate as many as 50 tags with an alarm.</p>  <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values.</p>
    ///   - [`actions_suppressor(impl Into<String>)`](crate::client::fluent_builders::PutCompositeAlarm::actions_suppressor) / [`set_actions_suppressor(Option<String>)`](crate::client::fluent_builders::PutCompositeAlarm::set_actions_suppressor): <p> Actions will be suppressed if the suppressor alarm is in the <code>ALARM</code> state. <code>ActionsSuppressor</code> can be an AlarmName or an Amazon Resource Name (ARN) from an existing alarm. </p>
    ///   - [`actions_suppressor_wait_period(i32)`](crate::client::fluent_builders::PutCompositeAlarm::actions_suppressor_wait_period) / [`set_actions_suppressor_wait_period(Option<i32>)`](crate::client::fluent_builders::PutCompositeAlarm::set_actions_suppressor_wait_period): <p> The maximum time in seconds that the composite alarm waits for the suppressor alarm to go into the <code>ALARM</code> state. After this time, the composite alarm performs its actions. </p> <important>   <p> <code>WaitPeriod</code> is required only when <code>ActionsSuppressor</code> is specified. </p>  </important>
    ///   - [`actions_suppressor_extension_period(i32)`](crate::client::fluent_builders::PutCompositeAlarm::actions_suppressor_extension_period) / [`set_actions_suppressor_extension_period(Option<i32>)`](crate::client::fluent_builders::PutCompositeAlarm::set_actions_suppressor_extension_period): <p> The maximum time in seconds that the composite alarm waits after suppressor alarm goes out of the <code>ALARM</code> state. After this time, the composite alarm performs its actions. </p> <important>   <p> <code>ExtensionPeriod</code> is required only when <code>ActionsSuppressor</code> is specified. </p>  </important>
                            /// - On success, responds with [`PutCompositeAlarmOutput`](crate::output::PutCompositeAlarmOutput)
                            /// - On failure, responds with [`SdkError<PutCompositeAlarmError>`](crate::error::PutCompositeAlarmError)
    pub fn put_composite_alarm(&self) -> crate::client::fluent_builders::PutCompositeAlarm {
                                crate::client::fluent_builders::PutCompositeAlarm::new(self.handle.clone())
                            }
}

