// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchCreateTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_create_table_rows`](crate::client::fluent_builders::BatchCreateTableRows).
            ///
            /// `ParseStrictResponse` impl for `BatchCreateTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchCreateTableRows {
    _private: ()
}
impl BatchCreateTableRows {
    /// Creates a new builder-style object to manufacture [`BatchCreateTableRowsInput`](crate::input::BatchCreateTableRowsInput).
    pub fn builder() -> crate::input::batch_create_table_rows_input::Builder {
        crate::input::batch_create_table_rows_input::Builder::default()
    }
    /// Creates a new `BatchCreateTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchCreateTableRows {
                type Output = std::result::Result<crate::output::BatchCreateTableRowsOutput, crate::error::BatchCreateTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_create_table_rows::de_batch_create_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_create_table_rows::de_batch_create_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `BatchDeleteTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_delete_table_rows`](crate::client::fluent_builders::BatchDeleteTableRows).
            ///
            /// `ParseStrictResponse` impl for `BatchDeleteTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchDeleteTableRows {
    _private: ()
}
impl BatchDeleteTableRows {
    /// Creates a new builder-style object to manufacture [`BatchDeleteTableRowsInput`](crate::input::BatchDeleteTableRowsInput).
    pub fn builder() -> crate::input::batch_delete_table_rows_input::Builder {
        crate::input::batch_delete_table_rows_input::Builder::default()
    }
    /// Creates a new `BatchDeleteTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchDeleteTableRows {
                type Output = std::result::Result<crate::output::BatchDeleteTableRowsOutput, crate::error::BatchDeleteTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_delete_table_rows::de_batch_delete_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_delete_table_rows::de_batch_delete_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `BatchUpdateTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_update_table_rows`](crate::client::fluent_builders::BatchUpdateTableRows).
            ///
            /// `ParseStrictResponse` impl for `BatchUpdateTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchUpdateTableRows {
    _private: ()
}
impl BatchUpdateTableRows {
    /// Creates a new builder-style object to manufacture [`BatchUpdateTableRowsInput`](crate::input::BatchUpdateTableRowsInput).
    pub fn builder() -> crate::input::batch_update_table_rows_input::Builder {
        crate::input::batch_update_table_rows_input::Builder::default()
    }
    /// Creates a new `BatchUpdateTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchUpdateTableRows {
                type Output = std::result::Result<crate::output::BatchUpdateTableRowsOutput, crate::error::BatchUpdateTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_update_table_rows::de_batch_update_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_update_table_rows::de_batch_update_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `BatchUpsertTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_upsert_table_rows`](crate::client::fluent_builders::BatchUpsertTableRows).
            ///
            /// `ParseStrictResponse` impl for `BatchUpsertTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchUpsertTableRows {
    _private: ()
}
impl BatchUpsertTableRows {
    /// Creates a new builder-style object to manufacture [`BatchUpsertTableRowsInput`](crate::input::BatchUpsertTableRowsInput).
    pub fn builder() -> crate::input::batch_upsert_table_rows_input::Builder {
        crate::input::batch_upsert_table_rows_input::Builder::default()
    }
    /// Creates a new `BatchUpsertTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchUpsertTableRows {
                type Output = std::result::Result<crate::output::BatchUpsertTableRowsOutput, crate::error::BatchUpsertTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_upsert_table_rows::de_batch_upsert_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_upsert_table_rows::de_batch_upsert_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `DescribeTableDataImportJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_table_data_import_job`](crate::client::fluent_builders::DescribeTableDataImportJob).
            ///
            /// `ParseStrictResponse` impl for `DescribeTableDataImportJob`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTableDataImportJob {
    _private: ()
}
impl DescribeTableDataImportJob {
    /// Creates a new builder-style object to manufacture [`DescribeTableDataImportJobInput`](crate::input::DescribeTableDataImportJobInput).
    pub fn builder() -> crate::input::describe_table_data_import_job_input::Builder {
        crate::input::describe_table_data_import_job_input::Builder::default()
    }
    /// Creates a new `DescribeTableDataImportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTableDataImportJob {
                type Output = std::result::Result<crate::output::DescribeTableDataImportJobOutput, crate::error::DescribeTableDataImportJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_describe_table_data_import_job::de_describe_table_data_import_job_http_error(response)
                     } else {
                        crate::protocol_serde::shape_describe_table_data_import_job::de_describe_table_data_import_job_http_response(response)
                     }
                }
            }

/// Operation shape for `GetScreenData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_screen_data`](crate::client::fluent_builders::GetScreenData).
            ///
            /// `ParseStrictResponse` impl for `GetScreenData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetScreenData {
    _private: ()
}
impl GetScreenData {
    /// Creates a new builder-style object to manufacture [`GetScreenDataInput`](crate::input::GetScreenDataInput).
    pub fn builder() -> crate::input::get_screen_data_input::Builder {
        crate::input::get_screen_data_input::Builder::default()
    }
    /// Creates a new `GetScreenData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetScreenData {
                type Output = std::result::Result<crate::output::GetScreenDataOutput, crate::error::GetScreenDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_screen_data::de_get_screen_data_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_screen_data::de_get_screen_data_http_response(response)
                     }
                }
            }

/// Operation shape for `InvokeScreenAutomation`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`invoke_screen_automation`](crate::client::fluent_builders::InvokeScreenAutomation).
            ///
            /// `ParseStrictResponse` impl for `InvokeScreenAutomation`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct InvokeScreenAutomation {
    _private: ()
}
impl InvokeScreenAutomation {
    /// Creates a new builder-style object to manufacture [`InvokeScreenAutomationInput`](crate::input::InvokeScreenAutomationInput).
    pub fn builder() -> crate::input::invoke_screen_automation_input::Builder {
        crate::input::invoke_screen_automation_input::Builder::default()
    }
    /// Creates a new `InvokeScreenAutomation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for InvokeScreenAutomation {
                type Output = std::result::Result<crate::output::InvokeScreenAutomationOutput, crate::error::InvokeScreenAutomationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_invoke_screen_automation::de_invoke_screen_automation_http_error(response)
                     } else {
                        crate::protocol_serde::shape_invoke_screen_automation::de_invoke_screen_automation_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTableColumns`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_table_columns`](crate::client::fluent_builders::ListTableColumns).
            ///
            /// `ParseStrictResponse` impl for `ListTableColumns`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTableColumns {
    _private: ()
}
impl ListTableColumns {
    /// Creates a new builder-style object to manufacture [`ListTableColumnsInput`](crate::input::ListTableColumnsInput).
    pub fn builder() -> crate::input::list_table_columns_input::Builder {
        crate::input::list_table_columns_input::Builder::default()
    }
    /// Creates a new `ListTableColumns` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTableColumns {
                type Output = std::result::Result<crate::output::ListTableColumnsOutput, crate::error::ListTableColumnsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_table_columns::de_list_table_columns_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_table_columns::de_list_table_columns_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_table_rows`](crate::client::fluent_builders::ListTableRows).
            ///
            /// `ParseStrictResponse` impl for `ListTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTableRows {
    _private: ()
}
impl ListTableRows {
    /// Creates a new builder-style object to manufacture [`ListTableRowsInput`](crate::input::ListTableRowsInput).
    pub fn builder() -> crate::input::list_table_rows_input::Builder {
        crate::input::list_table_rows_input::Builder::default()
    }
    /// Creates a new `ListTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTableRows {
                type Output = std::result::Result<crate::output::ListTableRowsOutput, crate::error::ListTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_table_rows::de_list_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_table_rows::de_list_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTables`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tables`](crate::client::fluent_builders::ListTables).
            ///
            /// `ParseStrictResponse` impl for `ListTables`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTables {
    _private: ()
}
impl ListTables {
    /// Creates a new builder-style object to manufacture [`ListTablesInput`](crate::input::ListTablesInput).
    pub fn builder() -> crate::input::list_tables_input::Builder {
        crate::input::list_tables_input::Builder::default()
    }
    /// Creates a new `ListTables` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTables {
                type Output = std::result::Result<crate::output::ListTablesOutput, crate::error::ListTablesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_tables::de_list_tables_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_tables::de_list_tables_http_response(response)
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

/// Operation shape for `QueryTableRows`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`query_table_rows`](crate::client::fluent_builders::QueryTableRows).
            ///
            /// `ParseStrictResponse` impl for `QueryTableRows`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct QueryTableRows {
    _private: ()
}
impl QueryTableRows {
    /// Creates a new builder-style object to manufacture [`QueryTableRowsInput`](crate::input::QueryTableRowsInput).
    pub fn builder() -> crate::input::query_table_rows_input::Builder {
        crate::input::query_table_rows_input::Builder::default()
    }
    /// Creates a new `QueryTableRows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for QueryTableRows {
                type Output = std::result::Result<crate::output::QueryTableRowsOutput, crate::error::QueryTableRowsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_query_table_rows::de_query_table_rows_http_error(response)
                     } else {
                        crate::protocol_serde::shape_query_table_rows::de_query_table_rows_http_response(response)
                     }
                }
            }

/// Operation shape for `StartTableDataImportJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_table_data_import_job`](crate::client::fluent_builders::StartTableDataImportJob).
            ///
            /// `ParseStrictResponse` impl for `StartTableDataImportJob`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartTableDataImportJob {
    _private: ()
}
impl StartTableDataImportJob {
    /// Creates a new builder-style object to manufacture [`StartTableDataImportJobInput`](crate::input::StartTableDataImportJobInput).
    pub fn builder() -> crate::input::start_table_data_import_job_input::Builder {
        crate::input::start_table_data_import_job_input::Builder::default()
    }
    /// Creates a new `StartTableDataImportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartTableDataImportJob {
                type Output = std::result::Result<crate::output::StartTableDataImportJobOutput, crate::error::StartTableDataImportJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_table_data_import_job::de_start_table_data_import_job_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_table_data_import_job::de_start_table_data_import_job_http_response(response)
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

/// Operation customization and supporting types
pub mod customize;

