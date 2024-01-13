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

- Create the database:

  ```bash
  cargo sqlx database create
  ```

- Apply the migrations

  ```bash
  cargo sqlx migrate run
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

## Progress

### Server

- [x] Authentication (JWT)
- [x] Register/Login
- [x] Create a todo list
- [x] Add a todo item
- [x] Remove a todo item
- [x] Mark a todo item as done
- [x] Mark a todo item as not done

### UI

- [x] Home page
- [x] Register/Login pages
- [x] Create a todo list
- [x] Add a todo item
- [x] Remove a todo item
- [x] Mark a todo item as done
- [x] Mark a todo item as not done

## Packages

- [Server](/packages/server/)
- [UI](/packages/ui/)

## License

This project is licensed under the MIT License.
