// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_broker;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_configuration;

pub(crate) mod shape_create_tags;

pub(crate) mod shape_create_user;

pub(crate) mod shape_delete_broker;

pub(crate) mod shape_delete_tags;

pub(crate) mod shape_delete_user;

pub(crate) mod shape_describe_broker;

pub(crate) mod shape_describe_broker_engine_types;

pub(crate) mod shape_describe_broker_instance_options;

pub(crate) mod shape_describe_configuration;

pub(crate) mod shape_describe_configuration_revision;

pub(crate) mod shape_describe_user;

pub(crate) mod shape_list_brokers;

pub(crate) mod shape_list_configuration_revisions;

pub(crate) mod shape_list_configurations;

pub(crate) mod shape_list_tags;

pub(crate) mod shape_list_users;

pub(crate) mod shape_reboot_broker;

pub(crate) mod shape_update_broker;

pub(crate) mod shape_update_configuration;

pub(crate) mod shape_update_user;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_broker_input;

pub(crate) mod shape_create_configuration_input;

pub(crate) mod shape_create_tags_input;

pub(crate) mod shape_create_user_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_internal_server_error_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_update_broker_input;

pub(crate) mod shape_update_configuration_input;

pub(crate) mod shape_update_user_input;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of__string;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_action_required;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_broker_engine_type;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_broker_instance;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_broker_instance_option;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_broker_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_configuration;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_configuration_revision;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_sanitization_warning;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_user_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___map_of__string;

pub(crate) mod shape_configuration_id;

pub(crate) mod shape_configuration_revision;

pub(crate) mod shape_configurations;

pub(crate) mod shape_encryption_options;

pub(crate) mod shape_ldap_server_metadata_input;

pub(crate) mod shape_ldap_server_metadata_output;

pub(crate) mod shape_logs;

pub(crate) mod shape_logs_summary;

pub(crate) mod shape_user;

pub(crate) mod shape_user_pending_changes;

pub(crate) mod shape_weekly_start_time;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_configuration_id;

pub(crate) mod shape_action_required;

pub(crate) mod shape_broker_engine_type;

pub(crate) mod shape_broker_instance;

pub(crate) mod shape_broker_instance_option;

pub(crate) mod shape_broker_summary;

pub(crate) mod shape_configuration;

pub(crate) mod shape_pending_logs;

pub(crate) mod shape_sanitization_warning;

pub(crate) mod shape_user_summary;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_availability_zone;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_deployment_mode;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_engine_version;

pub(crate) mod shape_availability_zone;

pub(crate) mod shape_engine_version;

