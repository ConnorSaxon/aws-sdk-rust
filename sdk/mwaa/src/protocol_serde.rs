// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_cli_token;

pub(crate) mod shape_create_environment;

pub(crate) mod shape_create_web_login_token;

pub(crate) mod shape_delete_environment;

pub(crate) mod shape_get_environment;

pub(crate) mod shape_list_environments;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_publish_metrics;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_environment;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_create_environment_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_publish_metrics_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_update_environment_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_environment;

pub(crate) mod shape_environment_list;

pub(crate) mod shape_logging_configuration_input;

pub(crate) mod shape_metric_datum;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_update_network_configuration_input;

pub(crate) mod shape_airflow_configuration_options;

pub(crate) mod shape_dimension;

pub(crate) mod shape_last_update;

pub(crate) mod shape_logging_configuration;

pub(crate) mod shape_module_logging_configuration_input;

pub(crate) mod shape_statistic_set;

pub(crate) mod shape_module_logging_configuration;

pub(crate) mod shape_security_group_list;

pub(crate) mod shape_subnet_list;

pub(crate) mod shape_update_error;

