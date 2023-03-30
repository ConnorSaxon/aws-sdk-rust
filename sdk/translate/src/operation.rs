// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateParallelData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_parallel_data`](crate::client::fluent_builders::CreateParallelData).
            ///
            /// `ParseStrictResponse` impl for `CreateParallelData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateParallelData {
    _private: ()
}
impl CreateParallelData {
    /// Creates a new builder-style object to manufacture [`CreateParallelDataInput`](crate::input::CreateParallelDataInput).
    pub fn builder() -> crate::input::create_parallel_data_input::Builder {
        crate::input::create_parallel_data_input::Builder::default()
    }
    /// Creates a new `CreateParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateParallelData {
                type Output = std::result::Result<crate::output::CreateParallelDataOutput, crate::error::CreateParallelDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_parallel_data::de_create_parallel_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_parallel_data::de_create_parallel_data_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteParallelData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_parallel_data`](crate::client::fluent_builders::DeleteParallelData).
            ///
            /// `ParseStrictResponse` impl for `DeleteParallelData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteParallelData {
    _private: ()
}
impl DeleteParallelData {
    /// Creates a new builder-style object to manufacture [`DeleteParallelDataInput`](crate::input::DeleteParallelDataInput).
    pub fn builder() -> crate::input::delete_parallel_data_input::Builder {
        crate::input::delete_parallel_data_input::Builder::default()
    }
    /// Creates a new `DeleteParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteParallelData {
                type Output = std::result::Result<crate::output::DeleteParallelDataOutput, crate::error::DeleteParallelDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_parallel_data::de_delete_parallel_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_parallel_data::de_delete_parallel_data_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteTerminology`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_terminology`](crate::client::fluent_builders::DeleteTerminology).
            ///
            /// `ParseStrictResponse` impl for `DeleteTerminology`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteTerminology {
    _private: ()
}
impl DeleteTerminology {
    /// Creates a new builder-style object to manufacture [`DeleteTerminologyInput`](crate::input::DeleteTerminologyInput).
    pub fn builder() -> crate::input::delete_terminology_input::Builder {
        crate::input::delete_terminology_input::Builder::default()
    }
    /// Creates a new `DeleteTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTerminology {
                type Output = std::result::Result<crate::output::DeleteTerminologyOutput, crate::error::DeleteTerminologyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_terminology::de_delete_terminology_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_terminology::de_delete_terminology_http_response(response)
                     }
                }
            }

/// Operation shape for `DescribeTextTranslationJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_text_translation_job`](crate::client::fluent_builders::DescribeTextTranslationJob).
            ///
            /// `ParseStrictResponse` impl for `DescribeTextTranslationJob`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTextTranslationJob {
    _private: ()
}
impl DescribeTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`DescribeTextTranslationJobInput`](crate::input::DescribeTextTranslationJobInput).
    pub fn builder() -> crate::input::describe_text_translation_job_input::Builder {
        crate::input::describe_text_translation_job_input::Builder::default()
    }
    /// Creates a new `DescribeTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTextTranslationJob {
                type Output = std::result::Result<crate::output::DescribeTextTranslationJobOutput, crate::error::DescribeTextTranslationJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_describe_text_translation_job::de_describe_text_translation_job_http_error(response)
                     } else {
                        crate::protocol_serde::shape_describe_text_translation_job::de_describe_text_translation_job_http_response(response)
                     }
                }
            }

/// Operation shape for `GetParallelData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_parallel_data`](crate::client::fluent_builders::GetParallelData).
            ///
            /// `ParseStrictResponse` impl for `GetParallelData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetParallelData {
    _private: ()
}
impl GetParallelData {
    /// Creates a new builder-style object to manufacture [`GetParallelDataInput`](crate::input::GetParallelDataInput).
    pub fn builder() -> crate::input::get_parallel_data_input::Builder {
        crate::input::get_parallel_data_input::Builder::default()
    }
    /// Creates a new `GetParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetParallelData {
                type Output = std::result::Result<crate::output::GetParallelDataOutput, crate::error::GetParallelDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_parallel_data::de_get_parallel_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_parallel_data::de_get_parallel_data_http_response(response)
                     }
                }
            }

/// Operation shape for `GetTerminology`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_terminology`](crate::client::fluent_builders::GetTerminology).
            ///
            /// `ParseStrictResponse` impl for `GetTerminology`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTerminology {
    _private: ()
}
impl GetTerminology {
    /// Creates a new builder-style object to manufacture [`GetTerminologyInput`](crate::input::GetTerminologyInput).
    pub fn builder() -> crate::input::get_terminology_input::Builder {
        crate::input::get_terminology_input::Builder::default()
    }
    /// Creates a new `GetTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTerminology {
                type Output = std::result::Result<crate::output::GetTerminologyOutput, crate::error::GetTerminologyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_terminology::de_get_terminology_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_terminology::de_get_terminology_http_response(response)
                     }
                }
            }

/// Operation shape for `ImportTerminology`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`import_terminology`](crate::client::fluent_builders::ImportTerminology).
            ///
            /// `ParseStrictResponse` impl for `ImportTerminology`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportTerminology {
    _private: ()
}
impl ImportTerminology {
    /// Creates a new builder-style object to manufacture [`ImportTerminologyInput`](crate::input::ImportTerminologyInput).
    pub fn builder() -> crate::input::import_terminology_input::Builder {
        crate::input::import_terminology_input::Builder::default()
    }
    /// Creates a new `ImportTerminology` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportTerminology {
                type Output = std::result::Result<crate::output::ImportTerminologyOutput, crate::error::ImportTerminologyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_import_terminology::de_import_terminology_http_error(response)
                     } else {
                        crate::protocol_serde::shape_import_terminology::de_import_terminology_http_response(response)
                     }
                }
            }

/// Operation shape for `ListLanguages`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_languages`](crate::client::fluent_builders::ListLanguages).
            ///
            /// `ParseStrictResponse` impl for `ListLanguages`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLanguages {
    _private: ()
}
impl ListLanguages {
    /// Creates a new builder-style object to manufacture [`ListLanguagesInput`](crate::input::ListLanguagesInput).
    pub fn builder() -> crate::input::list_languages_input::Builder {
        crate::input::list_languages_input::Builder::default()
    }
    /// Creates a new `ListLanguages` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLanguages {
                type Output = std::result::Result<crate::output::ListLanguagesOutput, crate::error::ListLanguagesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_languages::de_list_languages_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_languages::de_list_languages_http_response(response)
                     }
                }
            }

/// Operation shape for `ListParallelData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_parallel_data`](crate::client::fluent_builders::ListParallelData).
            ///
            /// `ParseStrictResponse` impl for `ListParallelData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListParallelData {
    _private: ()
}
impl ListParallelData {
    /// Creates a new builder-style object to manufacture [`ListParallelDataInput`](crate::input::ListParallelDataInput).
    pub fn builder() -> crate::input::list_parallel_data_input::Builder {
        crate::input::list_parallel_data_input::Builder::default()
    }
    /// Creates a new `ListParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListParallelData {
                type Output = std::result::Result<crate::output::ListParallelDataOutput, crate::error::ListParallelDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_parallel_data::de_list_parallel_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_parallel_data::de_list_parallel_data_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::fluent_builders::ListTagsForResource).
            ///
            /// `ParseStrictResponse` impl for `ListTagsForResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTerminologies`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_terminologies`](crate::client::fluent_builders::ListTerminologies).
            ///
            /// `ParseStrictResponse` impl for `ListTerminologies`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTerminologies {
    _private: ()
}
impl ListTerminologies {
    /// Creates a new builder-style object to manufacture [`ListTerminologiesInput`](crate::input::ListTerminologiesInput).
    pub fn builder() -> crate::input::list_terminologies_input::Builder {
        crate::input::list_terminologies_input::Builder::default()
    }
    /// Creates a new `ListTerminologies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTerminologies {
                type Output = std::result::Result<crate::output::ListTerminologiesOutput, crate::error::ListTerminologiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_terminologies::de_list_terminologies_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_terminologies::de_list_terminologies_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTextTranslationJobs`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_text_translation_jobs`](crate::client::fluent_builders::ListTextTranslationJobs).
            ///
            /// `ParseStrictResponse` impl for `ListTextTranslationJobs`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTextTranslationJobs {
    _private: ()
}
impl ListTextTranslationJobs {
    /// Creates a new builder-style object to manufacture [`ListTextTranslationJobsInput`](crate::input::ListTextTranslationJobsInput).
    pub fn builder() -> crate::input::list_text_translation_jobs_input::Builder {
        crate::input::list_text_translation_jobs_input::Builder::default()
    }
    /// Creates a new `ListTextTranslationJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTextTranslationJobs {
                type Output = std::result::Result<crate::output::ListTextTranslationJobsOutput, crate::error::ListTextTranslationJobsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_text_translation_jobs::de_list_text_translation_jobs_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_text_translation_jobs::de_list_text_translation_jobs_http_response(response)
                     }
                }
            }

/// Operation shape for `StartTextTranslationJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_text_translation_job`](crate::client::fluent_builders::StartTextTranslationJob).
            ///
            /// `ParseStrictResponse` impl for `StartTextTranslationJob`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartTextTranslationJob {
    _private: ()
}
impl StartTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`StartTextTranslationJobInput`](crate::input::StartTextTranslationJobInput).
    pub fn builder() -> crate::input::start_text_translation_job_input::Builder {
        crate::input::start_text_translation_job_input::Builder::default()
    }
    /// Creates a new `StartTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartTextTranslationJob {
                type Output = std::result::Result<crate::output::StartTextTranslationJobOutput, crate::error::StartTextTranslationJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_text_translation_job::de_start_text_translation_job_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_text_translation_job::de_start_text_translation_job_http_response(response)
                     }
                }
            }

/// Operation shape for `StopTextTranslationJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_text_translation_job`](crate::client::fluent_builders::StopTextTranslationJob).
            ///
            /// `ParseStrictResponse` impl for `StopTextTranslationJob`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopTextTranslationJob {
    _private: ()
}
impl StopTextTranslationJob {
    /// Creates a new builder-style object to manufacture [`StopTextTranslationJobInput`](crate::input::StopTextTranslationJobInput).
    pub fn builder() -> crate::input::stop_text_translation_job_input::Builder {
        crate::input::stop_text_translation_job_input::Builder::default()
    }
    /// Creates a new `StopTextTranslationJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopTextTranslationJob {
                type Output = std::result::Result<crate::output::StopTextTranslationJobOutput, crate::error::StopTextTranslationJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_stop_text_translation_job::de_stop_text_translation_job_http_error(response)
                     } else {
                        crate::protocol_serde::shape_stop_text_translation_job::de_stop_text_translation_job_http_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::fluent_builders::TagResource).
            ///
            /// `ParseStrictResponse` impl for `TagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_tag_resource::de_tag_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_tag_resource::de_tag_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `TranslateText`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`translate_text`](crate::client::fluent_builders::TranslateText).
            ///
            /// `ParseStrictResponse` impl for `TranslateText`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TranslateText {
    _private: ()
}
impl TranslateText {
    /// Creates a new builder-style object to manufacture [`TranslateTextInput`](crate::input::TranslateTextInput).
    pub fn builder() -> crate::input::translate_text_input::Builder {
        crate::input::translate_text_input::Builder::default()
    }
    /// Creates a new `TranslateText` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TranslateText {
                type Output = std::result::Result<crate::output::TranslateTextOutput, crate::error::TranslateTextError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_translate_text::de_translate_text_http_error(response)
                     } else {
                        crate::protocol_serde::shape_translate_text::de_translate_text_http_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::fluent_builders::UntagResource).
            ///
            /// `ParseStrictResponse` impl for `UntagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_untag_resource::de_untag_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_untag_resource::de_untag_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateParallelData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_parallel_data`](crate::client::fluent_builders::UpdateParallelData).
            ///
            /// `ParseStrictResponse` impl for `UpdateParallelData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateParallelData {
    _private: ()
}
impl UpdateParallelData {
    /// Creates a new builder-style object to manufacture [`UpdateParallelDataInput`](crate::input::UpdateParallelDataInput).
    pub fn builder() -> crate::input::update_parallel_data_input::Builder {
        crate::input::update_parallel_data_input::Builder::default()
    }
    /// Creates a new `UpdateParallelData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateParallelData {
                type Output = std::result::Result<crate::output::UpdateParallelDataOutput, crate::error::UpdateParallelDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_parallel_data::de_update_parallel_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_parallel_data::de_update_parallel_data_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

