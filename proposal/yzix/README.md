# yzix-proto

in order to make distributed workflows both simple and reasonable reliable,
use etcd as a unordered message queue (for build requests/schedule ops)
and as a state hashmap.
structure:

## `{store-path}/{hash-type}/run/{inhash}`

contains a serialization of a build request, which,
with the prev/current design corresponds to a `build_graph::Node`.
it gains an additional field `state` (equiv. to current `rest.output`),
which contains one of the following values:
- `Unclaimed{ data, pushtime }` (unclaimed work)
- `Claimed{ data, host }` (claimed work)
- `Success(outhash)` (done work)
- `Failure(errmsg)` (failed work)
Using an etcd transaction, the server updates the state from `NotScheduled`
to `Scheduled` (and nothing else, if the item is not in `NotScheduled` state,
no server can additionally claim it, as it is already claimed).
  TODO: figure out how to check if the server doing the work hasn't crashed.

The `data` parameter contains all the `WorkItem` information, which is necessary
to execute the request.
  TODO: add a `system` parameter to the `WorkItem` information.

The `pushtime` parameter contains the time at which the item was submitted,
and is used to sort requests when scheduling.
  TODO: figure out how to make scheduling preference and such dependent on the
  amount of data that the server needs to download in order to start working.
