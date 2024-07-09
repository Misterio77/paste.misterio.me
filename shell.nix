{ pkgs, ... }: let
  colors = ''
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'
    NC='\033[0m'
  '';
  project_help = pkgs.writeShellScriptBin "project_help" ''
    ${colors}
    echo -e $PURPLE"Next steps:"$NC
    echo -e "1. Start Postgres:   "$GREEN"pg_ctl start"$NC
    echo -e "2. Run migrations:   "$GREEN"cat server/db/*.sql | psql"$NC
    echo -e "3. Run app:          "$GREEN"cargo wr"$NC
    echo -e ""
    echo -e "Other available commands:"
    echo -e "- Build tool:        "$YELLOW"cargo"$NC
    echo -e "- Connect to DB:     "$BLUE"psql"$NC
    echo -e "- Manage DB:         "$BLUE"pg_ctl"$NC
    echo -e ""
    echo -e "- Show this message: "$PURPLE"project_help"$NC
    echo -e "- Re-init:           "$PURPLE"project_init"$NC
  '';
  project_init = pkgs.writeShellScriptBin "project_init" ''
    ${colors}
    echo -e $PURPLE"Initializing environment..."$NC

    # Postgres
    mkdir -p "$PGDATA"
    if [ -z "$(ls -A "$PGDATA")" ]; then
      pg_ctl init -o "-A trust" > /dev/null
      shuf -i 2000-65000 -n 1 > "$PGDATA/port"
      echo "unix_socket_directories = '$PGDATA'" >> "$PGDATA/postgresql.conf"
      echo -e $GREEN"Postgres initialized."$NC
    else
      echo -e $YELLOW"Postgres has already been initialized."$NC
      echo -e "Delete '$PGDATA' if you want to start from scratch."
    fi

    echo -e $PURPLE"Done."$NC
    echo -e ""
  '';

  server = pkgs.callPackage ./server { };
  cli = pkgs.callPackage ./cli { };
in
pkgs.mkShell {
  inherit (server) PICO_PATH TURBO_PATH SYNTAXES_PATH;
  inputsFrom = [ server cli ];
  buildInputs = [
    # Rust tooling
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.rustfmt
    pkgs.clippy
    pkgs.cargo-msrv
    pkgs.cargo-watch
    # Postgres tooling
    pkgs.postgresql
    pkgs.pgformatter
    pkgs.sqls
    # To generate secret
    pkgs.libressl
    # Scripts
    project_help
    project_init
  ];
  shellHook = ''
    export PGCOLOR="always"
    export PGDATA="$(pwd)/.database"
    project_init
    export PGHOST="$PGDATA"
    export PGPORT="$(cat "$PGDATA/port")"
    export PGDATABASE="postgres"
    export PGUSER="$USER"

    connection_string="postgresql://$PGUSER/$PGDATABASE?user=$PGUSER&host=$PGHOST&port=$PGPORT"
    export ROCKET_DATABASES="{database={url=\"$connection_string\"}}"
    export ROCKET_SECRET_KEY=$(openssl rand -base64 32)
    export ROCKET_TEMPLATE_DIR="$(pwd)/server/templates"

    project_help
  '';
}
