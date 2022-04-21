import { ReadableStream as PolyfillReadableStream, TransformStream as PolyfillTransformStream } from '/web-streams.es6.mjs';
import { createReadableStreamWrapper } from '/web-streams-adapter.mjs';

// `progressiveFetch` is a wrapper over `window.fetch`. It allows you to insert middle-ware that is
// polled as the fetch completes. See bevy-website/issues/338 for details.
async function progressiveFetch(resource, callbacks={}) {
  const toPolyfillReadable = createReadableStreamWrapper(PolyfillReadableStream);
  const toNativeReadable = createReadableStreamWrapper(window.ReadableStream);

  const cb = Object.assign({
    start: (length) => {},
    update: (loaded, length) => {},
    finish: (length) => {},
  }, callbacks);
  let response = await fetch(resource);
  const lengthBytes = response.headers.get('content-length');
  let loadedBytes = 0;
  const transform = new PolyfillTransformStream({
    start() {
      cb.start(lengthBytes);
    },
    transform(chunk, controller) {
      loadedBytes += chunk.byteLength;
      cb.update(loadedBytes, lengthBytes);
      controller.enqueue(chunk);
    },
    flush() {
      cb.finish(lengthBytes);
    },
  });
  return new Response(toNativeReadable(toPolyfillReadable(response.body).pipeThrough(transform)), response);
}

export { progressiveFetch };
