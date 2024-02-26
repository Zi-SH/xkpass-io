# xkpass.io - xkcd-inspired passphrase via curl

Inspired by [xkpasswd.net](https://github.com/bartificer/xkpasswd-js), known for generating secure but memorable passwords, this process is intended to do the same but via curl and terminal. Think icanhazip.com or ifconfig.me, but for password generation.

This project is running at https://xkpass.io for easy querying.

## Installing
    $ cargo build --release


## Usage

Simply run the binary and the process will listen on http://127.0.0.1:8080
It is heavily suggested that the service be run behind a reverse proxy that exposes TLS.

## Credit

Word list derived from the work of Josh Kaufman
https://github.com/first20hours/google-10000-english

List can be recreated by the following command (some entries were removed by hand):
curl https://raw.githubusercontent.com/first20hours/google-10000-english/master/google-10000-english-no-swears.txt | awk 'length($0) > 3' | sort
