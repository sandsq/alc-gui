import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	css: {
		preprocessorOptions: {
			scss: {
				additionalData: `
				@use '/src/styles/colors.scss' as *;
				@use '/src/styles/global.scss' as *;
				`
			}
		}
	}
});
