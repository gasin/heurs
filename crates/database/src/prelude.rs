//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.12

// Entity exports
pub use crate::entity::execution_results::{
    Entity as ExecutionResult, Model as ExecutionResultModel,
};
pub use crate::entity::submissions::{Entity as Submission, Model as SubmissionModel};
pub use crate::entity::test_cases::{Entity as TestCase, Model as TestCaseModel};

// Repository exports
pub use crate::repository::execution_result::ExecutionResultRepository;
pub use crate::repository::submission::SubmissionRepository;
pub use crate::repository::test_case::TestCaseRepository;
