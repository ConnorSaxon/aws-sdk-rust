// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateDefaultView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_default_view`](crate::client::fluent_builders::AssociateDefaultView).
            ///
            /// `ParseStrictResponse` impl for `AssociateDefaultView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateDefaultView {
    _private: ()
}
impl AssociateDefaultView {
    /// Creates a new builder-style object to manufacture [`AssociateDefaultViewInput`](crate::input::AssociateDefaultViewInput).
    pub fn builder() -> crate::input::associate_default_view_input::Builder {
        crate::input::associate_default_view_input::Builder::default()
    }
    /// Creates a new `AssociateDefaultView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateDefaultView {
                type Output = std::result::Result<crate::output::AssociateDefaultViewOutput, crate::error::AssociateDefaultViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_associate_default_view::de_associate_default_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_associate_default_view::de_associate_default_view_http_response(response)
                     }
                }
            }

/// Operation shape for `BatchGetView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_get_view`](crate::client::fluent_builders::BatchGetView).
            ///
            /// `ParseStrictResponse` impl for `BatchGetView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetView {
    _private: ()
}
impl BatchGetView {
    /// Creates a new builder-style object to manufacture [`BatchGetViewInput`](crate::input::BatchGetViewInput).
    pub fn builder() -> crate::input::batch_get_view_input::Builder {
        crate::input::batch_get_view_input::Builder::default()
    }
    /// Creates a new `BatchGetView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetView {
                type Output = std::result::Result<crate::output::BatchGetViewOutput, crate::error::BatchGetViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_get_view::de_batch_get_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_get_view::de_batch_get_view_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateIndex`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_index`](crate::client::fluent_builders::CreateIndex).
            ///
            /// `ParseStrictResponse` impl for `CreateIndex`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateIndex {
    _private: ()
}
impl CreateIndex {
    /// Creates a new builder-style object to manufacture [`CreateIndexInput`](crate::input::CreateIndexInput).
    pub fn builder() -> crate::input::create_index_input::Builder {
        crate::input::create_index_input::Builder::default()
    }
    /// Creates a new `CreateIndex` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateIndex {
                type Output = std::result::Result<crate::output::CreateIndexOutput, crate::error::CreateIndexError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_index::de_create_index_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_index::de_create_index_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_view`](crate::client::fluent_builders::CreateView).
            ///
            /// `ParseStrictResponse` impl for `CreateView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateView {
    _private: ()
}
impl CreateView {
    /// Creates a new builder-style object to manufacture [`CreateViewInput`](crate::input::CreateViewInput).
    pub fn builder() -> crate::input::create_view_input::Builder {
        crate::input::create_view_input::Builder::default()
    }
    /// Creates a new `CreateView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateView {
                type Output = std::result::Result<crate::output::CreateViewOutput, crate::error::CreateViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_view::de_create_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_view::de_create_view_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteIndex`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_index`](crate::client::fluent_builders::DeleteIndex).
            ///
            /// `ParseStrictResponse` impl for `DeleteIndex`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteIndex {
    _private: ()
}
impl DeleteIndex {
    /// Creates a new builder-style object to manufacture [`DeleteIndexInput`](crate::input::DeleteIndexInput).
    pub fn builder() -> crate::input::delete_index_input::Builder {
        crate::input::delete_index_input::Builder::default()
    }
    /// Creates a new `DeleteIndex` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteIndex {
                type Output = std::result::Result<crate::output::DeleteIndexOutput, crate::error::DeleteIndexError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_index::de_delete_index_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_index::de_delete_index_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_view`](crate::client::fluent_builders::DeleteView).
            ///
            /// `ParseStrictResponse` impl for `DeleteView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteView {
    _private: ()
}
impl DeleteView {
    /// Creates a new builder-style object to manufacture [`DeleteViewInput`](crate::input::DeleteViewInput).
    pub fn builder() -> crate::input::delete_view_input::Builder {
        crate::input::delete_view_input::Builder::default()
    }
    /// Creates a new `DeleteView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteView {
                type Output = std::result::Result<crate::output::DeleteViewOutput, crate::error::DeleteViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_view::de_delete_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_view::de_delete_view_http_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateDefaultView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_default_view`](crate::client::fluent_builders::DisassociateDefaultView).
            ///
            /// `ParseStrictResponse` impl for `DisassociateDefaultView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateDefaultView {
    _private: ()
}
impl DisassociateDefaultView {
    /// Creates a new builder-style object to manufacture [`DisassociateDefaultViewInput`](crate::input::DisassociateDefaultViewInput).
    pub fn builder() -> crate::input::disassociate_default_view_input::Builder {
        crate::input::disassociate_default_view_input::Builder::default()
    }
    /// Creates a new `DisassociateDefaultView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateDefaultView {
                type Output = std::result::Result<crate::output::DisassociateDefaultViewOutput, crate::error::DisassociateDefaultViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_disassociate_default_view::de_disassociate_default_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_disassociate_default_view::de_disassociate_default_view_http_response(response)
                     }
                }
            }

/// Operation shape for `GetDefaultView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_default_view`](crate::client::fluent_builders::GetDefaultView).
            ///
            /// `ParseStrictResponse` impl for `GetDefaultView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetDefaultView {
    _private: ()
}
impl GetDefaultView {
    /// Creates a new builder-style object to manufacture [`GetDefaultViewInput`](crate::input::GetDefaultViewInput).
    pub fn builder() -> crate::input::get_default_view_input::Builder {
        crate::input::get_default_view_input::Builder::default()
    }
    /// Creates a new `GetDefaultView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDefaultView {
                type Output = std::result::Result<crate::output::GetDefaultViewOutput, crate::error::GetDefaultViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_default_view::de_get_default_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_default_view::de_get_default_view_http_response(response)
                     }
                }
            }

/// Operation shape for `GetIndex`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_index`](crate::client::fluent_builders::GetIndex).
            ///
            /// `ParseStrictResponse` impl for `GetIndex`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetIndex {
    _private: ()
}
impl GetIndex {
    /// Creates a new builder-style object to manufacture [`GetIndexInput`](crate::input::GetIndexInput).
    pub fn builder() -> crate::input::get_index_input::Builder {
        crate::input::get_index_input::Builder::default()
    }
    /// Creates a new `GetIndex` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetIndex {
                type Output = std::result::Result<crate::output::GetIndexOutput, crate::error::GetIndexError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_index::de_get_index_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_index::de_get_index_http_response(response)
                     }
                }
            }

/// Operation shape for `GetView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_view`](crate::client::fluent_builders::GetView).
            ///
            /// `ParseStrictResponse` impl for `GetView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetView {
    _private: ()
}
impl GetView {
    /// Creates a new builder-style object to manufacture [`GetViewInput`](crate::input::GetViewInput).
    pub fn builder() -> crate::input::get_view_input::Builder {
        crate::input::get_view_input::Builder::default()
    }
    /// Creates a new `GetView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetView {
                type Output = std::result::Result<crate::output::GetViewOutput, crate::error::GetViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_view::de_get_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_view::de_get_view_http_response(response)
                     }
                }
            }

/// Operation shape for `ListIndexes`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_indexes`](crate::client::fluent_builders::ListIndexes).
            ///
            /// `ParseStrictResponse` impl for `ListIndexes`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListIndexes {
    _private: ()
}
impl ListIndexes {
    /// Creates a new builder-style object to manufacture [`ListIndexesInput`](crate::input::ListIndexesInput).
    pub fn builder() -> crate::input::list_indexes_input::Builder {
        crate::input::list_indexes_input::Builder::default()
    }
    /// Creates a new `ListIndexes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListIndexes {
                type Output = std::result::Result<crate::output::ListIndexesOutput, crate::error::ListIndexesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_indexes::de_list_indexes_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_indexes::de_list_indexes_http_response(response)
                     }
                }
            }

/// Operation shape for `ListSupportedResourceTypes`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_supported_resource_types`](crate::client::fluent_builders::ListSupportedResourceTypes).
            ///
            /// `ParseStrictResponse` impl for `ListSupportedResourceTypes`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSupportedResourceTypes {
    _private: ()
}
impl ListSupportedResourceTypes {
    /// Creates a new builder-style object to manufacture [`ListSupportedResourceTypesInput`](crate::input::ListSupportedResourceTypesInput).
    pub fn builder() -> crate::input::list_supported_resource_types_input::Builder {
        crate::input::list_supported_resource_types_input::Builder::default()
    }
    /// Creates a new `ListSupportedResourceTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSupportedResourceTypes {
                type Output = std::result::Result<crate::output::ListSupportedResourceTypesOutput, crate::error::ListSupportedResourceTypesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_supported_resource_types::de_list_supported_resource_types_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_supported_resource_types::de_list_supported_resource_types_http_response(response)
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

/// Operation shape for `ListViews`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_views`](crate::client::fluent_builders::ListViews).
            ///
            /// `ParseStrictResponse` impl for `ListViews`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListViews {
    _private: ()
}
impl ListViews {
    /// Creates a new builder-style object to manufacture [`ListViewsInput`](crate::input::ListViewsInput).
    pub fn builder() -> crate::input::list_views_input::Builder {
        crate::input::list_views_input::Builder::default()
    }
    /// Creates a new `ListViews` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListViews {
                type Output = std::result::Result<crate::output::ListViewsOutput, crate::error::ListViewsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_views::de_list_views_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_views::de_list_views_http_response(response)
                     }
                }
            }

/// Operation shape for `Search`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`search`](crate::client::fluent_builders::Search).
            ///
            /// `ParseStrictResponse` impl for `Search`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Search {
    _private: ()
}
impl Search {
    /// Creates a new builder-style object to manufacture [`SearchInput`](crate::input::SearchInput).
    pub fn builder() -> crate::input::search_input::Builder {
        crate::input::search_input::Builder::default()
    }
    /// Creates a new `Search` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Search {
                type Output = std::result::Result<crate::output::SearchOutput, crate::error::SearchError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_search::de_search_http_error(response)
                     } else {
                        crate::protocol_serde::shape_search::de_search_http_response(response)
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

/// Operation shape for `UpdateIndexType`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_index_type`](crate::client::fluent_builders::UpdateIndexType).
            ///
            /// `ParseStrictResponse` impl for `UpdateIndexType`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateIndexType {
    _private: ()
}
impl UpdateIndexType {
    /// Creates a new builder-style object to manufacture [`UpdateIndexTypeInput`](crate::input::UpdateIndexTypeInput).
    pub fn builder() -> crate::input::update_index_type_input::Builder {
        crate::input::update_index_type_input::Builder::default()
    }
    /// Creates a new `UpdateIndexType` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateIndexType {
                type Output = std::result::Result<crate::output::UpdateIndexTypeOutput, crate::error::UpdateIndexTypeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_index_type::de_update_index_type_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_index_type::de_update_index_type_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateView`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_view`](crate::client::fluent_builders::UpdateView).
            ///
            /// `ParseStrictResponse` impl for `UpdateView`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateView {
    _private: ()
}
impl UpdateView {
    /// Creates a new builder-style object to manufacture [`UpdateViewInput`](crate::input::UpdateViewInput).
    pub fn builder() -> crate::input::update_view_input::Builder {
        crate::input::update_view_input::Builder::default()
    }
    /// Creates a new `UpdateView` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateView {
                type Output = std::result::Result<crate::output::UpdateViewOutput, crate::error::UpdateViewError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_view::de_update_view_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_view::de_update_view_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

