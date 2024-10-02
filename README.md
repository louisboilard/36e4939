NOTE: The db needs to be wrapped inside an arc mutex, cloning the app state
implies new instances of the db (hashmap) are currently created, which is obviously not
what we want (adding this note here since I had ran out of time trying to fix a
silly parenthesis issue in axum's router which took me too long, and don't want
to "cheat" by implementing it afterwards).

As a next step it would also be beneficial to modify what we currently return in the get method when the movie is not present (don't return the default impl for a movie, return an http code indicating the movie isn't in the db).

NOTE2: Took a few mins after the test to refresh my knowledge of axum via the documentation and made a few quick corrections (including what's mentioned above) that I pushed on another branch as I was quite unhappy with the current state of things after the test as I had not used axum in about 3 months. Feel free to take a look here: https://github.com/louisboilard/36e4939/tree/corrections.
