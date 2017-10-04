# CECS 429 Group Project

## Members:

### Harold Agnote
### Braulio Flores
### David Van

#### Usage

Install Rust: https://www.rust-lang.org/en-US/install.html

Download Project using:

```
$ git clone https://github.com/DavidVan/CECS-429-Group-Project
```

Navigate to **CECS-429-Group-Project/search_engine/**

```
$ cd CECS-429-Group-Project/search_engine/
```

Run the following commands:

```
$ cargo install
$ cargo build --release
$ cargo run --release
```

#### Commands

**:q** or **:quit** - Quits Program

**:o** *FILE* or **:open** *FILE* - Opens a specified file in the current
working directory

**:s** *TERM* or **:stem** *TERM* - Normalizes and applies a stemmer on a term
before printing its result

**:i** *DIRECTORY* or **:index** *DIRECTORY* - Changes working directory to
specified directory and indexes files in new directory

**:v** or **:vocab** - Views vocabulary in index in sorted order

**:enable k** or **:enable kgram** - Enables K Gram Index when indexing
directories

**:disable k** or **:disable kgram** - Disables K Gram Index when indexing
directories

**:h** or **:help** - Displays list of commands
