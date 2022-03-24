api.senpy.club
==============

This API is on commit {}.

Routes
------

If a language requires a parameter, it will be notated like ":this".

For example; if a route is notated as "/v2/route/:parameter", you can access that route via the URL "https://api.senpy.club/v2/route/parameter-value".

- /
    - /: Index page, a mirror of /v2 (you are here)

- /v2
    - /github: GitHub API mirror for the https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books repository
    - /languages: A list of all languages that appear in the https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books repository
    - /language/:language: A list of all images that are labeled under the language, ":language", in the https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books repository
    - /random: A random image from the https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books repository
    - /me: For future implementation. At the moment; only contains the caller's IP.

- /v2/boys
    - /github: GitHub API mirror for the https://github.com/flyingcakes85/Anime-Boys-Holding-Programming-Books repository
    - /languages: A list of all languages that appear in the https://github.com/flyingcakes85/Anime-Boys-Holding-Programming-Books repository
    - /language/:language: A list of all images that are labeled under the language, ":language", in the https://github.com/flyingcakes85/Anime-Boys-Holding-Programming-Books repository
    - /random: A random image from the https://github.com/flyingcakes85/Anime-Boys-Holding-Programming-Books repository

Notes
-----

Contributing
^^^^^^^^^^^^

If you'd like to support the project in any way, check out the repository: https://github.com/senpy-club/api-worker

Supporting
^^^^^^^^^^

If you would like to support my development ventures, visit my GitHub profile: https://github.com/Fuwn

License
^^^^^^^

`GNU General Public License v3.0 <https://github.com/senpy-club/api-worker/blob/main/LICENSE>`_
