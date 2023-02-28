function getFilename(resource) {
    const pathname = (typeof resource === 'string')
        ? resource
        : (resource instanceof URL)
            ? resource.pathname
            : '';

    const parts = pathname.split('/');

    return parts[parts.length - 1];
}

// `progressiveFetch` is a wrapper over `window.fetch`. It allows you to insert middle-ware that is
// polled as the fetch completes. See bevy-website/issues/338 for details.
async function progressiveFetch(resource, callbacks={}) {
    const filename = getFilename(resource);
    const cb = Object.assign({
        start: (params) => {},
        update: (params) => {},
        finish: (params) => {},
    }, callbacks);

    let response = await fetch(resource);
    const lengthBytes = response.headers.get('content-length');
    let loadedBytes = 0;

    function update() {
        const loaded = Math.min(1.0, loadedBytes / lengthBytes);
        const loadedPercent = loaded * 100.0;
        const isIndeterminate = loadedBytes > lengthBytes; // Some compression is going on, so we can't know the real progress

        cb.update({ filename, isIndeterminate, loaded, loadedPercent, loadedBytes, lengthBytes });
    }

    const transform = new TransformStream({
        start() {
            cb.start({ filename, lengthBytes });
        },
        transform(chunk, controller) {
            loadedBytes += chunk.byteLength;
            update();
            controller.enqueue(chunk);
        },
        flush() {
            update();
            cb.finish({ filename, lengthBytes });
        },
    });

    return new Response(response.body.pipeThrough(transform), response);
}

export { progressiveFetch };
