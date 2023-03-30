// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeIntent`](crate::client::fluent_builders::DescribeIntent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`intent_id(impl Into<String>)`](crate::client::fluent_builders::DescribeIntent::intent_id) / [`set_intent_id(Option<String>)`](crate::client::fluent_builders::DescribeIntent::set_intent_id): <p>The identifier of the intent to describe.</p>
    ///   - [`bot_id(impl Into<String>)`](crate::client::fluent_builders::DescribeIntent::bot_id) / [`set_bot_id(Option<String>)`](crate::client::fluent_builders::DescribeIntent::set_bot_id): <p>The identifier of the bot associated with the intent.</p>
    ///   - [`bot_version(impl Into<String>)`](crate::client::fluent_builders::DescribeIntent::bot_version) / [`set_bot_version(Option<String>)`](crate::client::fluent_builders::DescribeIntent::set_bot_version): <p>The version of the bot associated with the intent.</p>
    ///   - [`locale_id(impl Into<String>)`](crate::client::fluent_builders::DescribeIntent::locale_id) / [`set_locale_id(Option<String>)`](crate::client::fluent_builders::DescribeIntent::set_locale_id): <p>The identifier of the language and locale of the intent to describe. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
                            /// - On success, responds with [`DescribeIntentOutput`](crate::output::DescribeIntentOutput) with field(s):
    ///   - [`intent_id(Option<String>)`](crate::output::DescribeIntentOutput::intent_id): <p>The unique identifier assigned to the intent when it was created.</p>
    ///   - [`intent_name(Option<String>)`](crate::output::DescribeIntentOutput::intent_name): <p>The name specified for the intent.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeIntentOutput::description): <p>The description of the intent.</p>
    ///   - [`parent_intent_signature(Option<String>)`](crate::output::DescribeIntentOutput::parent_intent_signature): <p>The identifier of the built-in intent that this intent is derived from, if any.</p>
    ///   - [`sample_utterances(Option<Vec<SampleUtterance>>)`](crate::output::DescribeIntentOutput::sample_utterances): <p>User utterances that trigger this intent.</p>
    ///   - [`dialog_code_hook(Option<DialogCodeHookSettings>)`](crate::output::DescribeIntentOutput::dialog_code_hook): <p>The Lambda function called during each turn of a conversation with the intent.</p>
    ///   - [`fulfillment_code_hook(Option<FulfillmentCodeHookSettings>)`](crate::output::DescribeIntentOutput::fulfillment_code_hook): <p>The Lambda function called when the intent is complete and ready for fulfillment.</p>
    ///   - [`slot_priorities(Option<Vec<SlotPriority>>)`](crate::output::DescribeIntentOutput::slot_priorities): <p>The list that determines the priority that slots should be elicited from the user.</p>
    ///   - [`intent_confirmation_setting(Option<IntentConfirmationSetting>)`](crate::output::DescribeIntentOutput::intent_confirmation_setting): <p>Prompts that Amazon Lex sends to the user to confirm completion of an intent.</p>
    ///   - [`intent_closing_setting(Option<IntentClosingSetting>)`](crate::output::DescribeIntentOutput::intent_closing_setting): <p>The response that Amazon Lex sends to when the intent is closed.</p>
    ///   - [`input_contexts(Option<Vec<InputContext>>)`](crate::output::DescribeIntentOutput::input_contexts): <p>A list of contexts that must be active for the intent to be considered for sending to the user.</p>
    ///   - [`output_contexts(Option<Vec<OutputContext>>)`](crate::output::DescribeIntentOutput::output_contexts): <p>A list of contexts that are activated when the intent is fulfilled.</p>
    ///   - [`kendra_configuration(Option<KendraConfiguration>)`](crate::output::DescribeIntentOutput::kendra_configuration): <p>Configuration information required to use the <code>AMAZON.KendraSearchIntent</code> intent.</p>
    ///   - [`bot_id(Option<String>)`](crate::output::DescribeIntentOutput::bot_id): <p>The identifier of the bot associated with the intent.</p>
    ///   - [`bot_version(Option<String>)`](crate::output::DescribeIntentOutput::bot_version): <p>The version of the bot associated with the intent.</p>
    ///   - [`locale_id(Option<String>)`](crate::output::DescribeIntentOutput::locale_id): <p>The language and locale specified for the intent.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::output::DescribeIntentOutput::creation_date_time): <p>A timestamp of the date and time that the intent was created.</p>
    ///   - [`last_updated_date_time(Option<DateTime>)`](crate::output::DescribeIntentOutput::last_updated_date_time): <p>A timestamp of the date and time that the intent was last updated.</p>
    ///   - [`initial_response_setting(Option<InitialResponseSetting>)`](crate::output::DescribeIntentOutput::initial_response_setting): <p></p>
                            /// - On failure, responds with [`SdkError<DescribeIntentError>`](crate::error::DescribeIntentError)
    pub fn describe_intent(&self) -> crate::client::fluent_builders::DescribeIntent {
                                crate::client::fluent_builders::DescribeIntent::new(self.handle.clone())
                            }
}

