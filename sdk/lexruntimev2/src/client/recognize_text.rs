// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RecognizeText`](crate::client::fluent_builders::RecognizeText) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bot_id(impl Into<String>)`](crate::client::fluent_builders::RecognizeText::bot_id) / [`set_bot_id(Option<String>)`](crate::client::fluent_builders::RecognizeText::set_bot_id): <p>The identifier of the bot that processes the request.</p>
    ///   - [`bot_alias_id(impl Into<String>)`](crate::client::fluent_builders::RecognizeText::bot_alias_id) / [`set_bot_alias_id(Option<String>)`](crate::client::fluent_builders::RecognizeText::set_bot_alias_id): <p>The alias identifier in use for the bot that processes the request.</p>
    ///   - [`locale_id(impl Into<String>)`](crate::client::fluent_builders::RecognizeText::locale_id) / [`set_locale_id(Option<String>)`](crate::client::fluent_builders::RecognizeText::set_locale_id): <p>The locale where the session is in use.</p>
    ///   - [`session_id(impl Into<String>)`](crate::client::fluent_builders::RecognizeText::session_id) / [`set_session_id(Option<String>)`](crate::client::fluent_builders::RecognizeText::set_session_id): <p>The identifier of the user session that is having the conversation.</p>
    ///   - [`text(impl Into<String>)`](crate::client::fluent_builders::RecognizeText::text) / [`set_text(Option<String>)`](crate::client::fluent_builders::RecognizeText::set_text): <p>The text that the user entered. Amazon Lex V2 interprets this text.</p>
    ///   - [`session_state(SessionState)`](crate::client::fluent_builders::RecognizeText::session_state) / [`set_session_state(Option<SessionState>)`](crate::client::fluent_builders::RecognizeText::set_session_state): <p>The current state of the dialog between the user and the bot.</p>
    ///   - [`request_attributes(HashMap<String, String>)`](crate::client::fluent_builders::RecognizeText::request_attributes) / [`set_request_attributes(Option<HashMap<String, String>>)`](crate::client::fluent_builders::RecognizeText::set_request_attributes): <p>Request-specific information passed between the client application and Amazon Lex V2 </p>  <p>The namespace <code>x-amz-lex:</code> is reserved for special attributes. Don't create any request attributes with the prefix <code>x-amz-lex:</code>.</p>
                            /// - On success, responds with [`RecognizeTextOutput`](crate::output::RecognizeTextOutput) with field(s):
    ///   - [`messages(Option<Vec<Message>>)`](crate::output::RecognizeTextOutput::messages): <p>A list of messages last sent to the user. The messages are ordered based on the order that you returned the messages from your Lambda function or the order that the messages are defined in the bot.</p>
    ///   - [`session_state(Option<SessionState>)`](crate::output::RecognizeTextOutput::session_state): <p>Represents the current state of the dialog between the user and the bot. </p>  <p>Use this to determine the progress of the conversation and what the next action may be.</p>
    ///   - [`interpretations(Option<Vec<Interpretation>>)`](crate::output::RecognizeTextOutput::interpretations): <p>A list of intents that Amazon Lex V2 determined might satisfy the user's utterance. </p>  <p>Each interpretation includes the intent, a score that indicates now confident Amazon Lex V2 is that the interpretation is the correct one, and an optional sentiment response that indicates the sentiment expressed in the utterance.</p>
    ///   - [`request_attributes(Option<HashMap<String, String>>)`](crate::output::RecognizeTextOutput::request_attributes): <p>The attributes sent in the request.</p>
    ///   - [`session_id(Option<String>)`](crate::output::RecognizeTextOutput::session_id): <p>The identifier of the session in use.</p>
                            /// - On failure, responds with [`SdkError<RecognizeTextError>`](crate::error::RecognizeTextError)
    pub fn recognize_text(&self) -> crate::client::fluent_builders::RecognizeText {
                                crate::client::fluent_builders::RecognizeText::new(self.handle.clone())
                            }
}

