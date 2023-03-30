// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetApplicationComponentDetails`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_application_component_details`](crate::client::fluent_builders::GetApplicationComponentDetails).
            ///
            /// `ParseStrictResponse` impl for `GetApplicationComponentDetails`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetApplicationComponentDetails {
    _private: ()
}
impl GetApplicationComponentDetails {
    /// Creates a new builder-style object to manufacture [`GetApplicationComponentDetailsInput`](crate::input::GetApplicationComponentDetailsInput).
    pub fn builder() -> crate::input::get_application_component_details_input::Builder {
        crate::input::get_application_component_details_input::Builder::default()
    }
    /// Creates a new `GetApplicationComponentDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetApplicationComponentDetails {
                type Output = std::result::Result<crate::output::GetApplicationComponentDetailsOutput, crate::error::GetApplicationComponentDetailsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_application_component_details::de_get_application_component_details_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_application_component_details::de_get_application_component_details_http_response(response)
                     }
                }
            }

/// Operation shape for `GetApplicationComponentStrategies`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_application_component_strategies`](crate::client::fluent_builders::GetApplicationComponentStrategies).
            ///
            /// `ParseStrictResponse` impl for `GetApplicationComponentStrategies`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetApplicationComponentStrategies {
    _private: ()
}
impl GetApplicationComponentStrategies {
    /// Creates a new builder-style object to manufacture [`GetApplicationComponentStrategiesInput`](crate::input::GetApplicationComponentStrategiesInput).
    pub fn builder() -> crate::input::get_application_component_strategies_input::Builder {
        crate::input::get_application_component_strategies_input::Builder::default()
    }
    /// Creates a new `GetApplicationComponentStrategies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetApplicationComponentStrategies {
                type Output = std::result::Result<crate::output::GetApplicationComponentStrategiesOutput, crate::error::GetApplicationComponentStrategiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_application_component_strategies::de_get_application_component_strategies_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_application_component_strategies::de_get_application_component_strategies_http_response(response)
                     }
                }
            }

/// Operation shape for `GetAssessment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_assessment`](crate::client::fluent_builders::GetAssessment).
            ///
            /// `ParseStrictResponse` impl for `GetAssessment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAssessment {
    _private: ()
}
impl GetAssessment {
    /// Creates a new builder-style object to manufacture [`GetAssessmentInput`](crate::input::GetAssessmentInput).
    pub fn builder() -> crate::input::get_assessment_input::Builder {
        crate::input::get_assessment_input::Builder::default()
    }
    /// Creates a new `GetAssessment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAssessment {
                type Output = std::result::Result<crate::output::GetAssessmentOutput, crate::error::GetAssessmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_assessment::de_get_assessment_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_assessment::de_get_assessment_http_response(response)
                     }
                }
            }

/// Operation shape for `GetImportFileTask`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_import_file_task`](crate::client::fluent_builders::GetImportFileTask).
            ///
            /// `ParseStrictResponse` impl for `GetImportFileTask`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetImportFileTask {
    _private: ()
}
impl GetImportFileTask {
    /// Creates a new builder-style object to manufacture [`GetImportFileTaskInput`](crate::input::GetImportFileTaskInput).
    pub fn builder() -> crate::input::get_import_file_task_input::Builder {
        crate::input::get_import_file_task_input::Builder::default()
    }
    /// Creates a new `GetImportFileTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetImportFileTask {
                type Output = std::result::Result<crate::output::GetImportFileTaskOutput, crate::error::GetImportFileTaskError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_import_file_task::de_get_import_file_task_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_import_file_task::de_get_import_file_task_http_response(response)
                     }
                }
            }

/// Operation shape for `GetLatestAssessmentId`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_latest_assessment_id`](crate::client::fluent_builders::GetLatestAssessmentId).
            ///
            /// `ParseStrictResponse` impl for `GetLatestAssessmentId`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLatestAssessmentId {
    _private: ()
}
impl GetLatestAssessmentId {
    /// Creates a new builder-style object to manufacture [`GetLatestAssessmentIdInput`](crate::input::GetLatestAssessmentIdInput).
    pub fn builder() -> crate::input::get_latest_assessment_id_input::Builder {
        crate::input::get_latest_assessment_id_input::Builder::default()
    }
    /// Creates a new `GetLatestAssessmentId` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLatestAssessmentId {
                type Output = std::result::Result<crate::output::GetLatestAssessmentIdOutput, crate::error::GetLatestAssessmentIdError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_latest_assessment_id::de_get_latest_assessment_id_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_latest_assessment_id::de_get_latest_assessment_id_http_response(response)
                     }
                }
            }

/// Operation shape for `GetPortfolioPreferences`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_portfolio_preferences`](crate::client::fluent_builders::GetPortfolioPreferences).
            ///
            /// `ParseStrictResponse` impl for `GetPortfolioPreferences`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPortfolioPreferences {
    _private: ()
}
impl GetPortfolioPreferences {
    /// Creates a new builder-style object to manufacture [`GetPortfolioPreferencesInput`](crate::input::GetPortfolioPreferencesInput).
    pub fn builder() -> crate::input::get_portfolio_preferences_input::Builder {
        crate::input::get_portfolio_preferences_input::Builder::default()
    }
    /// Creates a new `GetPortfolioPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPortfolioPreferences {
                type Output = std::result::Result<crate::output::GetPortfolioPreferencesOutput, crate::error::GetPortfolioPreferencesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_portfolio_preferences::de_get_portfolio_preferences_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_portfolio_preferences::de_get_portfolio_preferences_http_response(response)
                     }
                }
            }

/// Operation shape for `GetPortfolioSummary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_portfolio_summary`](crate::client::fluent_builders::GetPortfolioSummary).
            ///
            /// `ParseStrictResponse` impl for `GetPortfolioSummary`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPortfolioSummary {
    _private: ()
}
impl GetPortfolioSummary {
    /// Creates a new builder-style object to manufacture [`GetPortfolioSummaryInput`](crate::input::GetPortfolioSummaryInput).
    pub fn builder() -> crate::input::get_portfolio_summary_input::Builder {
        crate::input::get_portfolio_summary_input::Builder::default()
    }
    /// Creates a new `GetPortfolioSummary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPortfolioSummary {
                type Output = std::result::Result<crate::output::GetPortfolioSummaryOutput, crate::error::GetPortfolioSummaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_portfolio_summary::de_get_portfolio_summary_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_portfolio_summary::de_get_portfolio_summary_http_response(response)
                     }
                }
            }

/// Operation shape for `GetRecommendationReportDetails`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_recommendation_report_details`](crate::client::fluent_builders::GetRecommendationReportDetails).
            ///
            /// `ParseStrictResponse` impl for `GetRecommendationReportDetails`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecommendationReportDetails {
    _private: ()
}
impl GetRecommendationReportDetails {
    /// Creates a new builder-style object to manufacture [`GetRecommendationReportDetailsInput`](crate::input::GetRecommendationReportDetailsInput).
    pub fn builder() -> crate::input::get_recommendation_report_details_input::Builder {
        crate::input::get_recommendation_report_details_input::Builder::default()
    }
    /// Creates a new `GetRecommendationReportDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecommendationReportDetails {
                type Output = std::result::Result<crate::output::GetRecommendationReportDetailsOutput, crate::error::GetRecommendationReportDetailsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_recommendation_report_details::de_get_recommendation_report_details_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_recommendation_report_details::de_get_recommendation_report_details_http_response(response)
                     }
                }
            }

/// Operation shape for `GetServerDetails`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_server_details`](crate::client::fluent_builders::GetServerDetails).
            ///
            /// `ParseStrictResponse` impl for `GetServerDetails`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServerDetails {
    _private: ()
}
impl GetServerDetails {
    /// Creates a new builder-style object to manufacture [`GetServerDetailsInput`](crate::input::GetServerDetailsInput).
    pub fn builder() -> crate::input::get_server_details_input::Builder {
        crate::input::get_server_details_input::Builder::default()
    }
    /// Creates a new `GetServerDetails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServerDetails {
                type Output = std::result::Result<crate::output::GetServerDetailsOutput, crate::error::GetServerDetailsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_server_details::de_get_server_details_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_server_details::de_get_server_details_http_response(response)
                     }
                }
            }

/// Operation shape for `GetServerStrategies`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_server_strategies`](crate::client::fluent_builders::GetServerStrategies).
            ///
            /// `ParseStrictResponse` impl for `GetServerStrategies`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServerStrategies {
    _private: ()
}
impl GetServerStrategies {
    /// Creates a new builder-style object to manufacture [`GetServerStrategiesInput`](crate::input::GetServerStrategiesInput).
    pub fn builder() -> crate::input::get_server_strategies_input::Builder {
        crate::input::get_server_strategies_input::Builder::default()
    }
    /// Creates a new `GetServerStrategies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServerStrategies {
                type Output = std::result::Result<crate::output::GetServerStrategiesOutput, crate::error::GetServerStrategiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_server_strategies::de_get_server_strategies_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_server_strategies::de_get_server_strategies_http_response(response)
                     }
                }
            }

/// Operation shape for `ListApplicationComponents`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_application_components`](crate::client::fluent_builders::ListApplicationComponents).
            ///
            /// `ParseStrictResponse` impl for `ListApplicationComponents`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListApplicationComponents {
    _private: ()
}
impl ListApplicationComponents {
    /// Creates a new builder-style object to manufacture [`ListApplicationComponentsInput`](crate::input::ListApplicationComponentsInput).
    pub fn builder() -> crate::input::list_application_components_input::Builder {
        crate::input::list_application_components_input::Builder::default()
    }
    /// Creates a new `ListApplicationComponents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplicationComponents {
                type Output = std::result::Result<crate::output::ListApplicationComponentsOutput, crate::error::ListApplicationComponentsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_application_components::de_list_application_components_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_application_components::de_list_application_components_http_response(response)
                     }
                }
            }

/// Operation shape for `ListCollectors`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_collectors`](crate::client::fluent_builders::ListCollectors).
            ///
            /// `ParseStrictResponse` impl for `ListCollectors`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCollectors {
    _private: ()
}
impl ListCollectors {
    /// Creates a new builder-style object to manufacture [`ListCollectorsInput`](crate::input::ListCollectorsInput).
    pub fn builder() -> crate::input::list_collectors_input::Builder {
        crate::input::list_collectors_input::Builder::default()
    }
    /// Creates a new `ListCollectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCollectors {
                type Output = std::result::Result<crate::output::ListCollectorsOutput, crate::error::ListCollectorsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_collectors::de_list_collectors_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_collectors::de_list_collectors_http_response(response)
                     }
                }
            }

/// Operation shape for `ListImportFileTask`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_import_file_task`](crate::client::fluent_builders::ListImportFileTask).
            ///
            /// `ParseStrictResponse` impl for `ListImportFileTask`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListImportFileTask {
    _private: ()
}
impl ListImportFileTask {
    /// Creates a new builder-style object to manufacture [`ListImportFileTaskInput`](crate::input::ListImportFileTaskInput).
    pub fn builder() -> crate::input::list_import_file_task_input::Builder {
        crate::input::list_import_file_task_input::Builder::default()
    }
    /// Creates a new `ListImportFileTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListImportFileTask {
                type Output = std::result::Result<crate::output::ListImportFileTaskOutput, crate::error::ListImportFileTaskError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_import_file_task::de_list_import_file_task_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_import_file_task::de_list_import_file_task_http_response(response)
                     }
                }
            }

/// Operation shape for `ListServers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_servers`](crate::client::fluent_builders::ListServers).
            ///
            /// `ParseStrictResponse` impl for `ListServers`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListServers {
    _private: ()
}
impl ListServers {
    /// Creates a new builder-style object to manufacture [`ListServersInput`](crate::input::ListServersInput).
    pub fn builder() -> crate::input::list_servers_input::Builder {
        crate::input::list_servers_input::Builder::default()
    }
    /// Creates a new `ListServers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServers {
                type Output = std::result::Result<crate::output::ListServersOutput, crate::error::ListServersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_servers::de_list_servers_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_servers::de_list_servers_http_response(response)
                     }
                }
            }

/// Operation shape for `PutPortfolioPreferences`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_portfolio_preferences`](crate::client::fluent_builders::PutPortfolioPreferences).
            ///
            /// `ParseStrictResponse` impl for `PutPortfolioPreferences`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutPortfolioPreferences {
    _private: ()
}
impl PutPortfolioPreferences {
    /// Creates a new builder-style object to manufacture [`PutPortfolioPreferencesInput`](crate::input::PutPortfolioPreferencesInput).
    pub fn builder() -> crate::input::put_portfolio_preferences_input::Builder {
        crate::input::put_portfolio_preferences_input::Builder::default()
    }
    /// Creates a new `PutPortfolioPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutPortfolioPreferences {
                type Output = std::result::Result<crate::output::PutPortfolioPreferencesOutput, crate::error::PutPortfolioPreferencesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_put_portfolio_preferences::de_put_portfolio_preferences_http_error(response)
                     } else {
                        crate::protocol_serde::shape_put_portfolio_preferences::de_put_portfolio_preferences_http_response(response)
                     }
                }
            }

/// Operation shape for `StartAssessment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_assessment`](crate::client::fluent_builders::StartAssessment).
            ///
            /// `ParseStrictResponse` impl for `StartAssessment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartAssessment {
    _private: ()
}
impl StartAssessment {
    /// Creates a new builder-style object to manufacture [`StartAssessmentInput`](crate::input::StartAssessmentInput).
    pub fn builder() -> crate::input::start_assessment_input::Builder {
        crate::input::start_assessment_input::Builder::default()
    }
    /// Creates a new `StartAssessment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartAssessment {
                type Output = std::result::Result<crate::output::StartAssessmentOutput, crate::error::StartAssessmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_assessment::de_start_assessment_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_assessment::de_start_assessment_http_response(response)
                     }
                }
            }

/// Operation shape for `StartImportFileTask`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_import_file_task`](crate::client::fluent_builders::StartImportFileTask).
            ///
            /// `ParseStrictResponse` impl for `StartImportFileTask`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartImportFileTask {
    _private: ()
}
impl StartImportFileTask {
    /// Creates a new builder-style object to manufacture [`StartImportFileTaskInput`](crate::input::StartImportFileTaskInput).
    pub fn builder() -> crate::input::start_import_file_task_input::Builder {
        crate::input::start_import_file_task_input::Builder::default()
    }
    /// Creates a new `StartImportFileTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartImportFileTask {
                type Output = std::result::Result<crate::output::StartImportFileTaskOutput, crate::error::StartImportFileTaskError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_import_file_task::de_start_import_file_task_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_import_file_task::de_start_import_file_task_http_response(response)
                     }
                }
            }

/// Operation shape for `StartRecommendationReportGeneration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_recommendation_report_generation`](crate::client::fluent_builders::StartRecommendationReportGeneration).
            ///
            /// `ParseStrictResponse` impl for `StartRecommendationReportGeneration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartRecommendationReportGeneration {
    _private: ()
}
impl StartRecommendationReportGeneration {
    /// Creates a new builder-style object to manufacture [`StartRecommendationReportGenerationInput`](crate::input::StartRecommendationReportGenerationInput).
    pub fn builder() -> crate::input::start_recommendation_report_generation_input::Builder {
        crate::input::start_recommendation_report_generation_input::Builder::default()
    }
    /// Creates a new `StartRecommendationReportGeneration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartRecommendationReportGeneration {
                type Output = std::result::Result<crate::output::StartRecommendationReportGenerationOutput, crate::error::StartRecommendationReportGenerationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_recommendation_report_generation::de_start_recommendation_report_generation_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_recommendation_report_generation::de_start_recommendation_report_generation_http_response(response)
                     }
                }
            }

/// Operation shape for `StopAssessment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_assessment`](crate::client::fluent_builders::StopAssessment).
            ///
            /// `ParseStrictResponse` impl for `StopAssessment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopAssessment {
    _private: ()
}
impl StopAssessment {
    /// Creates a new builder-style object to manufacture [`StopAssessmentInput`](crate::input::StopAssessmentInput).
    pub fn builder() -> crate::input::stop_assessment_input::Builder {
        crate::input::stop_assessment_input::Builder::default()
    }
    /// Creates a new `StopAssessment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopAssessment {
                type Output = std::result::Result<crate::output::StopAssessmentOutput, crate::error::StopAssessmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_stop_assessment::de_stop_assessment_http_error(response)
                     } else {
                        crate::protocol_serde::shape_stop_assessment::de_stop_assessment_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateApplicationComponentConfig`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_application_component_config`](crate::client::fluent_builders::UpdateApplicationComponentConfig).
            ///
            /// `ParseStrictResponse` impl for `UpdateApplicationComponentConfig`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateApplicationComponentConfig {
    _private: ()
}
impl UpdateApplicationComponentConfig {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationComponentConfigInput`](crate::input::UpdateApplicationComponentConfigInput).
    pub fn builder() -> crate::input::update_application_component_config_input::Builder {
        crate::input::update_application_component_config_input::Builder::default()
    }
    /// Creates a new `UpdateApplicationComponentConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateApplicationComponentConfig {
                type Output = std::result::Result<crate::output::UpdateApplicationComponentConfigOutput, crate::error::UpdateApplicationComponentConfigError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_application_component_config::de_update_application_component_config_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_application_component_config::de_update_application_component_config_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateServerConfig`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_server_config`](crate::client::fluent_builders::UpdateServerConfig).
            ///
            /// `ParseStrictResponse` impl for `UpdateServerConfig`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateServerConfig {
    _private: ()
}
impl UpdateServerConfig {
    /// Creates a new builder-style object to manufacture [`UpdateServerConfigInput`](crate::input::UpdateServerConfigInput).
    pub fn builder() -> crate::input::update_server_config_input::Builder {
        crate::input::update_server_config_input::Builder::default()
    }
    /// Creates a new `UpdateServerConfig` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateServerConfig {
                type Output = std::result::Result<crate::output::UpdateServerConfigOutput, crate::error::UpdateServerConfigError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_server_config::de_update_server_config_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_server_config::de_update_server_config_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

