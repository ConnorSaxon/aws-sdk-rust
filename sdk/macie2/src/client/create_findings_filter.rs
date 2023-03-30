// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFindingsFilter`](crate::client::fluent_builders::CreateFindingsFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`action(FindingsFilterAction)`](crate::client::fluent_builders::CreateFindingsFilter::action) / [`set_action(Option<FindingsFilterAction>)`](crate::client::fluent_builders::CreateFindingsFilter::set_action): <p>The action to perform on findings that match the filter criteria (findingCriteria). Valid values are: ARCHIVE, suppress (automatically archive) the findings; and, NOOP, don't perform any action on the findings.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateFindingsFilter::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateFindingsFilter::set_client_token): <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateFindingsFilter::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateFindingsFilter::set_description): <p>A custom description of the filter. The description can contain as many as 512 characters.</p>  <p>We strongly recommend that you avoid including any sensitive data in the description of a filter. Other users of your account might be able to see this description, depending on the actions that they're allowed to perform in Amazon Macie.</p>
    ///   - [`finding_criteria(FindingCriteria)`](crate::client::fluent_builders::CreateFindingsFilter::finding_criteria) / [`set_finding_criteria(Option<FindingCriteria>)`](crate::client::fluent_builders::CreateFindingsFilter::set_finding_criteria): <p>The criteria to use to filter findings.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateFindingsFilter::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateFindingsFilter::set_name): <p>A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters.</p>  <p>We strongly recommend that you avoid including any sensitive data in the name of a filter. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p>
    ///   - [`position(i32)`](crate::client::fluent_builders::CreateFindingsFilter::position) / [`set_position(i32)`](crate::client::fluent_builders::CreateFindingsFilter::set_position): <p>The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateFindingsFilter::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateFindingsFilter::set_tags): <p>A map of key-value pairs that specifies the tags to associate with the filter.</p>  <p>A findings filter can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
                            /// - On success, responds with [`CreateFindingsFilterOutput`](crate::output::CreateFindingsFilterOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::CreateFindingsFilterOutput::arn): <p>The Amazon Resource Name (ARN) of the filter that was created.</p>
    ///   - [`id(Option<String>)`](crate::output::CreateFindingsFilterOutput::id): <p>The unique identifier for the filter that was created.</p>
                            /// - On failure, responds with [`SdkError<CreateFindingsFilterError>`](crate::error::CreateFindingsFilterError)
    pub fn create_findings_filter(&self) -> crate::client::fluent_builders::CreateFindingsFilter {
                                crate::client::fluent_builders::CreateFindingsFilter::new(self.handle.clone())
                            }
}

