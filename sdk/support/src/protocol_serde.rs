// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_add_attachments_to_set;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_add_communication_to_case;

pub(crate) mod shape_create_case;

pub(crate) mod shape_describe_attachment;

pub(crate) mod shape_describe_cases;

pub(crate) mod shape_describe_communications;

pub(crate) mod shape_describe_services;

pub(crate) mod shape_describe_severity_levels;

pub(crate) mod shape_describe_trusted_advisor_check_refresh_statuses;

pub(crate) mod shape_describe_trusted_advisor_check_result;

pub(crate) mod shape_describe_trusted_advisor_check_summaries;

pub(crate) mod shape_describe_trusted_advisor_checks;

pub(crate) mod shape_refresh_trusted_advisor_check;

pub(crate) mod shape_resolve_case;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_add_attachments_to_set_input;

pub(crate) mod shape_add_communication_to_case_input;

pub(crate) mod shape_attachment_id_not_found;

pub(crate) mod shape_attachment_limit_exceeded;

pub(crate) mod shape_attachment_set_expired;

pub(crate) mod shape_attachment_set_id_not_found;

pub(crate) mod shape_attachment_set_size_limit_exceeded;

pub(crate) mod shape_case_creation_limit_exceeded;

pub(crate) mod shape_case_id_not_found;

pub(crate) mod shape_create_case_input;

pub(crate) mod shape_describe_attachment_input;

pub(crate) mod shape_describe_attachment_limit_exceeded;

pub(crate) mod shape_describe_cases_input;

pub(crate) mod shape_describe_communications_input;

pub(crate) mod shape_describe_services_input;

pub(crate) mod shape_describe_severity_levels_input;

pub(crate) mod shape_describe_trusted_advisor_check_refresh_statuses_input;

pub(crate) mod shape_describe_trusted_advisor_check_result_input;

pub(crate) mod shape_describe_trusted_advisor_check_summaries_input;

pub(crate) mod shape_describe_trusted_advisor_checks_input;

pub(crate) mod shape_internal_server_error;

pub(crate) mod shape_refresh_trusted_advisor_check_input;

pub(crate) mod shape_resolve_case_input;

pub(crate) mod shape_attachment;

pub(crate) mod shape_case_list;

pub(crate) mod shape_communication_list;

pub(crate) mod shape_service_list;

pub(crate) mod shape_severity_levels_list;

pub(crate) mod shape_trusted_advisor_check_list;

pub(crate) mod shape_trusted_advisor_check_refresh_status;

pub(crate) mod shape_trusted_advisor_check_refresh_status_list;

pub(crate) mod shape_trusted_advisor_check_result;

pub(crate) mod shape_trusted_advisor_check_summary_list;

pub(crate) mod shape_case_details;

pub(crate) mod shape_communication;

pub(crate) mod shape_service;

pub(crate) mod shape_severity_level;

pub(crate) mod shape_trusted_advisor_category_specific_summary;

pub(crate) mod shape_trusted_advisor_check_description;

pub(crate) mod shape_trusted_advisor_check_summary;

pub(crate) mod shape_trusted_advisor_resource_detail_list;

pub(crate) mod shape_trusted_advisor_resources_summary;

pub(crate) mod shape_attachment_set;

pub(crate) mod shape_category_list;

pub(crate) mod shape_cc_email_address_list;

pub(crate) mod shape_recent_case_communications;

pub(crate) mod shape_string_list;

pub(crate) mod shape_trusted_advisor_cost_optimizing_summary;

pub(crate) mod shape_trusted_advisor_resource_detail;

pub(crate) mod shape_attachment_details;

pub(crate) mod shape_category;

