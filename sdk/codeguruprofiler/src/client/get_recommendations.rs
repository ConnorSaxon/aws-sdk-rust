// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRecommendations`](crate::client::fluent_builders::GetRecommendations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`profiling_group_name(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::profiling_group_name) / [`set_profiling_group_name(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_profiling_group_name): <p> The name of the profiling group to get analysis data about. </p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::GetRecommendations::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::GetRecommendations::set_start_time): <p> The end time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::GetRecommendations::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::GetRecommendations::set_end_time): <p> The start time of the profile to get analysis data about. You must specify <code>startTime</code> and <code>endTime</code>. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`locale(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::locale) / [`set_locale(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_locale): <p> The language used to provide analysis. Specify using a string that is one of the following <code>BCP 47</code> language codes. </p>  <ul>   <li> <p> <code>de-DE</code> - German, Germany </p> </li>   <li> <p> <code>en-GB</code> - English, United Kingdom </p> </li>   <li> <p> <code>en-US</code> - English, United States </p> </li>   <li> <p> <code>es-ES</code> - Spanish, Spain </p> </li>   <li> <p> <code>fr-FR</code> - French, France </p> </li>   <li> <p> <code>it-IT</code> - Italian, Italy </p> </li>   <li> <p> <code>ja-JP</code> - Japanese, Japan </p> </li>   <li> <p> <code>ko-KR</code> - Korean, Republic of Korea </p> </li>   <li> <p> <code>pt-BR</code> - Portugese, Brazil </p> </li>   <li> <p> <code>zh-CN</code> - Chinese, China </p> </li>   <li> <p> <code>zh-TW</code> - Chinese, Taiwan </p> </li>  </ul>
                            /// - On success, responds with [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput) with field(s):
    ///   - [`profiling_group_name(Option<String>)`](crate::output::GetRecommendationsOutput::profiling_group_name): <p>The name of the profiling group the analysis data is about.</p>
    ///   - [`profile_start_time(Option<DateTime>)`](crate::output::GetRecommendationsOutput::profile_start_time): <p> The start time of the profile the analysis data is about. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`profile_end_time(Option<DateTime>)`](crate::output::GetRecommendationsOutput::profile_end_time): <p> The end time of the profile the analysis data is about. This is specified using the ISO 8601 format. For example, 2020-06-01T13:15:02.001Z represents 1 millisecond past June 1, 2020 1:15:02 PM UTC. </p>
    ///   - [`recommendations(Option<Vec<Recommendation>>)`](crate::output::GetRecommendationsOutput::recommendations): <p>The list of recommendations that the analysis found for this profile.</p>
    ///   - [`anomalies(Option<Vec<Anomaly>>)`](crate::output::GetRecommendationsOutput::anomalies): <p> The list of anomalies that the analysis has found for this profile. </p>
                            /// - On failure, responds with [`SdkError<GetRecommendationsError>`](crate::error::GetRecommendationsError)
    pub fn get_recommendations(&self) -> crate::client::fluent_builders::GetRecommendations {
                                crate::client::fluent_builders::GetRecommendations::new(self.handle.clone())
                            }
}

