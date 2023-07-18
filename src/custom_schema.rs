diesel::table! {
    dolt_log (commit_hash) {
    commit_hash -> Text,
    committer -> Text,
    email -> Text,
    date -> Timestamp,
    message -> Text
    }
}

diesel::table! {
    dolt_branches (name, hash) {
    name -> Text,
    hash -> Text,
    latest_committer -> Text,
    latest_committer_email -> Text,
    latest_commit_date -> Timestamp,
    latest_commit_message -> Text,
    remote -> Text,
    branch -> Text
    }
}

diesel::table! {
    dolt_status (table_name) {
    table_name -> Text,
    staged -> Integer,
    status -> Text,
    }
}
