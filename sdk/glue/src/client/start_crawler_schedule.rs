// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartCrawlerSchedule`](crate::client::fluent_builders::StartCrawlerSchedule) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`crawler_name(impl Into<String>)`](crate::client::fluent_builders::StartCrawlerSchedule::crawler_name) / [`set_crawler_name(Option<String>)`](crate::client::fluent_builders::StartCrawlerSchedule::set_crawler_name): <p>Name of the crawler to schedule.</p>
                            /// - On success, responds with [`StartCrawlerScheduleOutput`](crate::output::StartCrawlerScheduleOutput)
                            /// - On failure, responds with [`SdkError<StartCrawlerScheduleError>`](crate::error::StartCrawlerScheduleError)
    pub fn start_crawler_schedule(&self) -> crate::client::fluent_builders::StartCrawlerSchedule {
                                crate::client::fluent_builders::StartCrawlerSchedule::new(self.handle.clone())
                            }
}

