import { PUBLIC_API_URL } from '$env/static/public';

export interface Watchpoint {
	status: number;
	watchpoint: {
		id: string;
		ip: string;
		name: string;
		url: string;
	};
}

export interface StatusData {
	is_valid: boolean;
	watchpoints?: Array<Watchpoint>;
}

export const loadStatus = async (): Promise<StatusData> => {
	try {
		const res = await fetch(`${PUBLIC_API_URL}/status`);
		if (res.status !== 200) {
			return {
				is_valid: false
			};
		}

		const watchpoints = await res.json();

		return {
			is_valid: true,
			watchpoints
		};
	} catch (_) {
		return {
			is_valid: false
		};
	}
};
