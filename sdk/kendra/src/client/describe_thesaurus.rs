// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeThesaurus`](crate::client::fluent_builders::DescribeThesaurus) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DescribeThesaurus::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DescribeThesaurus::set_id): <p>The identifier of the thesaurus you want to get information on.</p>
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::DescribeThesaurus::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::DescribeThesaurus::set_index_id): <p>The identifier of the index for the thesaurus.</p>
                            /// - On success, responds with [`DescribeThesaurusOutput`](crate::output::DescribeThesaurusOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::DescribeThesaurusOutput::id): <p>The identifier of the thesaurus.</p>
    ///   - [`index_id(Option<String>)`](crate::output::DescribeThesaurusOutput::index_id): <p>The identifier of the index for the thesaurus.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeThesaurusOutput::name): <p>The thesaurus name.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeThesaurusOutput::description): <p>The thesaurus description.</p>
    ///   - [`status(Option<ThesaurusStatus>)`](crate::output::DescribeThesaurusOutput::status): <p>The current status of the thesaurus. When the value is <code>ACTIVE</code>, queries are able to use the thesaurus. If the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field provides more information. </p>  <p>If the status is <code>ACTIVE_BUT_UPDATE_FAILED</code>, it means that Amazon Kendra could not ingest the new thesaurus file. The old thesaurus file is still active. </p>
    ///   - [`error_message(Option<String>)`](crate::output::DescribeThesaurusOutput::error_message): <p>When the <code>Status</code> field value is <code>FAILED</code>, the <code>ErrorMessage</code> field provides more information. </p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::DescribeThesaurusOutput::created_at): <p>The Unix datetime that the thesaurus was created.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::output::DescribeThesaurusOutput::updated_at): <p>The Unix datetime that the thesaurus was last updated.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribeThesaurusOutput::role_arn): <p>An IAM role that gives Amazon Kendra permissions to access thesaurus file specified in <code>SourceS3Path</code>. </p>
    ///   - [`source_s3_path(Option<S3Path>)`](crate::output::DescribeThesaurusOutput::source_s3_path): <p>Information required to find a specific file in an Amazon S3 bucket.</p>
    ///   - [`file_size_bytes(Option<i64>)`](crate::output::DescribeThesaurusOutput::file_size_bytes): <p>The size of the thesaurus file in bytes.</p>
    ///   - [`term_count(Option<i64>)`](crate::output::DescribeThesaurusOutput::term_count): <p>The number of unique terms in the thesaurus file. For example, the synonyms <code>a,b,c</code> and <code>a=&gt;d</code>, the term count would be 4. </p>
    ///   - [`synonym_rule_count(Option<i64>)`](crate::output::DescribeThesaurusOutput::synonym_rule_count): <p>The number of synonym rules in the thesaurus file.</p>
                            /// - On failure, responds with [`SdkError<DescribeThesaurusError>`](crate::error::DescribeThesaurusError)
    pub fn describe_thesaurus(&self) -> crate::client::fluent_builders::DescribeThesaurus {
                                crate::client::fluent_builders::DescribeThesaurus::new(self.handle.clone())
                            }
}

