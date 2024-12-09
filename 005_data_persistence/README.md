# How to run this app

- Run `docker container ls -a` to know if you have any existing containers running.
- Remove any of the containers you don't want with `docker container rm <CONTAINER_ID>`.
- Alternatively, nuke all stopped containers with `docker container prune`.
- `docker-compose up` - Then exit with *Ctrl+C*.
- `docker container ls -a` - Copy the CONTAINER_ID
![alt text](image.png)
- `diesel migration generate create_to_do_items`
- `docker compose up`
- `docker exec -it <CONTAINER_ID> psql -U username to_do` -> this get's a shell interface `to_do=#`.
- Inside the `to_do=#` shell interface, run `\c` to connect to the db.
- 