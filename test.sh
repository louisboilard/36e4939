#!/bin/bash

    # id: String,
    # name: String,
    # year: u16,
    # was_good: bool,
curl -v -X POST localhost:3000/movie \
    -H "Content-Type: application/json" \
    --data '{"id":"1", "name":"fun movie 2nd edition", "year":1998, "was_good": true}' \
