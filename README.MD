# Next Rust Todo list app

<!-- Logo centered -->

<p align="center">
  <img src="packages/ui/public/logo.png" alt="Next Rust Todo list app" width="300" style="border-radius: 30px;" />

</p>

This project is a simple todo list app written in Rust. It is a learning project for me to learn Rust.

## How to run

- Install the `sqlx-cli` for postgres:

  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

- Create a `.env` file in the `packages/server` folder and add the following:

  ```bash
  DATABASE_URL="postgresql://postgres:password@localhost:5432/next-rust-todolist"
  ```

  Replace the `postgres` and `password` with your own postgres username and password and choose a database name of your choice.

- Run a postgres server, the easiest way is to use docker:

  ```bash
  docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d next-rust-todolist
  ```

- Create the database:

  ```bash
  pnpm db:create
  ```

- Apply the migrations

  ```bash
  pnpm migrate:run
  ```

- Run the app:

  ```bash
  pnpm dev
  ```

- Run server only (optional)

  ```bash
  pnpm dev:server
  ```

- Run UI only (optional)

  ```bash
  pnpm dev:ui
  ```

- Create a migration

  ```bash
  pnpm migrate:create "<migration-name>"
  ```

> Note: Sqlx requires the database to be running when compiling the Rust server, so make sure to run the postgres server before running `pnpm dev` or `pnpm dev:server`. To read more about sqlx and how to use it, check out the [sqlx docs](https://github.com/launchbadge/sqlx).

## Progress

### Server

- [x] Authentication (JWT)
- [x] Register/Login
- [ ] Create a todo list
- [ ] Add a todo item
- [ ] Remove a todo item
- [ ] Mark a todo item as done
- [ ] Mark a todo item as not done

### UI

- [ ] Home page
- [ ] Register/Login pages
- [ ] Create a todo list
- [ ] Add a todo item
- [ ] Remove a todo item
- [ ] Mark a todo item as done
- [ ] Mark a todo item as not done

## Packages

- [Server](/packages/server/)
- [UI](/packages/ui/)

## License

This project is licensed under the MIT License.
