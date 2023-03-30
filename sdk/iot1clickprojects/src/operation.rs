// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateDeviceWithPlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_device_with_placement`](crate::client::fluent_builders::AssociateDeviceWithPlacement).
            ///
            /// `ParseStrictResponse` impl for `AssociateDeviceWithPlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateDeviceWithPlacement {
    _private: ()
}
impl AssociateDeviceWithPlacement {
    /// Creates a new builder-style object to manufacture [`AssociateDeviceWithPlacementInput`](crate::input::AssociateDeviceWithPlacementInput).
    pub fn builder() -> crate::input::associate_device_with_placement_input::Builder {
        crate::input::associate_device_with_placement_input::Builder::default()
    }
    /// Creates a new `AssociateDeviceWithPlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateDeviceWithPlacement {
                type Output = std::result::Result<crate::output::AssociateDeviceWithPlacementOutput, crate::error::AssociateDeviceWithPlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_associate_device_with_placement::de_associate_device_with_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_associate_device_with_placement::de_associate_device_with_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `CreatePlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_placement`](crate::client::fluent_builders::CreatePlacement).
            ///
            /// `ParseStrictResponse` impl for `CreatePlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreatePlacement {
    _private: ()
}
impl CreatePlacement {
    /// Creates a new builder-style object to manufacture [`CreatePlacementInput`](crate::input::CreatePlacementInput).
    pub fn builder() -> crate::input::create_placement_input::Builder {
        crate::input::create_placement_input::Builder::default()
    }
    /// Creates a new `CreatePlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePlacement {
                type Output = std::result::Result<crate::output::CreatePlacementOutput, crate::error::CreatePlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_placement::de_create_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_placement::de_create_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateProject`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_project`](crate::client::fluent_builders::CreateProject).
            ///
            /// `ParseStrictResponse` impl for `CreateProject`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateProject {
    _private: ()
}
impl CreateProject {
    /// Creates a new builder-style object to manufacture [`CreateProjectInput`](crate::input::CreateProjectInput).
    pub fn builder() -> crate::input::create_project_input::Builder {
        crate::input::create_project_input::Builder::default()
    }
    /// Creates a new `CreateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateProject {
                type Output = std::result::Result<crate::output::CreateProjectOutput, crate::error::CreateProjectError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_project::de_create_project_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_project::de_create_project_http_response(response)
                     }
                }
            }

/// Operation shape for `DeletePlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_placement`](crate::client::fluent_builders::DeletePlacement).
            ///
            /// `ParseStrictResponse` impl for `DeletePlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeletePlacement {
    _private: ()
}
impl DeletePlacement {
    /// Creates a new builder-style object to manufacture [`DeletePlacementInput`](crate::input::DeletePlacementInput).
    pub fn builder() -> crate::input::delete_placement_input::Builder {
        crate::input::delete_placement_input::Builder::default()
    }
    /// Creates a new `DeletePlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePlacement {
                type Output = std::result::Result<crate::output::DeletePlacementOutput, crate::error::DeletePlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_placement::de_delete_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_placement::de_delete_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteProject`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_project`](crate::client::fluent_builders::DeleteProject).
            ///
            /// `ParseStrictResponse` impl for `DeleteProject`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteProject {
    _private: ()
}
impl DeleteProject {
    /// Creates a new builder-style object to manufacture [`DeleteProjectInput`](crate::input::DeleteProjectInput).
    pub fn builder() -> crate::input::delete_project_input::Builder {
        crate::input::delete_project_input::Builder::default()
    }
    /// Creates a new `DeleteProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteProject {
                type Output = std::result::Result<crate::output::DeleteProjectOutput, crate::error::DeleteProjectError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_project::de_delete_project_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_project::de_delete_project_http_response(response)
                     }
                }
            }

/// Operation shape for `DescribePlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_placement`](crate::client::fluent_builders::DescribePlacement).
            ///
            /// `ParseStrictResponse` impl for `DescribePlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribePlacement {
    _private: ()
}
impl DescribePlacement {
    /// Creates a new builder-style object to manufacture [`DescribePlacementInput`](crate::input::DescribePlacementInput).
    pub fn builder() -> crate::input::describe_placement_input::Builder {
        crate::input::describe_placement_input::Builder::default()
    }
    /// Creates a new `DescribePlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePlacement {
                type Output = std::result::Result<crate::output::DescribePlacementOutput, crate::error::DescribePlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_describe_placement::de_describe_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_describe_placement::de_describe_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `DescribeProject`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_project`](crate::client::fluent_builders::DescribeProject).
            ///
            /// `ParseStrictResponse` impl for `DescribeProject`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeProject {
    _private: ()
}
impl DescribeProject {
    /// Creates a new builder-style object to manufacture [`DescribeProjectInput`](crate::input::DescribeProjectInput).
    pub fn builder() -> crate::input::describe_project_input::Builder {
        crate::input::describe_project_input::Builder::default()
    }
    /// Creates a new `DescribeProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeProject {
                type Output = std::result::Result<crate::output::DescribeProjectOutput, crate::error::DescribeProjectError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_describe_project::de_describe_project_http_error(response)
                     } else {
                        crate::protocol_serde::shape_describe_project::de_describe_project_http_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateDeviceFromPlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_device_from_placement`](crate::client::fluent_builders::DisassociateDeviceFromPlacement).
            ///
            /// `ParseStrictResponse` impl for `DisassociateDeviceFromPlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateDeviceFromPlacement {
    _private: ()
}
impl DisassociateDeviceFromPlacement {
    /// Creates a new builder-style object to manufacture [`DisassociateDeviceFromPlacementInput`](crate::input::DisassociateDeviceFromPlacementInput).
    pub fn builder() -> crate::input::disassociate_device_from_placement_input::Builder {
        crate::input::disassociate_device_from_placement_input::Builder::default()
    }
    /// Creates a new `DisassociateDeviceFromPlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateDeviceFromPlacement {
                type Output = std::result::Result<crate::output::DisassociateDeviceFromPlacementOutput, crate::error::DisassociateDeviceFromPlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_disassociate_device_from_placement::de_disassociate_device_from_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_disassociate_device_from_placement::de_disassociate_device_from_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `GetDevicesInPlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_devices_in_placement`](crate::client::fluent_builders::GetDevicesInPlacement).
            ///
            /// `ParseStrictResponse` impl for `GetDevicesInPlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDevicesInPlacement {
    _private: ()
}
impl GetDevicesInPlacement {
    /// Creates a new builder-style object to manufacture [`GetDevicesInPlacementInput`](crate::input::GetDevicesInPlacementInput).
    pub fn builder() -> crate::input::get_devices_in_placement_input::Builder {
        crate::input::get_devices_in_placement_input::Builder::default()
    }
    /// Creates a new `GetDevicesInPlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDevicesInPlacement {
                type Output = std::result::Result<crate::output::GetDevicesInPlacementOutput, crate::error::GetDevicesInPlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_devices_in_placement::de_get_devices_in_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_devices_in_placement::de_get_devices_in_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `ListPlacements`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_placements`](crate::client::fluent_builders::ListPlacements).
            ///
            /// `ParseStrictResponse` impl for `ListPlacements`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPlacements {
    _private: ()
}
impl ListPlacements {
    /// Creates a new builder-style object to manufacture [`ListPlacementsInput`](crate::input::ListPlacementsInput).
    pub fn builder() -> crate::input::list_placements_input::Builder {
        crate::input::list_placements_input::Builder::default()
    }
    /// Creates a new `ListPlacements` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPlacements {
                type Output = std::result::Result<crate::output::ListPlacementsOutput, crate::error::ListPlacementsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_placements::de_list_placements_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_placements::de_list_placements_http_response(response)
                     }
                }
            }

/// Operation shape for `ListProjects`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_projects`](crate::client::fluent_builders::ListProjects).
            ///
            /// `ParseStrictResponse` impl for `ListProjects`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListProjects {
    _private: ()
}
impl ListProjects {
    /// Creates a new builder-style object to manufacture [`ListProjectsInput`](crate::input::ListProjectsInput).
    pub fn builder() -> crate::input::list_projects_input::Builder {
        crate::input::list_projects_input::Builder::default()
    }
    /// Creates a new `ListProjects` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProjects {
                type Output = std::result::Result<crate::output::ListProjectsOutput, crate::error::ListProjectsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_projects::de_list_projects_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_projects::de_list_projects_http_response(response)
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

/// Operation shape for `UpdatePlacement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_placement`](crate::client::fluent_builders::UpdatePlacement).
            ///
            /// `ParseStrictResponse` impl for `UpdatePlacement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdatePlacement {
    _private: ()
}
impl UpdatePlacement {
    /// Creates a new builder-style object to manufacture [`UpdatePlacementInput`](crate::input::UpdatePlacementInput).
    pub fn builder() -> crate::input::update_placement_input::Builder {
        crate::input::update_placement_input::Builder::default()
    }
    /// Creates a new `UpdatePlacement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdatePlacement {
                type Output = std::result::Result<crate::output::UpdatePlacementOutput, crate::error::UpdatePlacementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_placement::de_update_placement_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_placement::de_update_placement_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateProject`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_project`](crate::client::fluent_builders::UpdateProject).
            ///
            /// `ParseStrictResponse` impl for `UpdateProject`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateProject {
    _private: ()
}
impl UpdateProject {
    /// Creates a new builder-style object to manufacture [`UpdateProjectInput`](crate::input::UpdateProjectInput).
    pub fn builder() -> crate::input::update_project_input::Builder {
        crate::input::update_project_input::Builder::default()
    }
    /// Creates a new `UpdateProject` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateProject {
                type Output = std::result::Result<crate::output::UpdateProjectOutput, crate::error::UpdateProjectError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_project::de_update_project_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_project::de_update_project_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

