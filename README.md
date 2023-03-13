# cmdw

`cmdw` is a CLI tool to execute an arbitrary command and to reveal the result of the executed command via HTTP. It can be useful when you want someone to execute a command in your local environment.

## Usage

```shell
cargo run -- --command ls --addr 127.0.0.1 --port 8082
```

Then you'll see result like the below:

```shell
curl "http://localhost:8082/?args=-l+-a+-h"
total 96
drwxr-xr-x   9 antipop  staff   288B Mar 13 19:40 .
drwxr-xr-x  15 antipop  staff   480B Mar 13 17:38 ..
drwxr-xr-x   9 antipop  staff   288B Mar 13 19:40 .git
-rw-r--r--   1 antipop  staff     8B Mar 13 17:38 .gitignore
-rw-r--r--   1 antipop  staff    34K Mar 13 19:27 Cargo.lock
-rw-r--r--   1 antipop  staff   312B Mar 13 19:27 Cargo.toml
-rw-r--r--   1 antipop  staff   301B Mar 13 19:46 README.md
drwxr-xr-x   3 antipop  staff    96B Mar 13 17:38 src
drwxr-xr-x@  5 antipop  staff   160B Mar 13 17:40 target⏎
```

## Author

Kentaro Kuribayashi &lt;kentarok@gmail.com&gt;
