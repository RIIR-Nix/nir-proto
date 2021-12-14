# yzix-proto

in order to make distributed workflows both simple and reasonable reliable,
use etcd as a unordered message queue (for build requests/schedule ops)
and as a state hashmap.
structure:

## `{store-path}/{hash-type}/run/{inhash}`

contains a serialization of a build request, which,
with the prev/current design roughly corresponds to a [`build_graph::NodeKind::Run`](https://git.ytrizja.de/zseri/yzix/src/commit/c1ff0d05ca8dd3a27998ddcbe79bbf60b55d3825/crates/yzix-core/src/build_graph/mod.rs#L53-L66).
it is organised as an enumn/oneof (equiv. to current `Node::rest.output`),
which contains one of the following values:

- `RunWorkReq+Meta { rwr, pushtime, claimed }`

  The `rwr` parameter contains all the `WorkItemRun` information, which is
  necessary to execute the request. It has now an additional `system` field.

  The `pushtime` parameter contains the time at which the item was submitted,
  and is used to sort requests when scheduling.

  TODO: figure out how to make scheduling preference and such dependent on the
  amount of data that the server needs to download in order to start working.

  Using an etcd transaction, the server updates `.claimed` from `""` to it's
  hostname (and nothing else, if the item is not already claimed or in an
  entirely different state, no server can additionally claim it,
  as it is already claimed, or already completed).

  TODO: figure out how to check if the server doing the work hasn't crashed.
  Maybe use leases for this.

  The worker server automatically downloads necessary inputs from it's known
  stores (which thus also act as substitutes), and uploads build results+logs
  to stores on which it has upload access. The worker decides which
  output hash to use and the stores verify the hash. When the upload is
  complete, the worker server updates the build request item to `Success(outhash)`.

  TODO: figure out how to give the clients and workers the information which
  stores are available and how to authenticate to them.

- `Success(outhash)` (done work)
- `Failure(errmsg)` (failed work)
