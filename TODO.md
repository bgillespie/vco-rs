# TODO

## Operations to implement

* Operator: to allow renewing passwords and API tokens...
  * List
  * Get
  * Modify
  * Create
  * Delete
* Properties:
  * Modify
  * Create
  * Delete
* Edges
  * List
* Gateway Metrics
  * Show

## Internals

* Docs.
* More tests besides running it against real test data held externally.
* Do we need to do anything special with `logicalId`s? Do we _need_ to parse them as `Uuid`?
* If a field in the API changes from a `tinyint` to a `boolean` between versions, it'll break this implementation as it stands. Even in the API docs for 4.5.0, some fields marked `boolean` are actually `tinyint`s. The current `TinyInt` is a `bool` internally but serializes as a `0` or `1`.

## Architectural questions

* `client` is meant to hide the internals of API calls.
  * Maybe don't directly expose the items from `api_v1` to users? Would this cause too much repetition?
  * Anticipating `api_v2` and beyond, should the interaction `reqwest` logic go into those lower-level API modules?
  * Considering REST and JSONRPC calls use the same objects but a different "wrapper", should there be separate e.g. `object_v1` and `rest_v1`/`jsonrpc_v1` modules?
