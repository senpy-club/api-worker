import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import {
  Miniflare,
  Response,
  convertV4MiniflareOptions,
} from "../.tools/node_modules/miniflare/dist/src/index.js";

const tree = (entries) => ({
  sha: "fixture",
  url: "https://example.test/tree",
  truncated: false,
  tree: entries.map(([path, type]) => ({
    path,
    type,
    mode: "100644",
    sha: "fixture",
    url: "https://example.test/blob",
  })),
});

const catalog = tree([
  ["Python", "tree"],
  ["Python/one.png", "blob"],
  ["Python/nested", "tree"],
  ["Python/two.png", "blob"],
]);

const createWorker = (upstream) =>
  new Miniflare(
    convertV4MiniflareOptions({
      modules: [
        {
          type: "ESModule",
          path: fileURLToPath(new URL("../build/index.js", import.meta.url)),
        },
        {
          type: "CompiledWasm",
          path: fileURLToPath(
            new URL("../build/index_bg.wasm", import.meta.url),
          ),
        },
      ],
      compatibilityDate: "2022-01-20",
      outboundService: upstream,
    }),
  );

const read = async (worker, path, status = 200) => {
  const response = await worker.dispatchFetch(`https://senpy.test${path}`);

  assert.equal(response.status, status, path);
  assert.equal(response.headers.get("Access-Control-Allow-Origin"), "*", path);

  return response.json();
};

for (const [label, payload, status] of [
  ["rate limit", { message: "Rate limited" }, 403],
  ["invalid JSON", "invalid", 200],
  ["empty tree", tree([]), 200],
  ["truncated tree", { ...catalog, truncated: true }, 200],
]) {
  const worker = createWorker(
    () =>
      new Response(
        typeof payload === "string" ? payload : JSON.stringify(payload),
        { status },
      ),
  );

  try {
    for (const path of [
      "/v2/random",
      "/v2/languages",
      "/v2/language/python",
      "/v2/github",
      "/v2/boys/random",
      "/v2/boys/languages",
    ])
      assert.ok((await read(worker, path, 503)).error);

    console.log(`Passed: ${label} produces a JSON error with CORS.`);
  } finally {
    await worker.dispose();
  }
}

let failing = true;
let requests = 0;

const worker = createWorker(() => {
  requests += 1;

  return new Response(
    JSON.stringify(failing ? { message: "Rate limited" } : catalog),
    { status: failing ? 403 : 200 },
  );
});

try {
  await read(worker, "/v2/random", 503);

  failing = false;

  assert.deepEqual(await read(worker, "/v2/languages"), ["Python"]);
  assert.equal((await read(worker, "/v2/language/python")).length, 2);
  assert.equal((await read(worker, "/v2/language/Python")).length, 2);
  assert.deepEqual(await read(worker, "/v2/language/unknown"), []);
  assert.equal((await read(worker, "/v2/random")).language, "Python");

  failing = true;

  for (let index = 0; index < 55; index++) {
    assert.equal((await read(worker, "/v2/random")).language, "Python");
  }

  assert.ok(requests >= 3);
  console.log(
    "The recovery, language matching, and cache refresh tests passed.",
  );
} finally {
  await worker.dispose();
}

if (process.argv.includes("--live")) {
  const live = createWorker(async (request) => {
    assert.equal(new URL(request.url).hostname, "api.github.com");

    const response = await fetch(request.url, {
      headers: { "User-Agent": "senpy-local-verification" },
    });

    return new Response(await response.text(), {
      status: response.status,
      headers: { "Content-Type": "application/json" },
    });
  });

  try {
    const languages = await read(live, "/v2/languages");
    const images = await read(live, "/v2/language/python");
    const random = await read(live, "/v2/random");

    assert.ok(languages.includes("Python"));
    assert.ok(images.length > 0);
    assert.ok(random.language && random.image);
    console.log(
      `The live catalogue contains ${languages.length} languages and ${images.length} Python images.`,
    );
  } finally {
    await live.dispose();
  }
}
