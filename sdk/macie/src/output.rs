// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateS3ResourcesOutput {
    /// <p>The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item. </p>
    pub failed_s3_resources: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
}
impl UpdateS3ResourcesOutput {
    /// <p>The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item. </p>
    pub fn failed_s3_resources(&self) -> std::option::Option<&[crate::model::FailedS3Resource]> {
        self.failed_s3_resources.as_deref()
    }
}
impl std::fmt::Debug for UpdateS3ResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateS3ResourcesOutput");
        formatter.field("failed_s3_resources", &self.failed_s3_resources);
        formatter.finish()
    }
}
/// See [`UpdateS3ResourcesOutput`](crate::output::UpdateS3ResourcesOutput)
pub mod update_s3_resources_output {
    /// A builder for [`UpdateS3ResourcesOutput`](crate::output::UpdateS3ResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_s3_resources:
            std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
    }
    impl Builder {
        /// Appends an item to `failed_s3_resources`.
        ///
        /// To override the contents of this collection use [`set_failed_s3_resources`](Self::set_failed_s3_resources).
        ///
        /// <p>The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item. </p>
        pub fn failed_s3_resources(mut self, input: crate::model::FailedS3Resource) -> Self {
            let mut v = self.failed_s3_resources.unwrap_or_default();
            v.push(input);
            self.failed_s3_resources = Some(v);
            self
        }
        /// <p>The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item. </p>
        pub fn set_failed_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
        ) -> Self {
            self.failed_s3_resources = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateS3ResourcesOutput`](crate::output::UpdateS3ResourcesOutput)
        pub fn build(self) -> crate::output::UpdateS3ResourcesOutput {
            crate::output::UpdateS3ResourcesOutput {
                failed_s3_resources: self.failed_s3_resources,
            }
        }
    }
}
impl UpdateS3ResourcesOutput {
    /// Creates a new builder-style object to manufacture [`UpdateS3ResourcesOutput`](crate::output::UpdateS3ResourcesOutput)
    pub fn builder() -> crate::output::update_s3_resources_output::Builder {
        crate::output::update_s3_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListS3ResourcesOutput {
    /// <p>A list of the associated S3 resources returned by the action.</p>
    pub s3_resources: std::option::Option<std::vec::Vec<crate::model::S3ResourceClassification>>,
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListS3ResourcesOutput {
    /// <p>A list of the associated S3 resources returned by the action.</p>
    pub fn s3_resources(&self) -> std::option::Option<&[crate::model::S3ResourceClassification]> {
        self.s3_resources.as_deref()
    }
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListS3ResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListS3ResourcesOutput");
        formatter.field("s3_resources", &self.s3_resources);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListS3ResourcesOutput`](crate::output::ListS3ResourcesOutput)
pub mod list_s3_resources_output {
    /// A builder for [`ListS3ResourcesOutput`](crate::output::ListS3ResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) s3_resources:
            std::option::Option<std::vec::Vec<crate::model::S3ResourceClassification>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `s3_resources`.
        ///
        /// To override the contents of this collection use [`set_s3_resources`](Self::set_s3_resources).
        ///
        /// <p>A list of the associated S3 resources returned by the action.</p>
        pub fn s3_resources(mut self, input: crate::model::S3ResourceClassification) -> Self {
            let mut v = self.s3_resources.unwrap_or_default();
            v.push(input);
            self.s3_resources = Some(v);
            self
        }
        /// <p>A list of the associated S3 resources returned by the action.</p>
        pub fn set_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::S3ResourceClassification>>,
        ) -> Self {
            self.s3_resources = input;
            self
        }
        /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListS3ResourcesOutput`](crate::output::ListS3ResourcesOutput)
        pub fn build(self) -> crate::output::ListS3ResourcesOutput {
            crate::output::ListS3ResourcesOutput {
                s3_resources: self.s3_resources,
                next_token: self.next_token,
            }
        }
    }
}
impl ListS3ResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ListS3ResourcesOutput`](crate::output::ListS3ResourcesOutput)
    pub fn builder() -> crate::output::list_s3_resources_output::Builder {
        crate::output::list_s3_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListMemberAccountsOutput {
    /// <p>A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list. </p>
    pub member_accounts: std::option::Option<std::vec::Vec<crate::model::MemberAccount>>,
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListMemberAccountsOutput {
    /// <p>A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list. </p>
    pub fn member_accounts(&self) -> std::option::Option<&[crate::model::MemberAccount]> {
        self.member_accounts.as_deref()
    }
    /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListMemberAccountsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListMemberAccountsOutput");
        formatter.field("member_accounts", &self.member_accounts);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListMemberAccountsOutput`](crate::output::ListMemberAccountsOutput)
pub mod list_member_accounts_output {
    /// A builder for [`ListMemberAccountsOutput`](crate::output::ListMemberAccountsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) member_accounts: std::option::Option<std::vec::Vec<crate::model::MemberAccount>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `member_accounts`.
        ///
        /// To override the contents of this collection use [`set_member_accounts`](Self::set_member_accounts).
        ///
        /// <p>A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list. </p>
        pub fn member_accounts(mut self, input: crate::model::MemberAccount) -> Self {
            let mut v = self.member_accounts.unwrap_or_default();
            v.push(input);
            self.member_accounts = Some(v);
            self
        }
        /// <p>A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list. </p>
        pub fn set_member_accounts(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::MemberAccount>>,
        ) -> Self {
            self.member_accounts = input;
            self
        }
        /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListMemberAccountsOutput`](crate::output::ListMemberAccountsOutput)
        pub fn build(self) -> crate::output::ListMemberAccountsOutput {
            crate::output::ListMemberAccountsOutput {
                member_accounts: self.member_accounts,
                next_token: self.next_token,
            }
        }
    }
}
impl ListMemberAccountsOutput {
    /// Creates a new builder-style object to manufacture [`ListMemberAccountsOutput`](crate::output::ListMemberAccountsOutput)
    pub fn builder() -> crate::output::list_member_accounts_output::Builder {
        crate::output::list_member_accounts_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DisassociateS3ResourcesOutput {
    /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
    pub failed_s3_resources: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
}
impl DisassociateS3ResourcesOutput {
    /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
    pub fn failed_s3_resources(&self) -> std::option::Option<&[crate::model::FailedS3Resource]> {
        self.failed_s3_resources.as_deref()
    }
}
impl std::fmt::Debug for DisassociateS3ResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DisassociateS3ResourcesOutput");
        formatter.field("failed_s3_resources", &self.failed_s3_resources);
        formatter.finish()
    }
}
/// See [`DisassociateS3ResourcesOutput`](crate::output::DisassociateS3ResourcesOutput)
pub mod disassociate_s3_resources_output {
    /// A builder for [`DisassociateS3ResourcesOutput`](crate::output::DisassociateS3ResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_s3_resources:
            std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
    }
    impl Builder {
        /// Appends an item to `failed_s3_resources`.
        ///
        /// To override the contents of this collection use [`set_failed_s3_resources`](Self::set_failed_s3_resources).
        ///
        /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
        pub fn failed_s3_resources(mut self, input: crate::model::FailedS3Resource) -> Self {
            let mut v = self.failed_s3_resources.unwrap_or_default();
            v.push(input);
            self.failed_s3_resources = Some(v);
            self
        }
        /// <p>S3 resources that couldn't be removed from being monitored and classified by Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
        pub fn set_failed_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
        ) -> Self {
            self.failed_s3_resources = input;
            self
        }
        /// Consumes the builder and constructs a [`DisassociateS3ResourcesOutput`](crate::output::DisassociateS3ResourcesOutput)
        pub fn build(self) -> crate::output::DisassociateS3ResourcesOutput {
            crate::output::DisassociateS3ResourcesOutput {
                failed_s3_resources: self.failed_s3_resources,
            }
        }
    }
}
impl DisassociateS3ResourcesOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateS3ResourcesOutput`](crate::output::DisassociateS3ResourcesOutput)
    pub fn builder() -> crate::output::disassociate_s3_resources_output::Builder {
        crate::output::disassociate_s3_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DisassociateMemberAccountOutput {}
impl std::fmt::Debug for DisassociateMemberAccountOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DisassociateMemberAccountOutput");
        formatter.finish()
    }
}
/// See [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
pub mod disassociate_member_account_output {
    /// A builder for [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
        pub fn build(self) -> crate::output::DisassociateMemberAccountOutput {
            crate::output::DisassociateMemberAccountOutput {}
        }
    }
}
impl DisassociateMemberAccountOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
    pub fn builder() -> crate::output::disassociate_member_account_output::Builder {
        crate::output::disassociate_member_account_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AssociateS3ResourcesOutput {
    /// <p>S3 resources that couldn't be associated with Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
    pub failed_s3_resources: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
}
impl AssociateS3ResourcesOutput {
    /// <p>S3 resources that couldn't be associated with Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
    pub fn failed_s3_resources(&self) -> std::option::Option<&[crate::model::FailedS3Resource]> {
        self.failed_s3_resources.as_deref()
    }
}
impl std::fmt::Debug for AssociateS3ResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AssociateS3ResourcesOutput");
        formatter.field("failed_s3_resources", &self.failed_s3_resources);
        formatter.finish()
    }
}
/// See [`AssociateS3ResourcesOutput`](crate::output::AssociateS3ResourcesOutput)
pub mod associate_s3_resources_output {
    /// A builder for [`AssociateS3ResourcesOutput`](crate::output::AssociateS3ResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_s3_resources:
            std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
    }
    impl Builder {
        /// Appends an item to `failed_s3_resources`.
        ///
        /// To override the contents of this collection use [`set_failed_s3_resources`](Self::set_failed_s3_resources).
        ///
        /// <p>S3 resources that couldn't be associated with Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
        pub fn failed_s3_resources(mut self, input: crate::model::FailedS3Resource) -> Self {
            let mut v = self.failed_s3_resources.unwrap_or_default();
            v.push(input);
            self.failed_s3_resources = Some(v);
            self
        }
        /// <p>S3 resources that couldn't be associated with Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
        pub fn set_failed_s3_resources(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FailedS3Resource>>,
        ) -> Self {
            self.failed_s3_resources = input;
            self
        }
        /// Consumes the builder and constructs a [`AssociateS3ResourcesOutput`](crate::output::AssociateS3ResourcesOutput)
        pub fn build(self) -> crate::output::AssociateS3ResourcesOutput {
            crate::output::AssociateS3ResourcesOutput {
                failed_s3_resources: self.failed_s3_resources,
            }
        }
    }
}
impl AssociateS3ResourcesOutput {
    /// Creates a new builder-style object to manufacture [`AssociateS3ResourcesOutput`](crate::output::AssociateS3ResourcesOutput)
    pub fn builder() -> crate::output::associate_s3_resources_output::Builder {
        crate::output::associate_s3_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AssociateMemberAccountOutput {}
impl std::fmt::Debug for AssociateMemberAccountOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AssociateMemberAccountOutput");
        formatter.finish()
    }
}
/// See [`AssociateMemberAccountOutput`](crate::output::AssociateMemberAccountOutput)
pub mod associate_member_account_output {
    /// A builder for [`AssociateMemberAccountOutput`](crate::output::AssociateMemberAccountOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AssociateMemberAccountOutput`](crate::output::AssociateMemberAccountOutput)
        pub fn build(self) -> crate::output::AssociateMemberAccountOutput {
            crate::output::AssociateMemberAccountOutput {}
        }
    }
}
impl AssociateMemberAccountOutput {
    /// Creates a new builder-style object to manufacture [`AssociateMemberAccountOutput`](crate::output::AssociateMemberAccountOutput)
    pub fn builder() -> crate::output::associate_member_account_output::Builder {
        crate::output::associate_member_account_output::Builder::default()
    }
}
