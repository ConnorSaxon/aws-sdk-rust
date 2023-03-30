// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateTrigger`](crate::client::fluent_builders::CreateTrigger) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateTrigger::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateTrigger::set_name): <p>The name of the trigger.</p>
    ///   - [`workflow_name(impl Into<String>)`](crate::client::fluent_builders::CreateTrigger::workflow_name) / [`set_workflow_name(Option<String>)`](crate::client::fluent_builders::CreateTrigger::set_workflow_name): <p>The name of the workflow associated with the trigger.</p>
    ///   - [`r#type(TriggerType)`](crate::client::fluent_builders::CreateTrigger::type) / [`set_type(Option<TriggerType>)`](crate::client::fluent_builders::CreateTrigger::set_type): <p>The type of the new trigger.</p>
    ///   - [`schedule(impl Into<String>)`](crate::client::fluent_builders::CreateTrigger::schedule) / [`set_schedule(Option<String>)`](crate::client::fluent_builders::CreateTrigger::set_schedule): <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>  <p>This field is required when the trigger type is SCHEDULED.</p>
    ///   - [`predicate(Predicate)`](crate::client::fluent_builders::CreateTrigger::predicate) / [`set_predicate(Option<Predicate>)`](crate::client::fluent_builders::CreateTrigger::set_predicate): <p>A predicate to specify when the new trigger should fire.</p>  <p>This field is required when the trigger type is <code>CONDITIONAL</code>.</p>
    ///   - [`actions(Vec<Action>)`](crate::client::fluent_builders::CreateTrigger::actions) / [`set_actions(Option<Vec<Action>>)`](crate::client::fluent_builders::CreateTrigger::set_actions): <p>The actions initiated by this trigger when it fires.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateTrigger::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateTrigger::set_description): <p>A description of the new trigger.</p>
    ///   - [`start_on_creation(bool)`](crate::client::fluent_builders::CreateTrigger::start_on_creation) / [`set_start_on_creation(bool)`](crate::client::fluent_builders::CreateTrigger::set_start_on_creation): <p>Set to <code>true</code> to start <code>SCHEDULED</code> and <code>CONDITIONAL</code> triggers when created. True is not supported for <code>ON_DEMAND</code> triggers.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateTrigger::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateTrigger::set_tags): <p>The tags to use with this trigger. You may use tags to limit access to the trigger. For more information about tags in Glue, see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-tags.html">Amazon Web Services Tags in Glue</a> in the developer guide. </p>
    ///   - [`event_batching_condition(EventBatchingCondition)`](crate::client::fluent_builders::CreateTrigger::event_batching_condition) / [`set_event_batching_condition(Option<EventBatchingCondition>)`](crate::client::fluent_builders::CreateTrigger::set_event_batching_condition): <p>Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires.</p>
                            /// - On success, responds with [`CreateTriggerOutput`](crate::output::CreateTriggerOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateTriggerOutput::name): <p>The name of the trigger.</p>
                            /// - On failure, responds with [`SdkError<CreateTriggerError>`](crate::error::CreateTriggerError)
    pub fn create_trigger(&self) -> crate::client::fluent_builders::CreateTrigger {
                                crate::client::fluent_builders::CreateTrigger::new(self.handle.clone())
                            }
}

