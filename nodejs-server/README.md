Laravel vite-plugin is being used to configure the bundle steps for both client and server code.

I don't really have the vite knowledge required to build my own plugin, so made some workarounds over
laravel-vite-plugin to bundle, and some scripts to make everything refresh, thus
making development supportable?..

### Scripts
**(npm run dev)** `vite build --watch`
Generate new bundle files everytime a file is changed.
Needs to be run in a separate process (terminal) during development.

**(npm run build)** `vite build && vite build --ssr`
Generates the client and server-side bundle for production.

**(npm run serve)** `node --watch dist/ssr/ssr`
Starts the inertia ssr server (see www/ssr.tsx) compiled file. Restarts the file everytime something change,
because vite will inject all the pages statically during build (no idea why),
so it needs to restart the server in order to serve the latest functions with the server function.

The ssr file will be rebundled also, on any file inside `/www` change.

## Running

Summarily, we need to run `npm run dev` in one process to keep exaustingly bundling
all www/* files on any change, and `npm run serve` in another one to raise
the ssr server.

On production, we need to run `npm run build` to generate the bundle. The rust adapter
should be responsible for starting a node process that will run **not the serve script**
(because it would be uselessly watching for any file change), but `node path/to/bundled/ssr.js`
to run the actual compiled server. As it is in production, files won't change at any time,
but the deploy.