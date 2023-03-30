// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateApplication`](crate::client::fluent_builders::CreateApplication) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`author(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::author) / [`set_author(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_author): <p>The name of the author publishing the app.</p> <p>Minimum length=1. Maximum length=127.</p> <p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_description): <p>The description of the application.</p> <p>Minimum length=1. Maximum length=256</p>
    ///   - [`home_page_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::home_page_url) / [`set_home_page_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_home_page_url): <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    ///   - [`labels(Vec<String>)`](crate::client::fluent_builders::CreateApplication::labels) / [`set_labels(Option<Vec<String>>)`](crate::client::fluent_builders::CreateApplication::set_labels): <p>Labels to improve discovery of apps in search results.</p> <p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p> <p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    ///   - [`license_body(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::license_body) / [`set_license_body(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_license_body): <p>A local text file that contains the license of the app that matches the spdxLicenseID value of your application. The file has the format file://&lt;path&gt;/&lt;filename&gt;.</p> <p>Maximum size 5 MB</p> <p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    ///   - [`license_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::license_url) / [`set_license_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_license_url): <p>A link to the S3 object that contains the license of the app that matches the spdxLicenseID value of your application.</p> <p>Maximum size 5 MB</p> <p>You can specify only one of licenseBody and licenseUrl; otherwise, an error results.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_name): <p>The name of the application that you want to publish.</p> <p>Minimum length=1. Maximum length=140</p> <p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    ///   - [`readme_body(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::readme_body) / [`set_readme_body(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_readme_body): <p>A local text readme file in Markdown language that contains a more detailed description of the application and how it works. The file has the format file://&lt;path&gt;/&lt;filename&gt;.</p> <p>Maximum size 5 MB</p> <p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    ///   - [`readme_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::readme_url) / [`set_readme_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_readme_url): <p>A link to the S3 object in Markdown language that contains a more detailed description of the application and how it works.</p> <p>Maximum size 5 MB</p> <p>You can specify only one of readmeBody and readmeUrl; otherwise, an error results.</p>
    ///   - [`semantic_version(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::semantic_version) / [`set_semantic_version(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_semantic_version): <p>The semantic version of the application:</p> <p> <a href="https://semver.org/">https://semver.org/</a> </p>
    ///   - [`source_code_archive_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::source_code_archive_url) / [`set_source_code_archive_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_source_code_archive_url): <p>A link to the S3 object that contains the ZIP archive of the source code for this version of your application.</p> <p>Maximum size 50 MB</p>
    ///   - [`source_code_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::source_code_url) / [`set_source_code_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_source_code_url): <p>A link to a public repository for the source code of your application, for example the URL of a specific GitHub commit.</p>
    ///   - [`spdx_license_id(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::spdx_license_id) / [`set_spdx_license_id(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_spdx_license_id): <p>A valid identifier from <a href="https://spdx.org/licenses/">https://spdx.org/licenses/</a>.</p>
    ///   - [`template_body(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::template_body) / [`set_template_body(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_template_body): <p>The local raw packaged AWS SAM template file of your application. The file has the format file://&lt;path&gt;/&lt;filename&gt;.</p> <p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
    ///   - [`template_url(impl Into<String>)`](crate::client::fluent_builders::CreateApplication::template_url) / [`set_template_url(Option<String>)`](crate::client::fluent_builders::CreateApplication::set_template_url): <p>A link to the S3 object containing the packaged AWS SAM template of your application.</p> <p>You can specify only one of templateBody and templateUrl; otherwise an error results.</p>
                            /// - On success, responds with [`CreateApplicationOutput`](crate::output::CreateApplicationOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::output::CreateApplicationOutput::application_id): <p>The application Amazon Resource Name (ARN).</p>
    ///   - [`author(Option<String>)`](crate::output::CreateApplicationOutput::author): <p>The name of the author publishing the app.</p> <p>Minimum length=1. Maximum length=127.</p> <p>Pattern "^[a-z0-9](([a-z0-9]|-(?!-))*[a-z0-9])?$";</p>
    ///   - [`creation_time(Option<String>)`](crate::output::CreateApplicationOutput::creation_time): <p>The date and time this resource was created.</p>
    ///   - [`description(Option<String>)`](crate::output::CreateApplicationOutput::description): <p>The description of the application.</p> <p>Minimum length=1. Maximum length=256</p>
    ///   - [`home_page_url(Option<String>)`](crate::output::CreateApplicationOutput::home_page_url): <p>A URL with more information about the application, for example the location of your GitHub repository for the application.</p>
    ///   - [`is_verified_author(bool)`](crate::output::CreateApplicationOutput::is_verified_author): <p>Whether the author of this application has been verified. This means means that AWS has made a good faith review, as a reasonable and prudent service provider, of the information provided by the requester and has confirmed that the requester's identity is as claimed.</p>
    ///   - [`labels(Option<Vec<String>>)`](crate::output::CreateApplicationOutput::labels): <p>Labels to improve discovery of apps in search results.</p> <p>Minimum length=1. Maximum length=127. Maximum number of labels: 10</p> <p>Pattern: "^[a-zA-Z0-9+\\-_:\\/@]+$";</p>
    ///   - [`license_url(Option<String>)`](crate::output::CreateApplicationOutput::license_url): <p>A link to a license file of the app that matches the spdxLicenseID value of your application.</p> <p>Maximum size 5 MB</p>
    ///   - [`name(Option<String>)`](crate::output::CreateApplicationOutput::name): <p>The name of the application.</p> <p>Minimum length=1. Maximum length=140</p> <p>Pattern: "[a-zA-Z0-9\\-]+";</p>
    ///   - [`readme_url(Option<String>)`](crate::output::CreateApplicationOutput::readme_url): <p>A link to the readme file in Markdown language that contains a more detailed description of the application and how it works.</p> <p>Maximum size 5 MB</p>
    ///   - [`spdx_license_id(Option<String>)`](crate::output::CreateApplicationOutput::spdx_license_id): <p>A valid identifier from https://spdx.org/licenses/.</p>
    ///   - [`verified_author_url(Option<String>)`](crate::output::CreateApplicationOutput::verified_author_url): <p>The URL to the public profile of a verified author. This URL is submitted by the author.</p>
    ///   - [`version(Option<Version>)`](crate::output::CreateApplicationOutput::version): <p>Version information about the application.</p>
                            /// - On failure, responds with [`SdkError<CreateApplicationError>`](crate::error::CreateApplicationError)
    pub fn create_application(&self) -> crate::client::fluent_builders::CreateApplication {
                                crate::client::fluent_builders::CreateApplication::new(self.handle.clone())
                            }
}

