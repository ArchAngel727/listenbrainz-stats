Rust program that pulls todays listen count from [listenbrainz](https://listenbrainz.com)

### Build
First clone the git repo and cd into it<br>
- To build the project run `cargo build` or `cargo build --release`.<br>
- To install the binary to the system (reccomended) run `cargo install --path=.`

### Config
inside of `~/.config/listenbrainz/` create two file `username` and `token`. Inside of username enter your listenbrainz username. Inside of the token file paste your listenbrainz token found at [here](https://listenbrainz.org/settings/)
