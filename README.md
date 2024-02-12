# jonas_jones-api

API supporting the Jonas_Jones project infrastructure.

## Installation

As of now, the project has no proper production build and unless the proper environment variables are set, the API will not work.


Clone the repository and install the dependencies.
```bash
git clone git@github.com:J-onasJones/jonas_jones-api.git
cd jonas_jones-api
cargo build
```

## Usage

To run the API, simply run the following command.
```bash
cargo run
```

If you want to run the API in a production environment, you will need to set the following environment variables.

- API_PORT
- API_IP
- LASTFM_API_KEY
- LASTFM_API_SECRET

```bash
export API_PORT={port}
export API_IP={ip_address}
export LASTFM_API_KEY={lastfm_api_key}
export LASTFM_API_SECRET={lastfm_api_secret}
```

## Docker Compose

`docker-compose.yaml` (folder paths need adjusting):
```yaml
version: '3.8'
services:
  arch-linux:
    image: archlinux:latest
    container_name: jonas_jones-api
    ports:
      - "3030:3030"
    volumes:
      - /home/jonas_jones/jonas_jones-api:/home/jonas_jones/jonas_jones-api
      - /home/jonas_jones/.config/rclone/:/root/.config/rclone/
    command: ["sh", "-c", "pacman -Syu --noconfirm --needed pkg-config openssl python3 python-pip rclone cargo && python3 -m venv api-venv && source api-venv/bin/activate && cd /home/jonas_jones/jonas_jones-api && pip install -r requirements.txt && /usr/bin/cargo run"]
```

run container:
```sh
docker-compose up -d
```
