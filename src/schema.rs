// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Integer,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        start_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    employees_teams (id) {
        id -> Integer,
        employee_id -> Integer,
        team_id -> Integer,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(employees_teams -> employees (employee_id));
diesel::joinable!(employees_teams -> teams (team_id));

diesel::allow_tables_to_appear_in_same_query!(employees, employees_teams, teams,);
