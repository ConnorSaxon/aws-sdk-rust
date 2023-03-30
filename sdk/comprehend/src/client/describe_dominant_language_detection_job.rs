// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDominantLanguageDetectionJob`](crate::client::fluent_builders::DescribeDominantLanguageDetectionJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::client::fluent_builders::DescribeDominantLanguageDetectionJob::job_id) / [`set_job_id(Option<String>)`](crate::client::fluent_builders::DescribeDominantLanguageDetectionJob::set_job_id): <p>The identifier that Amazon Comprehend generated for the job. The operation returns this identifier in its response.</p>
                            /// - On success, responds with [`DescribeDominantLanguageDetectionJobOutput`](crate::output::DescribeDominantLanguageDetectionJobOutput) with field(s):
    ///   - [`dominant_language_detection_job_properties(Option<DominantLanguageDetectionJobProperties>)`](crate::output::DescribeDominantLanguageDetectionJobOutput::dominant_language_detection_job_properties): <p>An object that contains the properties associated with a dominant language detection job.</p>
                            /// - On failure, responds with [`SdkError<DescribeDominantLanguageDetectionJobError>`](crate::error::DescribeDominantLanguageDetectionJobError)
    pub fn describe_dominant_language_detection_job(&self) -> crate::client::fluent_builders::DescribeDominantLanguageDetectionJob {
                                crate::client::fluent_builders::DescribeDominantLanguageDetectionJob::new(self.handle.clone())
                            }
}

