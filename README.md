# create-mailgun-user

This is a simple application to create new mailgun users using the mailgun api. The primary use case for this would be to create users with shorter passwords. This is useful when using mailgun with older devices like printers/scanners.

You'll be prompted for your api key, domain, a username for the new user, and a password length. If everything was successful a username and password will be written to the standard output.

## Installation
### Download from GitHub
I use an M1 Mac so unless you also have an M1 mac the binary probably won't work on your device and you'll need to build from source. If you do have an M1 Mac just download the most recent release and run the binary.

### Build from source
- Install rust https://www.rust-lang.org/tools/install
- Clone this repo
- CD into the repo
- Run ```cargo build --release``` (and assuming there were no compile errors) then ```cargo run --release```
