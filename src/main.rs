mod custom_schema;
mod models;
mod schema;

use dotenv::dotenv;
use std::env;

use diesel::{
    result::Error, sql_query, Connection, ExpressionMethods, JoinOnDsl, MysqlConnection, QueryDsl,
    RunQueryDsl, SelectableHelper,
};

use custom_schema::{dolt_branches, dolt_log, dolt_status};
use schema::{
    employees::dsl as employees, employees_teams::dsl as employees_teams, teams::dsl as teams,
};

use models::{
    ActiveBranch, DoltCallResponse, DoltCommitResponse, DoltDiffEmployeesEntry,
    DoltDiffEmployeesTeamsEntry, DoltMergeResponse, Employee, EmployeeAfterEdit,
    EmployeeTeamInsertable, Table, Team, TeamEmployee, TeamEmployeeAfterEdit,
};

use crate::models::DoltDiffEmployeesEntryAfterEdit;

pub fn establish_connection(database_url: String) -> MysqlConnection {
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn main() {
    let mut engine = dolt_checkout("main");
    print_active_branch(&mut engine);

    // Start fresh so we can re-run this script.
    reset_database(&mut engine);
    delete_non_main_branches(&mut engine);

    // Build our tables
    setup_database(&mut engine);
    print_tables(&mut engine);

    // Our first Dolt feature. This will commit the first time
    // But after that nothing has changed so there is nothing to commit.
    dolt_commit(&mut engine, "Tim <tim@dolthub.com>", "Created tables");

    // Examine a Dolt system table: dolt_log
    print_commit_log(&mut engine);

    // Load rows into the tables
    insert_data(&mut engine);

    print_summary_table(&mut engine, false);

    // Show off dolt_status and dolt_diff
    print_status(&mut engine);
    print_diff(&mut engine, "employees", false);

    // Dolt commit our changes
    dolt_commit(
        &mut engine,
        "Aaron <aaron@dolthub.com>",
        "Inserted data into tables",
    );

    print_commit_log(&mut engine);

    // Show off dolt_reset
    drop_table(&mut engine, "employees_teams");
    print_status(&mut engine);
    print_tables(&mut engine);
    dolt_reset_hard(&mut engine, None);
    print_status(&mut engine);
    print_tables(&mut engine);

    // Show off branch and merge
    dolt_create_branch(&mut engine, "modify_data");
    engine = dolt_checkout("modify_data");
    modify_data(&mut engine);
    print_status(&mut engine);
    print_diff(&mut engine, "employees", false);
    print_diff(&mut engine, "employees_teams", false);
    print_summary_table(&mut engine, false);
    dolt_commit(
        &mut engine,
        "Brian <brian@dolthub.com>",
        "Modified data on branch",
    );
    print_commit_log(&mut engine);

    // Switch back to main because I want the same merge base
    engine = dolt_checkout("main");
    dolt_create_branch(&mut engine, "modify_schema");
    engine = dolt_checkout("modify_schema");
    print_active_branch(&mut engine);
    modify_schema(&mut engine);
    print_status(&mut engine);
    print_diff(&mut engine, "employees", true);
    print_summary_table(&mut engine, true);
    dolt_commit(
        &mut engine,
        "Tim <tim@dolthub.com>",
        "Modified schema on branch",
    );
    print_commit_log(&mut engine);

    // Show off merge
    engine = dolt_checkout("main");
    print_active_branch(&mut engine);
    print_commit_log(&mut engine);
    print_summary_table(&mut engine, false);
    dolt_merge(&mut engine, "modify_data");
    print_summary_table(&mut engine, false);
    print_commit_log(&mut engine);
    dolt_merge(&mut engine, "modify_schema");
    print_commit_log(&mut engine);
    print_summary_table(&mut engine, true);
}

pub fn reset_database(conn: &mut MysqlConnection) {
    // Here we find the first commit in the log and reset to that commit

    let results = dolt_log::table
        .select(dolt_log::commit_hash)
        .order(dolt_log::date.asc())
        .limit(1)
        .load::<String>(conn)
        .expect("Error executing query at: 'reset_database'");

    dolt_reset_hard(conn, results.first());
}

pub fn delete_non_main_branches(conn: &mut MysqlConnection) {
    /*
    Iterate through the non-main branches and delete them with
    CALL DOLT_BRANCH('-D', '<branch>'). '-D' force deletes just in
    case I have some unmerged modifications from a failed run.
    */

    let results = dolt_branches::table
        .select(dolt_branches::name)
        .filter(dolt_branches::name.ne("main"))
        .load::<String>(conn)
        .expect("Error executing query at: 'delete_non_main_branches'");

    for branch in results {
        println!("Deleting branch: {branch}");
        let stmt = format!("CALL DOLT_BRANCH('-D', '{branch}')");
        sql_query(stmt)
            .load::<DoltCallResponse>(conn)
            .expect("Error executing query at: 'delete_non_main_branches'");
    }
}

pub fn setup_database(conn: &mut MysqlConnection) {
    /*
    CREATE is not standard for diesel.rs
    Usually Tables are not created dynamically with diesel.rs
    diesel.rs takes pride in being correct at compile time.

    This is only to stay more or less true to the original python demo.
    */

    let stmt_employees = "CREATE TABLE employees(
       id INTEGER PRIMARY KEY,
       last_name VARCHAR(255),
       first_name VARCHAR(255)
    )"
    .to_string();

    let stmt_teams = "CREATE TABLE teams(
       id INTEGER PRIMARY KEY,
       name VARCHAR(255)
    )"
    .to_string();

    // diesel.rs dislikes tables without a primary key, so adding an auto_incrementing one.
    let stmt_employees_teams = "CREATE TABLE employees_teams(
       id INTEGER PRIMARY KEY AUTO_INCREMENT,
       employee_id INTEGER NOT NULL, 
       team_id INTEGER NOT NULL, 
       FOREIGN KEY (employee_id) REFERENCES employees(id),
       FOREIGN KEY (team_id) REFERENCES teams(id)
    )"
    .to_string();

    let _result_employees = sql_query(stmt_employees).execute(conn);
    let _result_teams = sql_query(stmt_teams).execute(conn);
    let _result_employees_teams = sql_query(stmt_employees_teams).execute(conn);
}

pub fn insert_data(conn: &mut MysqlConnection) {
    let _result_employees = conn.transaction::<_, Error, _>(|conn| {
        let employee_values: Vec<Employee> = vec![
            Employee {
                id: 0,
                first_name: "Tim".to_string(),
                last_name: "Sehn".to_string(),
            },
            Employee {
                id: 1,
                first_name: "Brian".to_string(),
                last_name: "Hendricks".to_string(),
            },
            Employee {
                id: 2,
                first_name: "Aaron".to_string(),
                last_name: "Son".to_string(),
            },
            Employee {
                id: 3,
                first_name: "Brian".to_string(),
                last_name: "Fitzgerald".to_string(),
            },
            Employee {
                id: 5,
                first_name: "Aliyss".to_string(),
                last_name: "Snow".to_string(),
            },
        ];

        diesel::insert_into(employees::employees)
            .values(&employee_values)
            .execute(conn)?;

        Ok(())
    });

    let _result_teams = conn.transaction::<_, Error, _>(|conn| {
        let team_values: Vec<Team> = vec![
            Team {
                id: 0,
                name: "Engineering".to_string(),
            },
            Team {
                id: 1,
                name: "Sales".to_string(),
            },
        ];

        diesel::insert_into(teams::teams)
            .values(&team_values)
            .execute(conn)?;

        Ok(())
    });

    let _result_employees_teams = conn.transaction::<_, Error, _>(|conn| {
        let employee_team_values: Vec<EmployeeTeamInsertable> = vec![
            EmployeeTeamInsertable {
                employee_id: 0,
                team_id: 0,
            },
            EmployeeTeamInsertable {
                employee_id: 1,
                team_id: 0,
            },
            EmployeeTeamInsertable {
                employee_id: 2,
                team_id: 0,
            },
            EmployeeTeamInsertable {
                employee_id: 0,
                team_id: 1,
            },
            EmployeeTeamInsertable {
                employee_id: 3,
                team_id: 1,
            },
            EmployeeTeamInsertable {
                employee_id: 5,
                team_id: 0,
            },
        ];

        diesel::insert_into(employees_teams::employees_teams)
            .values(&employee_team_values)
            .execute(conn)?;

        Ok(())
    });
}

pub fn modify_data(conn: &mut MysqlConnection) {
    let _result_update_employee = conn.transaction::<_, Error, _>(|conn| {
        diesel::update(employees::employees.filter(employees::first_name.eq("Tim")))
            .set(employees::first_name.eq("Timothy"))
            .execute(conn)?;

        Ok(())
    });

    let _result_insert_employee = conn.transaction::<_, Error, _>(|conn| {
        diesel::insert_into(employees::employees)
            .values(Employee {
                id: 4,
                first_name: "Daylon".to_string(),
                last_name: "Wilkins".to_string(),
            })
            .execute(conn)?;

        Ok(())
    });

    let _result_insert_employee_team = conn.transaction::<_, Error, _>(|conn| {
        diesel::insert_into(employees_teams::employees_teams)
            .values(EmployeeTeamInsertable {
                employee_id: 4,
                team_id: 0,
            })
            .execute(conn)?;

        Ok(())
    });

    let _result_insert_employee_team = conn.transaction::<_, Error, _>(|conn| {
        diesel::delete(
            employees_teams::employees_teams
                .filter(employees_teams::employee_id.eq(0))
                .filter(employees_teams::team_id.eq(1)),
        )
        .execute(conn)?;

        Ok(())
    });
}

pub fn modify_schema(conn: &mut MysqlConnection) {
    /*
    ALTER is not standard for diesel.rs

    This is only to stay more or less true to the original python demo.
    */

    let stmt = "ALTER TABLE employees add column start_date date".to_string();
    let _altered_employees = conn.transaction::<_, Error, _>(|conn| {
        sql_query(stmt).execute(conn)?;
        Ok(())
    });

    let _result_update_employee = conn.transaction::<_, Error, _>(|conn| {
        diesel::update(employees::employees.filter(employees::id.eq(0)))
            .set(
                employees::start_date.eq(chrono::NaiveDate::from_ymd_opt(2018, 8, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()),
            )
            .execute(conn)?;

        diesel::update(employees::employees.filter(employees::id.eq(1)))
            .set(
                employees::start_date.eq(chrono::NaiveDate::from_ymd_opt(2018, 8, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()),
            )
            .execute(conn)?;

        diesel::update(employees::employees.filter(employees::id.eq(2)))
            .set(
                employees::start_date.eq(chrono::NaiveDate::from_ymd_opt(2018, 8, 6)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()),
            )
            .execute(conn)?;

        diesel::update(employees::employees.filter(employees::id.eq(5)))
            .set(
                employees::start_date.eq(chrono::NaiveDate::from_ymd_opt(2023, 7, 4)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()),
            )
            .execute(conn)?;

        let target_employees = employees::employees
            .filter(employees::last_name.eq("Fitzgerald"))
            .limit(1)
            .load::<EmployeeAfterEdit>(conn)
            .expect("Error loading employees");

        if let Some(target_employee) = target_employees.first() {
            diesel::update(employees::employees.filter(employees::id.eq(target_employee.id)))
                .set(
                    employees::start_date.eq(chrono::NaiveDate::from_ymd_opt(2021, 4, 19)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap()),
                )
                .execute(conn)?;
        }

        Ok(())
    });
}

pub fn drop_table(conn: &mut MysqlConnection, table: &str) {
    /*
    DROP is not standard for diesel.rs

    This is only to stay more or less true to the original python demo.
    */

    let table_name = match table {
        "employees" | "teams" | "employees_teams" => table,
        _ => {
            println!("{table}: Not found");
            return;
        }
    };

    let stmt = format!("DROP TABLE {table_name}");
    let _dropped = sql_query(stmt).execute(conn);
}

pub fn dolt_commit(conn: &mut MysqlConnection, author: &str, message: &str) {
    /*
    Dolt exposes version control writes as procedures
    Here, we use text to execute procedures.

    The other option (in python) is to do something like (Aliyss couldn't be bothered to research the alternative in rust):
    -- conn = engine.raw_connection()
    -- results = conn.cursor().callproc('dolt_commit', arguments)
    -- conn.close()

    I like the text approach better.
    */

    let stmt_add = "CALL DOLT_ADD('-A')".to_string();
    sql_query(stmt_add)
        .load::<DoltCallResponse>(conn)
        .expect("Error executing query at: 'dolt_commit DOLT_ADD'");

    let stmt_commit =
        format!("CALL DOLT_COMMIT('--skip-empty', '--author', '{author}', '-m', '{message}')");
    let result_commit = sql_query(stmt_commit)
        .load::<DoltCommitResponse>(conn)
        .expect("Error executing query at: 'dolt_commit DOLT_COMMIT'");

    if let Some(commit) = result_commit.first() {
        println!("Created commit: {}", &commit.hash);
    }
}

pub fn dolt_reset_hard(conn: &mut MysqlConnection, commit: Option<&String>) {
    let mut stmt = "CALL DOLT_RESET('--hard')".to_string();
    if let Some(commit_hash) = commit {
        stmt = format!("CALL DOLT_RESET('--hard', '{commit_hash}')");
        println!("Resetting to commit: {commit_hash}")
    } else {
        println!("Resetting to HEAD")
    }

    let _ = conn.transaction::<_, Error, _>(|conn| {
        sql_query(stmt)
            .load::<DoltCallResponse>(conn)
            .expect("Error executing query at: 'dolt_reset_hard'");

        Ok(())
    });
}

pub fn dolt_create_branch(conn: &mut MysqlConnection, branch: &str) {
    // Check if branch exists
    let results = dolt_branches::table
        .select(dolt_branches::name)
        .filter(dolt_branches::name.eq(branch))
        .load::<String>(conn)
        .expect("Error executing query at: 'delete_non_main_branches'");

    if !results.is_empty() {
        println!("Branch exists: {branch}");
        return;
    }

    // Create branch
    let stmt = format!("CALL DOLT_BRANCH('{branch}')");
    let result_branch = sql_query(stmt)
        .load::<DoltCallResponse>(conn)
        .expect("Error executing query at: 'dolt_commit DOLT_COMMIT'");

    if let Some(_status) = result_branch.first() {
        println!("Created branch: {}", branch);
    }
}

pub fn dolt_checkout(branch: &str) -> MysqlConnection {
    dotenv().ok();
    let engine_base = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    /*
    Using a Connection instead of an engine here.
    Couldn't connect by adding 'main' to the url.
    Using DOLT_CHECKOUT instead.
    */

    let mut conn = establish_connection(engine_base.to_string());

    let stmt = format!("CALL DOLT_CHECKOUT('{branch}')");
    sql_query(stmt)
        .load::<DoltCallResponse>(&mut conn)
        .expect("Error executing query at: 'dolt_checkout'");

    conn
}

pub fn dolt_merge(conn: &mut MysqlConnection, branch: &str) {
    let stmt = format!("CALL DOLT_MERGE('{branch}')");
    let result = sql_query(stmt)
        .load::<DoltMergeResponse>(conn)
        .expect("Error executing query at: 'dolt_merge'");

    println!("Merge Complete: {branch}");
    if let Some(merge) = result.first() {
        println!("\tCommit: {}", &merge.hash);
        println!("\tFast Forward: {}", &merge.fast_forward);
        println!("\tConflicts: {}", &merge.conflicts);
    }
}

pub fn print_commit_log(conn: &mut MysqlConnection) {
    // Examine a dolt system table, dolt_log, using reflection

    println!("Commit Log:");

    let results = dolt_log::table
        .select((
            dolt_log::commit_hash,
            dolt_log::committer,
            dolt_log::message,
        ))
        .order(dolt_log::date.desc())
        .load::<(String, String, String)>(conn)
        .expect("Error executing query at: 'print_commit_log'");

    for commit in results {
        println!("\t{}: {} by {}", commit.0, commit.2, commit.1);
    }
}

pub fn print_status(conn: &mut MysqlConnection) {
    println!("Status");

    let results = dolt_status::table
        .select((dolt_status::table_name, dolt_status::status))
        .load::<(String, String)>(conn)
        .expect("Error executing query at: 'print_status'");

    match results.len() {
        0 => println!("\tNo tables modified"),
        _ => {
            for status in results {
                println!("\t{}: {}", status.0, status.1);
            }
        }
    }
}

pub fn print_active_branch(conn: &mut MysqlConnection) {
    let stmt = "SELECT active_branch()";
    let result: Vec<ActiveBranch> = sql_query(stmt)
        .load(conn)
        .expect("Error executing query at: 'print_active_branch'");

    if let Some(active_branch) = result.first() {
        println!("Active Branch: {}", &active_branch.name);
    }
}

pub fn print_diff(conn: &mut MysqlConnection, table: &str, with_start_date: bool) {
    println!("Diffing table: {table}");

    // Show only working set changes
    let stmt = format!("SELECT * from dolt_diff_{table} WHERE to_commit = 'WORKING'");

    match table {
        "employees" => {
            if !with_start_date {
                let results = sql_query(stmt)
                    .load::<DoltDiffEmployeesEntry>(conn)
                    .expect("Error executing query at: 'print_diff'");
                print_as_table(results);
            } else {
                let results = sql_query(stmt)
                    .load::<DoltDiffEmployeesEntryAfterEdit>(conn)
                    .expect("Error executing query at: 'print_diff'");
                print_as_table(results);
            }
        }
        "employees_teams" => {
            let results = sql_query(stmt)
                .load::<DoltDiffEmployeesTeamsEntry>(conn)
                .expect("Error executing query at: 'print_diff'");
            print_as_table(results);
        }
        _ => panic!("Unknown Table"),
    };
}

pub fn print_as_table<T: tabled::Tabled>(table_entries: Vec<T>) {
    // I am using tabled here because dolt_diff_<table> is a wide table
    let results_table = tabled::tables::ExtendedTable::new(table_entries)
        .to_string()
        .replace('\n', "\n\t");

    println!("\t{results_table}");
}

pub fn print_tables(conn: &mut MysqlConnection) {
    // Raw SQL here to show what we've done

    let stmt = "SHOW tables".to_string();
    let result = sql_query(stmt)
        .load::<Table>(conn)
        .expect("Error executing query at: 'print_tables'");

    println!("Tables in database: ");

    for table in result {
        println!("\t{}", table.name);
    }
}

pub fn print_summary_table(conn: &mut MysqlConnection, with_start_date: bool) {
    println!("Team Summary");

    // Dolt supports up to 12 table joins. Here we do a 3 table join.
    let query_build = employees::employees
        .inner_join(
            employees_teams::employees_teams.on(employees::id.eq(employees_teams::employee_id)),
        )
        .inner_join(teams::teams.on(teams::id.eq(employees_teams::team_id)))
        .order(teams::name.asc());

    match with_start_date {
        true => {
            let result = query_build
                .select(TeamEmployeeAfterEdit::as_select())
                .load::<TeamEmployeeAfterEdit>(conn)
                .expect("Error executing query at: 'print_summary_table'");

            for employee_team in result {
                let mut output = format!(
                    "\t{}: {} {}",
                    employee_team.team_name.unwrap_or("".to_string()),
                    employee_team.first_name.unwrap_or("".to_string()),
                    employee_team.last_name.unwrap_or("".to_string())
                );

                if let Some(start_date) = employee_team.start_date {
                    output += &format!(" {}", start_date.format("%Y-%m-%d"))
                }

                println!("{output}");
            }
        }
        false => {
            let result = query_build
                .select(TeamEmployee::as_select())
                .load::<TeamEmployee>(conn)
                .expect("Error executing query at: 'print_summary_table'");

            for employee_team in result {
                let output = format!(
                    "\t{}: {} {}",
                    employee_team.team_name.unwrap_or("".to_string()),
                    employee_team.first_name.unwrap_or("".to_string()),
                    employee_team.last_name.unwrap_or("".to_string())
                );

                println!("{output}");
            }
        }
    };
}
