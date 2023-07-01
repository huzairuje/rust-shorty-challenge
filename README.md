# Rust - Sample API for shorty challenge (tiny-url)
## Prerequisite
1. rust build nightly using rustup chain command.
    a. install rust with this command (Unix like or linux)
    ```shell
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    b. install rust nightly (version this 1.69 rust stable)
    ```shell
      rustup install stable
    ```
    c. set the rust nightly installation set to default 
    ```shell
      rustup default stable
    ```
    d. install the crate by this command
    ```shell
      make install
    ```
    or use this command
    ```shell
     # HotReload
      cargo install cargo-watch
    ```
2. make `.env` file on the root project folder
3. setup the env like port or host, etc.
4. compile the web app
   ```shell
      cargo build
    ```
5. running the web app service
    ```shell
      cargo run 
    ```
   or prefer the release
   ```shell
      cargo run --release
   ```
    
