import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
    prerender: {
      handleHttpError: ({ path, message }) => {
        // ignore unavailable API endpoint during prerender
        if (path === '/status') {
          return;
        }

        // otherwise fail the build
        throw new Error(message);
      },
    },
  },
};

export default config;
