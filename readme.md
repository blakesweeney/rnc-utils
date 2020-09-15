# Pipeline utilities

This is a set of experimental set of tools mean to help with processing steps
in the pipeline. This isn't meant to replace any of the core functionality,
just some utilities that are too slow in python.

## Json Store

This is meant to help with the precompute and search export steps. These parts
involve processing data from a large join and processing the results. It is
much faster to split the query apart as much as possible and then run index the
results of each part. This set of commands is meant to deal with indexing and
accessing the data.