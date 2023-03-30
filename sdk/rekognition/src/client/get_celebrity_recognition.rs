// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCelebrityRecognition`](crate::client::fluent_builders::GetCelebrityRecognition) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetCelebrityRecognition::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::GetCelebrityRecognition::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::GetCelebrityRecognition::set_job_id): <p>Job identifier for the required celebrity recognition analysis. You can get the job identifer from a call to <code>StartCelebrityRecognition</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetCelebrityRecognition::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetCelebrityRecognition::set_max_results): <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetCelebrityRecognition::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetCelebrityRecognition::set_next_token): <p>If the previous response was incomplete (because there is more recognized celebrities to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of celebrities. </p>
    ///   - [`sort_by(CelebrityRecognitionSortBy)`](crate::client::fluent_builders::GetCelebrityRecognition::sort_by) / [`set_sort_by(Option<CelebrityRecognitionSortBy>)`](crate::client::fluent_builders::GetCelebrityRecognition::set_sort_by): <p>Sort to use for celebrities returned in <code>Celebrities</code> field. Specify <code>ID</code> to sort by the celebrity identifier, specify <code>TIMESTAMP</code> to sort by the time the celebrity was recognized.</p>
                            /// - On success, responds with [`GetCelebrityRecognitionOutput`](crate::output::GetCelebrityRecognitionOutput) with field(s):
    ///   - [`job_status(Option<VideoJobStatus>)`](crate::output::GetCelebrityRecognitionOutput::job_status): <p>The current status of the celebrity recognition job.</p>
    ///   - [`status_message(Option<String>)`](crate::output::GetCelebrityRecognitionOutput::status_message): <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    ///   - [`video_metadata(Option<VideoMetadata>)`](crate::output::GetCelebrityRecognitionOutput::video_metadata): <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetCelebrityRecognitionOutput::next_token): <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    ///   - [`celebrities(Option<Vec<CelebrityRecognition>>)`](crate::output::GetCelebrityRecognitionOutput::celebrities): <p>Array of celebrities recognized in the video.</p>
                            /// - On failure, responds with [`SdkError<GetCelebrityRecognitionError>`](crate::error::GetCelebrityRecognitionError)
    pub fn get_celebrity_recognition(&self) -> crate::client::fluent_builders::GetCelebrityRecognition {
                                crate::client::fluent_builders::GetCelebrityRecognition::new(self.handle.clone())
                            }
}

