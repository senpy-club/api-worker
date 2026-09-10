:code:`api-worker`
==================

Need an API wrapper for Rust? Check out `senpy-rs <https://github.com/senpy-club/senpy-rs>`_!

Need an API wrapper for another language? Check out `senpy-ffi <https://github.com/senpy-club/senpy-ffi>`_!

Quick Links
^^^^^^^^^^^

.. raw:: html

  <p>
    <a href="https://discord.com/invite/yWKgRT6">
      <img src="https://img.shields.io/discord/246524734718738442"
           alt="Discord" />
    </a>
    <a href="https://www.codefactor.io/repository/github/senpy-club/api-worker">
      <img src="https://www.codefactor.io/repository/github/senpy-club/api-worker/badge"
           alt="CodeFactor" />
    </a>
    <a href="https://saythanks.io/to/contact@fuwn.me">
      <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg"
           alt="Say Thanks" />
    </a>
    <a href="LICENSE">
      <img src="https://img.shields.io/github/license/senpy-club/api-worker"
           alt="License" />
    </a>
  </p>

Local Development and Verification
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Use stable Rust with the ``wasm32-unknown-unknown`` target and Node.js 24.
``Cargo.lock`` records the dependency versions used for testing. Build tools and
generated artefacts are stored in ignored directories. Installing the tools does
not deploy the worker.

.. code-block:: sh

  cargo test --locked --lib
  cargo clippy --locked --lib --tests
  cargo install worker-build --version 0.8.5 --locked --root .tools
  cargo install wasm-bindgen-cli --version 0.2.128 --locked --root .tools
  export PATH="$PWD/.tools/bin:$PATH"
  export WASM_BINDGEN_BIN="$PWD/.tools/bin/wasm-bindgen"
  worker-build --release
  npm install --prefix .tools --no-save --package-lock=false miniflare@5.20260825.0-alpha
  node scripts/test-worker.mjs
  node scripts/test-worker.mjs --live

The standard Wrangler build uses ``worker-build --release``. ``--no-opt`` skips
only the additional Binaryen optimisation pass and is useful when the optimiser
binary download is unavailable. The Rust release profile still applies.
The existing formatting configuration uses nightly rustfmt options.

The runtime tests run the compiled Wasm in Miniflare. They simulate GitHub rate
limits, malformed JSON, empty and truncated catalogues, recovery after a cold
failure, and a failed refresh after a successful lookup. ``--live`` additionally
reads the public GitHub catalogue; it does not contact or deploy the Senpy API.

Catalogue Behaviour
^^^^^^^^^^^^^^^^^^^

Only successful, complete catalogue responses replace cached data. A failed
refresh retains the previous good catalogue, with another refresh attempt on
the existing 50-access schedule. A cold failure is retried on the next request.
Without cached data, catalogue endpoints return a JSON error with HTTP status
503 and CORS headers instead of panicking or presenting an empty success
response.
Cached data can remain stale for the duration of an upstream outage.

Language matching is case-insensitive; returned language names and image paths
retain their original case. Unknown languages still return an empty array.
Random selection excludes empty language directories and can select every image,
including the final item. A catalogue with no selectable image returns HTTP
status 503.

On 9 September 2026, all five Rust tests and the strict Clippy checks passed.
The release Wasm build, including the Binaryen optimisation pass, succeeded.
The runtime tests also passed against the optimised build. Verification against
the live catalogue found 100 languages and 57 Python images. Nothing was deployed
to production.

Licence
^^^^^^^

`GNU General Public License v3.0 <https://github.com/senpy-club/api-worker/blob/main/LICENSE>`_
