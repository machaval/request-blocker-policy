# Request Blocker Policy

This policy blocks or accept request based on header values. The user can specify the header name and value that it needs to have for the request to be accepted.


For example:

```yaml
        config: {
          all: [ { headerName: "x-forwarded-port", headerValue: "443" } ]
        }
```
This configuration will accept only the request that have the `x-forwarded-port` with the value `443`

## Build the policy

* Install all  [tools needed for the PDK](https://docs.mulesoft.com/pdk/latest/policies-pdk-prerequisites)

*  Run make setup to configure all the tools
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
> IMPORTANT: Remember that the version of the asset to be deployed needs to be changed every time you want to release a new version. It is being changed on the Cargo.toml file

* Now under the Security section on API Manager the new policy will appear