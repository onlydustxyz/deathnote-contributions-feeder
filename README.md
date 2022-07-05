<div align="center">
  <h1 align="center">deathnote-contributions-feeder</h1>
  <p align="center">
    <a href="https://discord.gg/onlydust">
        <img src="https://img.shields.io/badge/Discord-6666FF?style=for-the-badge&logo=discord&logoColor=white">
    </a>
    <a href="https://twitter.com/intent/follow?screen_name=onlydust_xyz">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white">
    </a>
    <a href="https://contributions.onlydust.xyz/">
        <img src="https://img.shields.io/badge/Contribute-6A1B9A?style=for-the-badge&logo=notion&logoColor=white">
    </a>
  </p>
  
  <h3 align="center">Death Note off chain service to watch github contributions and update associated smart contract state on StarkNet.</h3>

</h3>
</div>

> ## ⚠️ WARNING! ⚠️
>
> This repo contains highly experimental code.
> Expect rapid iteration.

## 🎟️ Description

## 🎗️ Prerequisites

### 1. Setup your environment 

Create the `.env` file with the correct environment variables.
Copy the `.env.example` file and modify the values according to your setup.

### 2. Start the docker container
Make sure `docker-compose` is installed (see [Installation instructions](https://docs.docker.com/compose/install/)).
```
docker-compose -f scripts/docker/dev/docker-compose.yml up -d
```

### 3. Setup the database
Make sure `Diesel CLI` is installed (see [installation instructions](https://diesel.rs/guides/getting-started)):

Then, use `Diesel` to initialize the data model and the database:
```
source .env
diesel setup
diesel migration run
```

## 📦 Installation

To build the project, run the following command:
```
cargo build
```

## 🔬 Usage

Below are some examples of usage.

### Add a single repository for indexing

```
# Offline mode
cargo run --bin add_project onlydustxyz starkonquest

# Online mode
cargo run &
curl -d '{"owner":"onlydustxyz", "name":"starkonquest"}' -H "Content-Type: application/json" -X POST http://localhost:8000/projects
```

### Run the indexer (fetch data from GitHub and store it in our database)
```
cargo run --bin index_projects
```

### Upload data (upload data from our database to our on-chain contract)
```
cargo run --bin upload_contributions
```

## 🌡️ Testing

```
cargo test
```

## 🫶 Contributing

## 📄 License

**deathnote-contributions-feeder** is released under the [MIT](LICENSE).
