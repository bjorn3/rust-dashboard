use chrono::NaiveDateTime;
use diesel::ExpressionMethods;

use super::schema::*;

#[derive(Clone, Debug, Eq, Ord, Insertable, PartialEq, PartialOrd)]
#[table_name="fcp_proposal"]
pub struct NewFcpProposal<'a> {
    pub fk_issue: i32,
    pub fk_initiator: i32,
    pub fk_initiating_comment: i32,
    pub disposition: &'a str,
    pub fk_bot_tracking_comment: i32,
    pub fcp_start: Option<NaiveDateTime>,
    pub fcp_closed: bool,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Eq, Ord,
         PartialEq, PartialOrd, Queryable, Serialize)]
#[table_name="fcp_proposal"]
pub struct FcpProposal {
    pub id: i32,
    pub fk_issue: i32,
    pub fk_initiator: i32,
    pub fk_initiating_comment: i32,
    pub disposition: String,
    pub fk_bot_tracking_comment: i32,
    pub fcp_start: Option<NaiveDateTime>,
    pub fcp_closed: bool,
}

#[derive(Clone, Debug, Eq, Insertable, Ord, PartialEq, PartialOrd, Serialize)]
#[table_name="fcp_review_request"]
pub struct NewFcpReviewRequest {
    pub fk_proposal: i32,
    pub fk_reviewer: i32,
    pub reviewed: bool,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Queryable, Serialize)]
#[table_name="fcp_review_request"]
pub struct FcpReviewRequest {
    pub id: i32,
    pub fk_proposal: i32,
    pub fk_reviewer: i32,
    pub reviewed: bool,
}

#[derive(Clone, Debug, Eq, Insertable, Ord, PartialEq, PartialOrd)]
#[table_name="fcp_concern"]
pub struct NewFcpConcern<'a> {
    pub fk_proposal: i32,
    pub fk_initiator: i32,
    pub fk_resolved_comment: Option<i32>,
    pub name: &'a str,
    pub fk_initiating_comment: i32,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Queryable)]
#[table_name="fcp_concern"]
pub struct FcpConcern {
    pub id: i32,
    pub fk_proposal: i32,
    pub fk_initiator: i32,
    pub fk_resolved_comment: Option<i32>,
    pub name: String,
    pub fk_initiating_comment: i32,
}

#[derive(Clone, Debug, Eq, Insertable, Ord, PartialEq, PartialOrd)]
#[table_name="rfc_feedback_request"]
pub struct NewFeedbackRequest {
    pub fk_initiator: i32,
    pub fk_requested: i32,
    pub fk_issue: i32,
    pub fk_feedback_comment: Option<i32>,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Queryable)]
#[table_name="rfc_feedback_request"]
pub struct FeedbackRequest {
    pub id: i32,
    pub fk_initiator: i32,
    pub fk_requested: i32,
    pub fk_issue: i32,
    pub fk_feedback_comment: Option<i32>,
}
