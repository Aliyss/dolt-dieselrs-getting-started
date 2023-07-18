use crate::{
    custom_schema::{dolt_branches, dolt_log, dolt_status},
    schema::{employees, employees_teams, teams},
};
use diesel::prelude::*;
use tabled::Tabled;

fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(s) => format!("{}", s),
        None => "_".to_string(),
    }
}

#[derive(Debug, QueryableByName)]
pub struct ActiveBranch {
    #[diesel(column_name = "active_branch()", sql_type = diesel::sql_types::Text)]
    pub name: String,
}

#[derive(Debug, QueryableByName)]
pub struct Table {
    #[diesel(column_name = "Tables_in_dieselrs_big_demo", sql_type = diesel::sql_types::Text)]
    pub name: String,
}

#[derive(Debug, QueryableByName)]
pub struct DoltCallResponse {
    #[diesel(column_name = "status", sql_type = diesel::sql_types::Integer)]
    pub status: i32,
}

#[derive(Debug, QueryableByName)]
pub struct DoltCommitResponse {
    #[diesel(column_name = "hash", sql_type = diesel::sql_types::Text)]
    pub hash: String,
}

#[derive(Debug, QueryableByName)]
pub struct DoltMergeResponse {
    #[diesel(column_name = "hash", sql_type = diesel::sql_types::Text)]
    pub hash: String,
    #[diesel(column_name = "fast_forward", sql_type = diesel::sql_types::Integer)]
    pub fast_forward: i32,
    #[diesel(column_name = "conflicts", sql_type = diesel::sql_types::Integer)]
    pub conflicts: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = dolt_log)]
pub struct DoltLogEntry {
    pub commit_hash: String,
    pub committer: String,
    pub email: String,
    pub date: chrono::NaiveDateTime,
    pub message: String,
}

#[derive(Debug, QueryableByName, Tabled)]
pub struct DoltDiffEmployeesEntry {
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub to_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_last_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_last_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_first_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_first_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub to_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub from_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_last_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_last_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_first_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_first_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub from_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "diff_type", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub diff_type: Option<String>,
}

#[derive(Debug, QueryableByName, Tabled)]
pub struct DoltDiffEmployeesTeamsEntry {
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub to_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_employee_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub to_employee_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_team_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub to_team_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub to_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub from_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_employee_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub from_employee_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_team_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub from_team_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub from_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "diff_type", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub diff_type: Option<String>,
}

#[derive(Debug, QueryableByName, Tabled)]
pub struct DoltDiffEmployeesEntryAfterEdit {
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub to_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_last_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_last_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_first_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_first_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_start_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub to_start_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub to_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "to_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub to_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_id", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Integer>)]
    pub from_id: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_last_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_last_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_first_name", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_first_name: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_start_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub from_start_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub from_commit: Option<String>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "from_commit_date", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Timestamp>)]
    pub from_commit_date: Option<chrono::NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[diesel(column_name = "diff_type", sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub diff_type: Option<String>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = dolt_branches)]
pub struct DoltBranch {
    pub name: String,
    pub hash: String,
    pub latest_committer: String,
    pub latest_committer_email: String,
    pub latest_commit_date: chrono::NaiveDateTime,
    pub latest_commit_message: String,
    pub remote: String,
    pub branch: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = dolt_status)]
pub struct DoltStatus {
    pub table_name: String,
    pub staged: i32,
    pub status: String,
}

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = employees)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = employees)]
pub struct EmployeeAfterEdit {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub start_date: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
#[diesel(table_name = employees_teams)]
pub struct EmployeeTeam {
    pub id: i32,
    pub employee_id: i32,
    pub team_id: i32,
}

#[derive(Debug, PartialEq, Queryable, Selectable)]
pub struct TeamEmployee {
    #[diesel(select_expression = teams::columns::name)]
    #[diesel(select_expression_type = teams::columns::name)]
    pub team_name: Option<String>,
    #[diesel(select_expression = employees::columns::first_name)]
    #[diesel(select_expression_type = employees::columns::first_name)]
    pub first_name: Option<String>,
    #[diesel(select_expression = employees::columns::last_name)]
    #[diesel(select_expression_type = employees::columns::last_name)]
    pub last_name: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Selectable)]
pub struct TeamEmployeeAfterEdit {
    #[diesel(select_expression = teams::columns::name)]
    #[diesel(select_expression_type = teams::columns::name)]
    pub team_name: Option<String>,
    #[diesel(select_expression = employees::columns::first_name)]
    #[diesel(select_expression_type = employees::columns::first_name)]
    pub first_name: Option<String>,
    #[diesel(select_expression = employees::columns::last_name)]
    #[diesel(select_expression_type = employees::columns::last_name)]
    pub last_name: Option<String>,
    #[diesel(select_expression = employees::columns::start_date)]
    #[diesel(select_expression_type = employees::columns::start_date)]
    pub start_date: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Queryable, QueryableByName)]
pub struct MappedEmployeeTeamWithStartDate {
    #[diesel(column_name = "name", sql_type = diesel::sql_types::Text)]
    pub team_name: Option<String>,
    #[diesel(column_name = "first_name", sql_type = diesel::sql_types::Text)]
    pub employee_first_name: Option<String>,
    #[diesel(column_name = "last_name", sql_type = diesel::sql_types::Text)]
    pub employee_last_name: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = employees_teams)]
pub struct EmployeeTeamInsertable {
    pub employee_id: i32,
    pub team_id: i32,
}
