import type { PageLoad } from './$types';

import { loadStatus } from '$lib/dataprovider';

export const load = (async ({ fetch, depends }) => {
	depends('app:statusList');
	return {
		status: await loadStatus(fetch)
	};
}) satisfies PageLoad;
