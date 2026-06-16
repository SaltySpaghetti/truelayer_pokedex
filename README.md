# TrueLayer Pokedex Challenge

## How to run the project

### With Docker

```bash
docker build -t truelayer_pokedex .
docker run -p 5000:5000 truelayer_pokedex
```

---

### Directly with Rust

```bash
cargo run
```

---

## Available endpoints

- `GET /pokemon/{pokemon_name}` - Gives you basic info about a pokemon
- `GET /pokemon/translated/{pokemon_name}` - Same as previous endpoint but with pokemon description translated using Shakespeare or Yoda way of speaking
