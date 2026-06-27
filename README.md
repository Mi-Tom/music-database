# Rust Music Database
> [!NOTE]
> Czech version is available below / Česká verze je k dispozici níže

## What is the purpose of this project?
The program is a CLI music database. Each song features a name and an artist, with optional fields for the album, duration, and release year. The project serves primarily as a way to learn the Rust programming language.

## Technologies Used
- the program is written in Rust language
- you can find dependencies in [Cargo.toml](Cargo.toml)
 
> [!IMPORTANT]
> Prerequisites
> - git
> - cargo

## Usage 

### 1. Create a clone of this repository
```bash
git clone https://github.com/Mi-Tom/music-database.git
```
> [!TIP]
> You can do the same via ssh or github CLI

### 2. Compile the code
```bash
cd music-database
cargo build
```
> [!TIP]
> you can add '--release' tag for better optimization

### 3. Now you can run the program
 ```bash
# if you run "cargo build"
./target/debug/music-database 
# or 
./target/release/music-database
# if you run "cargo build --release"
 ```

---
# Rust Music Database
> [!NOTE]
> Česká verze / Czech Version

## K čemu projekt slouží?
Program je CLI s databází hudebních skladeb, každá skladba má jméno a autora, dále volitelně z jakého je alba, jak je dlouhá a v jakém roce byla vydána. Projekt slouží primárně k naučení se jazyka rust.

## Použité technologie
- program je napsán pomocí jazyka Rust
- použité knihovny lze nalézt v dependencies v [Cargo.toml](Cargo.toml)
 
> [!IMPORTANT]
> Prerekvizity
> - git
> - cargo

## Spuštění

### 1. Stáhněte tento repozitář
```bash
git clone https://github.com/Mi-Tom/music-database.git
```
> [!TIP]
> toho samého lze dosáhnout přes ssh metdu a github CLI

### 2. Zkompilujte kód
```bast
cd music-database
cargo build
```
> [!TIP]
> můžete přidat '--release' přepínač pro lepší optimalizaci

### 3. Teď už stačí jen program spustit
 ```bash
# pokud jste kompilovali pomocí "cargo build"
./target/debug/music-database 
# nebo
./target/release/music-database
# při "cargo build --release"
 ```
