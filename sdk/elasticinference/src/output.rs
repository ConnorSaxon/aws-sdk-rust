// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p> The tags of the Elastic Inference Accelerator. </p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p> The tags of the Elastic Inference Accelerator. </p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p> The tags of the Elastic Inference Accelerator. </p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p> The tags of the Elastic Inference Accelerator. </p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeAcceleratorTypesOutput {
    /// <p> The available accelerator types. </p>
    pub accelerator_types: std::option::Option<std::vec::Vec<crate::model::AcceleratorType>>,
}
impl DescribeAcceleratorTypesOutput {
    /// <p> The available accelerator types. </p>
    pub fn accelerator_types(&self) -> std::option::Option<&[crate::model::AcceleratorType]> {
        self.accelerator_types.as_deref()
    }
}
impl std::fmt::Debug for DescribeAcceleratorTypesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeAcceleratorTypesOutput");
        formatter.field("accelerator_types", &self.accelerator_types);
        formatter.finish()
    }
}
/// See [`DescribeAcceleratorTypesOutput`](crate::output::DescribeAcceleratorTypesOutput)
pub mod describe_accelerator_types_output {
    /// A builder for [`DescribeAcceleratorTypesOutput`](crate::output::DescribeAcceleratorTypesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_types:
            std::option::Option<std::vec::Vec<crate::model::AcceleratorType>>,
    }
    impl Builder {
        /// Appends an item to `accelerator_types`.
        ///
        /// To override the contents of this collection use [`set_accelerator_types`](Self::set_accelerator_types).
        ///
        /// <p> The available accelerator types. </p>
        pub fn accelerator_types(mut self, input: crate::model::AcceleratorType) -> Self {
            let mut v = self.accelerator_types.unwrap_or_default();
            v.push(input);
            self.accelerator_types = Some(v);
            self
        }
        /// <p> The available accelerator types. </p>
        pub fn set_accelerator_types(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AcceleratorType>>,
        ) -> Self {
            self.accelerator_types = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeAcceleratorTypesOutput`](crate::output::DescribeAcceleratorTypesOutput)
        pub fn build(self) -> crate::output::DescribeAcceleratorTypesOutput {
            crate::output::DescribeAcceleratorTypesOutput {
                accelerator_types: self.accelerator_types,
            }
        }
    }
}
impl DescribeAcceleratorTypesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorTypesOutput`](crate::output::DescribeAcceleratorTypesOutput)
    pub fn builder() -> crate::output::describe_accelerator_types_output::Builder {
        crate::output::describe_accelerator_types_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeAcceleratorsOutput {
    /// <p> The details of the Elastic Inference Accelerators. </p>
    pub accelerator_set:
        std::option::Option<std::vec::Vec<crate::model::ElasticInferenceAccelerator>>,
    /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeAcceleratorsOutput {
    /// <p> The details of the Elastic Inference Accelerators. </p>
    pub fn accelerator_set(
        &self,
    ) -> std::option::Option<&[crate::model::ElasticInferenceAccelerator]> {
        self.accelerator_set.as_deref()
    }
    /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeAcceleratorsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeAcceleratorsOutput");
        formatter.field("accelerator_set", &self.accelerator_set);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeAcceleratorsOutput`](crate::output::DescribeAcceleratorsOutput)
pub mod describe_accelerators_output {
    /// A builder for [`DescribeAcceleratorsOutput`](crate::output::DescribeAcceleratorsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_set:
            std::option::Option<std::vec::Vec<crate::model::ElasticInferenceAccelerator>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `accelerator_set`.
        ///
        /// To override the contents of this collection use [`set_accelerator_set`](Self::set_accelerator_set).
        ///
        /// <p> The details of the Elastic Inference Accelerators. </p>
        pub fn accelerator_set(mut self, input: crate::model::ElasticInferenceAccelerator) -> Self {
            let mut v = self.accelerator_set.unwrap_or_default();
            v.push(input);
            self.accelerator_set = Some(v);
            self
        }
        /// <p> The details of the Elastic Inference Accelerators. </p>
        pub fn set_accelerator_set(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ElasticInferenceAccelerator>>,
        ) -> Self {
            self.accelerator_set = input;
            self
        }
        /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p> A token to specify where to start paginating. This is the NextToken from a previously truncated response. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeAcceleratorsOutput`](crate::output::DescribeAcceleratorsOutput)
        pub fn build(self) -> crate::output::DescribeAcceleratorsOutput {
            crate::output::DescribeAcceleratorsOutput {
                accelerator_set: self.accelerator_set,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeAcceleratorsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorsOutput`](crate::output::DescribeAcceleratorsOutput)
    pub fn builder() -> crate::output::describe_accelerators_output::Builder {
        crate::output::describe_accelerators_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeAcceleratorOfferingsOutput {
    /// <p> The list of accelerator type offerings for a specific location. </p>
    pub accelerator_type_offerings:
        std::option::Option<std::vec::Vec<crate::model::AcceleratorTypeOffering>>,
}
impl DescribeAcceleratorOfferingsOutput {
    /// <p> The list of accelerator type offerings for a specific location. </p>
    pub fn accelerator_type_offerings(
        &self,
    ) -> std::option::Option<&[crate::model::AcceleratorTypeOffering]> {
        self.accelerator_type_offerings.as_deref()
    }
}
impl std::fmt::Debug for DescribeAcceleratorOfferingsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeAcceleratorOfferingsOutput");
        formatter.field(
            "accelerator_type_offerings",
            &self.accelerator_type_offerings,
        );
        formatter.finish()
    }
}
/// See [`DescribeAcceleratorOfferingsOutput`](crate::output::DescribeAcceleratorOfferingsOutput)
pub mod describe_accelerator_offerings_output {
    /// A builder for [`DescribeAcceleratorOfferingsOutput`](crate::output::DescribeAcceleratorOfferingsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) accelerator_type_offerings:
            std::option::Option<std::vec::Vec<crate::model::AcceleratorTypeOffering>>,
    }
    impl Builder {
        /// Appends an item to `accelerator_type_offerings`.
        ///
        /// To override the contents of this collection use [`set_accelerator_type_offerings`](Self::set_accelerator_type_offerings).
        ///
        /// <p> The list of accelerator type offerings for a specific location. </p>
        pub fn accelerator_type_offerings(
            mut self,
            input: crate::model::AcceleratorTypeOffering,
        ) -> Self {
            let mut v = self.accelerator_type_offerings.unwrap_or_default();
            v.push(input);
            self.accelerator_type_offerings = Some(v);
            self
        }
        /// <p> The list of accelerator type offerings for a specific location. </p>
        pub fn set_accelerator_type_offerings(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AcceleratorTypeOffering>>,
        ) -> Self {
            self.accelerator_type_offerings = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeAcceleratorOfferingsOutput`](crate::output::DescribeAcceleratorOfferingsOutput)
        pub fn build(self) -> crate::output::DescribeAcceleratorOfferingsOutput {
            crate::output::DescribeAcceleratorOfferingsOutput {
                accelerator_type_offerings: self.accelerator_type_offerings,
            }
        }
    }
}
impl DescribeAcceleratorOfferingsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorOfferingsOutput`](crate::output::DescribeAcceleratorOfferingsOutput)
    pub fn builder() -> crate::output::describe_accelerator_offerings_output::Builder {
        crate::output::describe_accelerator_offerings_output::Builder::default()
    }
}
