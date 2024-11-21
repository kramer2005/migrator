# Migrator

A simple db migrator that will be used in a personal project.

## Roadmap

1. <del>Publish docker image on dockerhub</del> (Released on v0.1.0 - 2024/11/20)
2. Add support for revert and deploy until checkpoint.
3. Handle 100% of the errors.
4. Code refactoring.
5. Create action.

## Usage

This project is intended to run on linux, there is two ways of running it:

### Binary:

The first one is downloading the binary:

```bash
wget https://github.com/kramer2005/migrator/releases/download/v0.1.0/migrator

chmod +x migrator

./migrator --help
```

### Docker:

The second one is running the docker image:

```bash
curl -sL https://github.com/kramer2005/migrator/releases/download/v0.1.0/migrator.sh | bash -s -- --help
```

You can replace `--help` with any other command.

## Contributing

Feel free to contribute to this project, just open a PR and I will review it as soon as possible.