# Command-Line Help for `aegeri`

This document contains the help content for the `aegeri` command-line program.

**Command Overview:**

* [`aegeri`↴](#aegeri)
* [`aegeri markdown`↴](#aegeri-markdown)
* [`aegeri markdown generate`↴](#aegeri-markdown-generate)
* [`aegeri markdown file`↴](#aegeri-markdown-file)
* [`aegeri markdown print`↴](#aegeri-markdown-print)
* [`aegeri markdown workspace`↴](#aegeri-markdown-workspace)
* [`aegeri local-cluster`↴](#aegeri-local-cluster)
* [`aegeri local-cluster quick-run`↴](#aegeri-local-cluster-quick-run)
* [`aegeri local-cluster quick-run where`↴](#aegeri-local-cluster-quick-run-where)
* [`aegeri local-cluster quick-run using`↴](#aegeri-local-cluster-quick-run-using)
* [`aegeri full-client`↴](#aegeri-full-client)
* [`aegeri full-client join`↴](#aegeri-full-client-join)
* [`aegeri full-client join where`↴](#aegeri-full-client-join-where)
* [`aegeri full-client join using`↴](#aegeri-full-client-join-using)
* [`aegeri full-client leave`↴](#aegeri-full-client-leave)
* [`aegeri full-client leave where`↴](#aegeri-full-client-leave-where)
* [`aegeri full-client leave using`↴](#aegeri-full-client-leave-using)
* [`aegeri full-client send-elf`↴](#aegeri-full-client-send-elf)
* [`aegeri full-client send-elf where`↴](#aegeri-full-client-send-elf-where)
* [`aegeri full-client send-elf using`↴](#aegeri-full-client-send-elf-using)

## `aegeri`

**Usage:** `aegeri <COMMAND>`

###### **Subcommands:**

* `markdown` — Generate CLI documentation
* `local-cluster` — Manage local cluster
* `full-client` — Manage full client



## `aegeri markdown`

Generate CLI documentation

**Usage:** `aegeri markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `aegeri markdown generate`

Generate and update the documentation

**Usage:** `aegeri markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `aegeri markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `aegeri markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `aegeri markdown print`

Print the documentation in the shell

**Usage:** `aegeri markdown print`



## `aegeri markdown workspace`

Generate the documentation for the workspace

**Usage:** `aegeri markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `aegeri local-cluster`

Manage local cluster

**Usage:** `aegeri local-cluster <COMMAND>`

###### **Subcommands:**

* `quick-run` — 



## `aegeri local-cluster quick-run`

**Usage:** `aegeri local-cluster quick-run <COMMAND>`

###### **Subcommands:**

* `where` — Run quickrun with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run quickrun with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `aegeri local-cluster quick-run where`

Run quickrun with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri local-cluster quick-run where [OPTIONS]`

###### **Options:**

* `--count <COUNT>` — The number of nodes to start

  Default value: `4`
* `--topic <TOPIC>` — The topic to use for the nodes

  Default value: `aegeri-local-cluster-quick-run`
* `--output-file <OUTPUT_FILE>` — The file to write the peer list to

  Default value: `aegeri.peer-list.json`



## `aegeri local-cluster quick-run using`

Run quickrun with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri local-cluster quick-run using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--count <COUNT>` — The number of nodes to start

  Default value: `4`
* `--topic <TOPIC>` — The topic to use for the nodes

  Default value: `aegeri-local-cluster-quick-run`
* `--output-file <OUTPUT_FILE>` — The file to write the peer list to

  Default value: `aegeri.peer-list.json`



## `aegeri full-client`

Manage full client

**Usage:** `aegeri full-client <COMMAND>`

###### **Subcommands:**

* `join` — 
* `leave` — 
* `send-elf` — 



## `aegeri full-client join`

**Usage:** `aegeri full-client join <COMMAND>`

###### **Subcommands:**

* `where` — Run join with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run join with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `aegeri full-client join where`

Run join with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client join where [OPTIONS]`

###### **Options:**

* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer.

   Currently interpreted as a 32-byte hex seed for ML-DSA key derivation.
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peers <PEERS>` — The list of public keys to join the cluster
* `--multiaddr <MULTIADDR>` — The multiaddress to join the cluster on
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`



## `aegeri full-client join using`

Run join with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client join using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--peer-list-path <PEER_LIST_PATH>`
* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer.

   Currently interpreted as a 32-byte hex seed for ML-DSA key derivation.
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`



## `aegeri full-client leave`

**Usage:** `aegeri full-client leave <COMMAND>`

###### **Subcommands:**

* `where` — Run leave with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run leave with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `aegeri full-client leave where`

Run leave with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client leave where [OPTIONS]`

###### **Options:**

* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peers <PEERS>` — The list of public keys to join the cluster
* `--multiaddr <MULTIADDR>` — The multiaddress to join the cluster on
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`



## `aegeri full-client leave using`

Run leave with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client leave using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--peer-list-path <PEER_LIST_PATH>`
* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`



## `aegeri full-client send-elf`

**Usage:** `aegeri full-client send-elf <COMMAND>`

###### **Subcommands:**

* `where` — Run sendelf with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run sendelf with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `aegeri full-client send-elf where`

Run sendelf with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client send-elf where [OPTIONS] --elf <ELF>`

###### **Options:**

* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peers <PEERS>` — The list of public keys to join the cluster
* `--multiaddr <MULTIADDR>` — The multiaddress to join the cluster on
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`
* `--elf <ELF>` — ELF path or workspace binary name



## `aegeri full-client send-elf using`

Run sendelf with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `aegeri full-client send-elf using [OPTIONS] --elf <ELF> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--peer-list-path <PEER_LIST_PATH>`
* `--topic <TOPIC>` — Topic to use for gossamer networking

  Default value: `aegeri-local-cluster-quick-run`
* `--private-key <PRIVATE_KEY>` — The private key hex string to use for the signer
* `--seed <SEED>` — The seed to use for the signer if no private key is provided

  Default value: `42`
* `--peer-count-required <PEER_COUNT_REQUIRED>` — The number of peers to require during bootstrap

  Default value: `3`
* `--timeout-seconds <TIMEOUT_SECONDS>` — Timeout in seconds to wait for transition confirmation

  Default value: `60`
* `--elf <ELF>` — ELF path or workspace binary name



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
