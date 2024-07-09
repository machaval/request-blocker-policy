# Request Blocker Policy

This policy will accept only requests that accept the header names and values configured on the policy configuration.

For example to block a request that only accepts request contaning the header `x-forwarded-port` with value `443` use:

```yaml
        config: {
  all: [ { headerName: "x-forwarded-port", headerValue: "443" } ]
}
```
It supports multiple headers and in that case it will verify that they `all` match.

## Build the policy

* Install all [tools needed for the PDK](https://docs.mulesoft.com/pdk/latest/policies-pdk-prerequisites)

* Install [JQ](https://jqlang.github.io/jq/download)

* Clone this repository and open its directory in a terminal

* Run `setup-business_group.sh` and select the business group you want to select

```bash
./setup-business_group.sh
```

* Run `make setup` to configure all the tools

```bash
make setup
```

* Build the policy

```bash
make build
```

* Publish the policy to exchange

```bash
make release
```

> IMPORTANT: Remember that the version of the asset to be deployed needs to be changed every time you want to release a new version. This can be changed in the `Cargo.toml` file. The value for asset version is under the `[package]` header with the key name `version`.

## Use the policy

* Now the new policy will appear under the Security section in API Manager.
