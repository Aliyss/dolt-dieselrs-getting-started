# dolt-dieselrs-getting-started

A Getting Started demo of [Dolt](https://www.doltdb.com) and [Diesel](https://github.com/diesel-rs/diesel.rs)

## dolt

### Initialize Dolt
Navigate to your path of preference and execute the following commands.

```sh
mkdir dieselrs_big_demo
cd dieselrs_big_demo 
dolt init --fun
```

**Result:**
```sh
Successfully initialized dolt data repository.
```

### Start SQL-Server
Start the sql server with dolt.

```sh
dolt sql-server
```

**Result:**
```sh
Starting server with Config HP="localhost:3306"|T="28800000"|R="false"|L="info"|S="/tmp/mysql.sock"
2023-07-19T10:31:42-08:00 INFO [no conn] Server ready. Accepting connections. {}
```

## dolt-dieselrs-getting-started

### Setup
Navigate to your path of preference and execute the following commands.

```sh
git clone git@github.com:aliyss/dolt-dieselrs-getting-started.git
cd dolt-dieselrs-getting-started
```

**Result:**
```sh
remote: Enumerating objects: 69, done.
remote: Counting objects: 100% (69/69), done.
remote: Compressing objects: 100% (42/42), done.
...
```

### Environment Variables
Make sure the environment variables in the ``.env`` file are correct, based on your setup.

### Run
Make sure you are in the correct directory.

```sh
cargo run
```

