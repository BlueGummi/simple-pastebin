# Running in Docker

## Prerequisites

This webserver can be *run* in **Docker**, however the **Docker** package must be installed, and the **Docker daemon** must be running.

The **binary** must also be compiled, and can be compiled by **executing** `cargo run --release` in the webserver directory.

The Docker image will **copy** `config.toml`, therefore **all configuration** must be done in `config.toml`, as it is *not possible* to pass CLI arguments to the Docker container **once it is running**.

## Building and running

To build the `Dockerfile`, 
```
docker build -t NAME .
``` 
can be *run in the directory* of the **repository containing the Dockerfile**.

To **run** the Docker container, 
```
docker run --it --name CONTAINER_NAME NAME
``` 
can be run.

The Dockerfile pulls the **Debian Bookworm Slim Docker image**.

To run the Dockerfile in the **background**, the `-d` flag can be passed.

## Stopping

If the Docker container is **being run in the foreground**, *Ctrl-C* can be pressed to **gracefully shutdown**. Otherwise, `docker stop CONTAINER_NAME` can be executed to **stop the container**. 

If no name is specified, run 
```
docker ps
```
to view **all running containers**, then find the **name** of the image and run 
```
docker stop CONTAINER_NAME
```


