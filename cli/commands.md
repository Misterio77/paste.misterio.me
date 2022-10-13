pmis <command>
    --api <url>
    Defaults to https://paste.misterio.me


pmis list [owner]
    Requires authentication to see unlisted pastes.
    Defaults to own user, if authenticated.
    Aliased to pmis ls and pmis l

pmis upload [file]
    -t --title <title>
    -d --description <description>
    -u --unlisted
    Reads from STDIN if file is empty or -
    Requires authentication
    Aliased to pmis up and pmis u

pmis download <id>
    -r --raw
    Aliased to pmis down and pmis d

pmis delete <id>
    Requires authentication
    Aliased to pmis del

pmis user [username]
    Defaults to own user, if authenticated.


pmis auth <token>
    Reads from STDIN if file is empty or -
