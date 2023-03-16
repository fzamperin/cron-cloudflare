# Cron Cloudflare

Docker image that allows you to update your own domain entry at cloudflare with you remote ip, like a ddns but using cloudflare-api. Each time the container runs it verifies if the domain has changed, and only then updates with the remote ip available.

NOTE: This container only works with ipv4, ipv6 and domains that are hosted at cloudflare

## Getting Started

Config file example:
```
auth: "auth_token"
domains:
  - name: "foo.com"
    type_ip: A
    zone_id: zone_id
  - name: "foo.com"
    type_ip: AAAA
    zone_id: zone_id
```
Running container with docker-compose:

```
version: '3'

services:
  cloudflareddns:
    image: fzamperin/cron-cloudflare:latest
    network_mode: bridge # For the use of ipv6 it's important to create an ipv6 enabled network on docker, otherwise will not work
    environment:
      - CRON_PATTERN=0 */5 * ? * * # Cron pattern to run the job
    volumes:
      - /home/user/config/config.yaml:/config/config.yaml # Config file with the token and domains to update
    restart: unless-stopped
```

### Prerequisities


In order to run this container you'll need docker installed.

* [Windows](https://docs.docker.com/windows/started)
* [OS X](https://docs.docker.com/mac/started/)
* [Linux](https://docs.docker.com/linux/started/)

#### Environment Variables

* `CRON_PATTERN` - The cron pattern to run the job, scheduling the process

#### Volumes

* `/home/user/config.yaml` - File location for the configuration file

## Built With

* Rust 1.66

## Authors

* **Fernando Penna** - *Initial work*