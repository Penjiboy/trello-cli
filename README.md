# Trello CLI

Trello CLI is a commandline tool that allows you to interact with Trello's API from the comfort of your terminal.

One of the goals for this project was to have this tool be a potentially standalone task management tool (in case I ever lose access to Trello for whatever reason). For this reason, you can use Trello CLI completely offline through the use of a MongoDB instance which can synchronize data with Trello if desired.

## Prerequisites

Before you begin, ensure you have met the following requirements:
<!--- These are just example requirements. Add, duplicate or remove as required --->
* You have installed the latest version of `Rust` (see [rust-lang.org](https://www.rust-lang.org/tools/install))
* [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/)
* You have setup and can access the Trello API using your developer key and token (see [setting up instructions](https://developer.atlassian.com/cloud/trello/guides/rest-api/api-introduction/))

**Note:** I have only tested this on a Linux machine.

## Building Trello CLI

To build Trello CLI, follow these steps:
* Clone this repository
  * e.g. `git clone https://github.com/Penjiboy/trello-cli.git`
* Move to the project root directory and build with `cargo`
  * e.g. `cd trello-cli && cargo build --release`

## Using Trello CLI

To use Trello CLI, follow these steps:

* Create a `config.json` file with the following contents:
```
{
    "trello":
    {
        "developer_api_key": "<Your Trello API Key>",
        "developer_api_token": "<Your Trello API Token>"
    },
    "mongodb":
    {
        "username": "<MongoDB Username>",
        "password": "<MongoDB Password>",
        "host": "<MongoDB Host, e.g. localhost>",
        "port": <MongoDB Port #>
    }
}
```

* If you wish to use MongoDB instance inside a local docker container, feel free to edit the details in `docker-compose.yml` and then run
```
docker-compose up -d
```

* Run the `trello-cli` executable, e.g.
```
./target/release/trello-cli -h
```

```
./target/release/trello-cli -i -c [/path/to/config/file]
```

* Running with `-i` takes you into the interactive shell
* NOTE: The very first config file you run with will be saved to \$HOME/.config/trello-cli/config.json and this will be the default config file that is used if you do not provide the argument

## Tips for running in the interactive shell

...

## Contributing to Trello CLI
To contribute to Trello CLI, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## License
This project uses the following license: [MIT](https://choosealicense.com/licenses/mit/).
