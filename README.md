<div align="center">
<img width="130px" style="border-radius: 100px" alt="Triox logo" src="./docs/assets/logo.svg" />
  <h1>Triox</h1>
  <p>
    <strong>
		Next Generation cloud server that is fast, reliable and secure.
	</strong>
  </p>

[![Build](https://github.com/Trioxidation/Triox/actions/workflows/rust.yml/badge.svg)](https://github.com/Trioxidation/Triox/actions/workflows/rust.yml)
![issues](https://img.shields.io/github/issues/Trioxidation/Triox?style=flat-square)
[![dependency status](https://deps.rs/repo/github/Trioxidation/Triox/status.svg)](https://deps.rs/repo/github/Trioxidation/Triox)
[![codecov](https://codecov.io/gh/Trioxidation/Triox/branch/master/graph/badge.svg?style=flat-square)](https://codecov.io/gh/Trioxidation/Triox)
<br />
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)
[![Chat](https://img.shields.io/badge/matrix-+triox:matrix.org-purple?style=flat-square)](https://matrix.to/#/+triox:matrix.org)

</div>

## Why Triox?

☘️ **Open Source** - We strongly believe in collaboration and
transparency.

⚡ **Speed** - Get the most out of your hardware! Triox runs fast, even
on weak hardware.

🔒 **Security** - We're using state-of-the-art algorithms and
authentication methods to protect your data.

⛓️ **Reliability** - Built on top of the strong guarantees of the [Rust
programming language](https://rust-lang.org).

🛫 **Easy Setup** - Triox comes with batteries included and is easy to
configure.

## Features

Triox is still in an early stage but is already usable. The features
we'd like to add before our first release can be found in [this
issue](https://github.com/Trioxidation/Triox/issues/17).

## Demo

### Hosted server

Sign in with username `demo_user` and password `demo_password`.

Sadly, we can't allow users to upload files due to legal restrictions.
Since we can't guarantee that no illegal data will be uploaded the demo
server runs in read-only mode.

**[triox-demo.aaron-erhardt.de](https://triox-demo.aaron-erhardt.de)**

## Self-hosted:

1. Clone the repository

`bash git clone https://github.com/Trioxidation/triox cd triox`

2. Build and start Triox

`bash docker-compose up -d --build`

Triox should be accessible at http://localhost:3000

## Contributing

Everyone is welcome to contribute to Triox. We are always open for new
ideas, features and improvements.

The easiest way to contribute changes is to fork Triox, and then create
a pull request to ask us to pull your changes into our repository. You
can find a list of good first issues
[here](https://github.com/Trioxidation/Triox/labels/good%20first%20issue).

## Setup

There are two ways to run Triox:

- With Docker
- Bare metal:

### Docker

1. Clone the repository

`bash git clone https://github.com/Trioxidation/triox cd triox`

2. Build docker image

`bash docker build -t triox/triox:latest .`

3. Run build docker image

`bash docker run triox/triox:latest`

### Bare metal

**This information is outdated** <strike>

1. Install Rust using [rustup](https://rustup.rs).
2. Install dependencies:

- pkg-config, common package name: `pkg-config`
- OpenSSL, common package name: `libssl-dev` or `openssl-devel`
- MySQL-client, common package name: `libmysqlclient-dev`,
  `libmariadb-dev-compat` or `mysql-devel`

3. Install a MySQL-server such as mariadb, common package name:
   `mariadb-server`

- Setup database (more below)
- [optional] setup SSL certificate for HTTPS

Now you should be ready to go! Use `cargo run` to compile and start the
server.

## Database setup

### Creating database user

````sql CREATE DATABASE triox; CREATE USER 'triox'@localhost IDENTIFIED
BY 'password'; GRANT ALL PRIVILEGES ON triox.* TO 'triox'@localhost;
FLUSH PRIVILEGES; ```

### Install diesel client

```bash cargo install diesel_cli --no-default-features --features mysql
````

### Add .env for diesel client

```bash echo DATABASE_URL=mysql://triox:password@localhost/triox > .env

```

### Run migrations

`bash diesel migration run`

## SSL setup

### Generating SSL key and certificate

````bash cd ssl openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem
-out cert.pem -days 365 cd .. ```

Then update `config/local.toml`:

```toml [ssl] enabled = true ```

</strike> # API Documentation

The API is documented in
[`API.md`](https://github.com/AaronErhardt/Triox/blob/master/API.md).
````
