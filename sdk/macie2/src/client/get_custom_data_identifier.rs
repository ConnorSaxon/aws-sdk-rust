// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCustomDataIdentifier`](crate::client::fluent_builders::GetCustomDataIdentifier) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetCustomDataIdentifier::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetCustomDataIdentifier::set_id): <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
                            /// - On success, responds with [`GetCustomDataIdentifierOutput`](crate::output::GetCustomDataIdentifierOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::GetCustomDataIdentifierOutput::arn): <p>The Amazon Resource Name (ARN) of the custom data identifier.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetCustomDataIdentifierOutput::created_at): <p>The date and time, in UTC and extended ISO 8601 format, when the custom data identifier was created.</p>
    ///   - [`deleted(bool)`](crate::output::GetCustomDataIdentifierOutput::deleted): <p>Specifies whether the custom data identifier was deleted. If you delete a custom data identifier, Amazon Macie doesn't delete it permanently. Instead, it soft deletes the identifier.</p>
    ///   - [`description(Option<String>)`](crate::output::GetCustomDataIdentifierOutput::description): <p>The custom description of the custom data identifier.</p>
    ///   - [`id(Option<String>)`](crate::output::GetCustomDataIdentifierOutput::id): <p>The unique identifier for the custom data identifier.</p>
    ///   - [`ignore_words(Option<Vec<String>>)`](crate::output::GetCustomDataIdentifierOutput::ignore_words): <p>An array that lists specific character sequences (<i>ignore words</i>) to exclude from the results. If the text matched by the regular expression contains any string in this array, Amazon Macie ignores it. Ignore words are case sensitive.</p>
    ///   - [`keywords(Option<Vec<String>>)`](crate::output::GetCustomDataIdentifierOutput::keywords): <p>An array that lists specific character sequences (<i>keywords</i>), one of which must precede and be within proximity (maximumMatchDistance) of the regular expression to match. Keywords aren't case sensitive.</p>
    ///   - [`maximum_match_distance(i32)`](crate::output::GetCustomDataIdentifierOutput::maximum_match_distance): <p>The maximum number of characters that can exist between the end of at least one complete character sequence specified by the keywords array and the end of the text that matches the regex pattern. If a complete keyword precedes all the text that matches the pattern and the keyword is within the specified distance, Amazon Macie includes the result. Otherwise, Macie excludes the result.</p>
    ///   - [`name(Option<String>)`](crate::output::GetCustomDataIdentifierOutput::name): <p>The custom name of the custom data identifier.</p>
    ///   - [`regex(Option<String>)`](crate::output::GetCustomDataIdentifierOutput::regex): <p>The regular expression (<i>regex</i>) that defines the pattern to match.</p>
    ///   - [`severity_levels(Option<Vec<SeverityLevel>>)`](crate::output::GetCustomDataIdentifierOutput::severity_levels): <p>Specifies the severity that's assigned to findings that the custom data identifier produces, based on the number of occurrences of text that matches the custom data identifier's detection criteria. By default, Amazon Macie creates findings for S3 objects that contain at least one occurrence of text that matches the detection criteria, and Macie assigns the MEDIUM severity to those findings.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetCustomDataIdentifierOutput::tags): <p>A map of key-value pairs that identifies the tags (keys and values) that are associated with the custom data identifier.</p>
                            /// - On failure, responds with [`SdkError<GetCustomDataIdentifierError>`](crate::error::GetCustomDataIdentifierError)
    pub fn get_custom_data_identifier(&self) -> crate::client::fluent_builders::GetCustomDataIdentifier {
                                crate::client::fluent_builders::GetCustomDataIdentifier::new(self.handle.clone())
                            }
}

