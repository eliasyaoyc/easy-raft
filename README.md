# Rust Raft KV

[comment]: <> (A very simple example to use [Raft]&#40;https://github.com/pingcap/raft-rs&#41; in Rust.)

[comment]: <> (## Build and Start)

[comment]: <> (```bash)

[comment]: <> (make)

[comment]: <> (# You can use goreman or other similar tools like foreman to manage the cluster)

[comment]: <> (# go get github.com/mattn/goreman)

[comment]: <> (goreman start )

[comment]: <> (```)

[comment]: <> (## Usage)

[comment]: <> (```bash)

[comment]: <> (# Get status of a server, we can know the leader from status)

[comment]: <> (curl http://127.0.0.1:20171/status)

[comment]: <> (# Send the request to leader)

[comment]: <> (# Put abc = 124)

[comment]: <> (curl http://127.0.0.1:20173/kv/abc -d 123)

[comment]: <> (# Get abc )

[comment]: <> (curl http://127.0.0.1:20173/kv/abc)

[comment]: <> (# Delete abc)

[comment]: <> (curl http://127.0.0.1:20173/kv/abc -x DELETE)

[comment]: <> (# Get abc locally, not through Raft )

[comment]: <> (curl http://127.0.0.1:20173/local_kv/abc)

[comment]: <> (```)