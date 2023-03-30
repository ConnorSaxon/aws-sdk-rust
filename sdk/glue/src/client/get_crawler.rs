// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCrawler`](crate::client::fluent_builders::GetCrawler) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetCrawler::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetCrawler::set_name): <p>The name of the crawler to retrieve metadata for.</p>
                            /// - On success, responds with [`GetCrawlerOutput`](crate::output::GetCrawlerOutput) with field(s):
    ///   - [`crawler(Option<Crawler>)`](crate::output::GetCrawlerOutput::crawler): <p>The metadata for the specified crawler.</p>
                            /// - On failure, responds with [`SdkError<GetCrawlerError>`](crate::error::GetCrawlerError)
    pub fn get_crawler(&self) -> crate::client::fluent_builders::GetCrawler {
                                crate::client::fluent_builders::GetCrawler::new(self.handle.clone())
                            }
}

