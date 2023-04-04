import { loadStatus } from '$lib/dataprovider';

/** @type {import('./$types').PageLoad} */
export const load = async () => {
	return {
		status: await loadStatus()
	};
};
