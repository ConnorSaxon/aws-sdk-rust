// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateCrawler`](crate::client::fluent_builders::UpdateCrawler) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_name): <p>Name of the new crawler.</p>
    ///   - [`role(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::role) / [`set_role(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_role): <p>The IAM role or Amazon Resource Name (ARN) of an IAM role that is used by the new crawler to access customer resources.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_database_name): <p>The Glue database where results are stored, such as: <code>arn:aws:daylight:us-east-1::database/sometable/*</code>.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_description): <p>A description of the new crawler.</p>
    ///   - [`targets(CrawlerTargets)`](crate::client::fluent_builders::UpdateCrawler::targets) / [`set_targets(Option<CrawlerTargets>)`](crate::client::fluent_builders::UpdateCrawler::set_targets): <p>A list of targets to crawl.</p>
    ///   - [`schedule(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::schedule) / [`set_schedule(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_schedule): <p>A <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    ///   - [`classifiers(Vec<String>)`](crate::client::fluent_builders::UpdateCrawler::classifiers) / [`set_classifiers(Option<Vec<String>>)`](crate::client::fluent_builders::UpdateCrawler::set_classifiers): <p>A list of custom classifiers that the user has registered. By default, all built-in classifiers are included in a crawl, but these custom classifiers always override the default classifiers for a given classification.</p>
    ///   - [`table_prefix(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::table_prefix) / [`set_table_prefix(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_table_prefix): <p>The table prefix used for catalog tables that are created.</p>
    ///   - [`schema_change_policy(SchemaChangePolicy)`](crate::client::fluent_builders::UpdateCrawler::schema_change_policy) / [`set_schema_change_policy(Option<SchemaChangePolicy>)`](crate::client::fluent_builders::UpdateCrawler::set_schema_change_policy): <p>The policy for the crawler's update and deletion behavior.</p>
    ///   - [`recrawl_policy(RecrawlPolicy)`](crate::client::fluent_builders::UpdateCrawler::recrawl_policy) / [`set_recrawl_policy(Option<RecrawlPolicy>)`](crate::client::fluent_builders::UpdateCrawler::set_recrawl_policy): <p>A policy that specifies whether to crawl the entire dataset again, or to crawl only folders that were added since the last crawler run.</p>
    ///   - [`lineage_configuration(LineageConfiguration)`](crate::client::fluent_builders::UpdateCrawler::lineage_configuration) / [`set_lineage_configuration(Option<LineageConfiguration>)`](crate::client::fluent_builders::UpdateCrawler::set_lineage_configuration): <p>Specifies data lineage configuration settings for the crawler.</p>
    ///   - [`lake_formation_configuration(LakeFormationConfiguration)`](crate::client::fluent_builders::UpdateCrawler::lake_formation_configuration) / [`set_lake_formation_configuration(Option<LakeFormationConfiguration>)`](crate::client::fluent_builders::UpdateCrawler::set_lake_formation_configuration): <p>Specifies Lake Formation configuration settings for the crawler.</p>
    ///   - [`configuration(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::configuration) / [`set_configuration(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_configuration): <p>Crawler configuration information. This versioned JSON string allows users to specify aspects of a crawler's behavior. For more information, see <a href="https://docs.aws.amazon.com/glue/latest/dg/crawler-configuration.html">Setting crawler configuration options</a>.</p>
    ///   - [`crawler_security_configuration(impl Into<String>)`](crate::client::fluent_builders::UpdateCrawler::crawler_security_configuration) / [`set_crawler_security_configuration(Option<String>)`](crate::client::fluent_builders::UpdateCrawler::set_crawler_security_configuration): <p>The name of the <code>SecurityConfiguration</code> structure to be used by this crawler.</p>
                            /// - On success, responds with [`UpdateCrawlerOutput`](crate::output::UpdateCrawlerOutput)
                            /// - On failure, responds with [`SdkError<UpdateCrawlerError>`](crate::error::UpdateCrawlerError)
    pub fn update_crawler(&self) -> crate::client::fluent_builders::UpdateCrawler {
                                crate::client::fluent_builders::UpdateCrawler::new(self.handle.clone())
                            }
}

