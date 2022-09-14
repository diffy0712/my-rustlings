# Campaigns web api

Simple CRUD api to manage campaigns. This is just for playing around with actix-web in a dockerized environment.

## Running the web server

### localy
```
cargo run
```

This will run the web server on port 8080

### Inside docker

Build the image:
```
docker build --target webserver --tag campaigns-api:1.0.0 .
```

Run the image:
```
docker run -p 8080:8080 campaigns-api:1.0.0
```

