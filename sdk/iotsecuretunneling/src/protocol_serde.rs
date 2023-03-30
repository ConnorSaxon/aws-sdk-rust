// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_close_tunnel;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_describe_tunnel;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_tunnels;

pub(crate) mod shape_open_tunnel;

pub(crate) mod shape_rotate_tunnel_access_token;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_close_tunnel_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_describe_tunnel_input;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_tunnels_input;

pub(crate) mod shape_open_tunnel_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_rotate_tunnel_access_token_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_destination_config;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_timeout_config;

pub(crate) mod shape_tunnel;

pub(crate) mod shape_tunnel_summary_list;

pub(crate) mod shape_connection_state;

pub(crate) mod shape_tunnel_summary;

pub(crate) mod shape_service_list;

