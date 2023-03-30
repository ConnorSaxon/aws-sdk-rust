// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpgradeElasticsearchDomain`](crate::client::fluent_builders::UpgradeElasticsearchDomain) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::set_domain_name): <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    ///   - [`target_version(impl Into<String>)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::target_version) / [`set_target_version(Option<String>)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::set_target_version): <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    ///   - [`perform_check_only(bool)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::perform_check_only) / [`set_perform_check_only(Option<bool>)`](crate::client::fluent_builders::UpgradeElasticsearchDomain::set_perform_check_only): <p> This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade. </p>
                            /// - On success, responds with [`UpgradeElasticsearchDomainOutput`](crate::output::UpgradeElasticsearchDomainOutput) with field(s):
    ///   - [`domain_name(Option<String>)`](crate::output::UpgradeElasticsearchDomainOutput::domain_name): <p>The name of an Elasticsearch domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    ///   - [`target_version(Option<String>)`](crate::output::UpgradeElasticsearchDomainOutput::target_version): <p>The version of Elasticsearch that you intend to upgrade the domain to.</p>
    ///   - [`perform_check_only(Option<bool>)`](crate::output::UpgradeElasticsearchDomainOutput::perform_check_only): <p> This flag, when set to True, indicates that an Upgrade Eligibility Check needs to be performed. This will not actually perform the Upgrade. </p>
    ///   - [`change_progress_details(Option<ChangeProgressDetails>)`](crate::output::UpgradeElasticsearchDomainOutput::change_progress_details): <p>Specifies change details of the domain configuration change.</p>
                            /// - On failure, responds with [`SdkError<UpgradeElasticsearchDomainError>`](crate::error::UpgradeElasticsearchDomainError)
    pub fn upgrade_elasticsearch_domain(&self) -> crate::client::fluent_builders::UpgradeElasticsearchDomain {
                                crate::client::fluent_builders::UpgradeElasticsearchDomain::new(self.handle.clone())
                            }
}

