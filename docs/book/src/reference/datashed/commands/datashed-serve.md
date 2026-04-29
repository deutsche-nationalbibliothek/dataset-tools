# datashed-serve(1)

## NAME

*datashed-serve* --- Serves the datashed inventory via HTTP.

## SYNOPSIS

`datashed serve` [_OPTIONS_]

## DESCRIPTION

This command serves the datashed's index and documents via HTTP.
The server is configured via the `[server]` section in the `config.toml`
file:

```toml
[server]
address = "0.0.0.0"
port = 9100
workers = 4
cert = "/etc/certs/cert.pem"
key = "/etc/certs/key.pem"
```

## OPTIONS

`-h`, `--help`
: Print help

### COMMON OPTIONS

{{ #include common-opts.md }}

## EXIT STATUS

{{ #include exit-status.md }}

## EXAMPLES

```console
$ datashed serve
[2026-04-29T07:10:22Z INFO ] starting 4 workers
[2026-04-29T07:10:22Z INFO ] Actix runtime found; starting in Actix runtime
[2026-04-29T07:10:22Z INFO ] starting service: "actix-web-service-0.0.0.0:9100", workers: 4, listening on: 0.0.0.0:9100
[2026-04-29T07:11:27Z INFO ] HEAD /health-check HTTP/2.0 200 0
[2026-04-29T07:11:28Z INFO ] HEAD /health-check HTTP/2.0 200 0
[2026-04-29T07:11:29Z INFO ] HEAD /health-check HTTP/2.0 200 0
...
```
