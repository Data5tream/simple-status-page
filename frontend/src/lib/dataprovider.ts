import { env } from '$env/dynamic/public';


export interface Watchpoint {
  status: number;
  watchpoint: {
    id: string;
    name: string;
    kind: string;
    target: string;
    keyword?: string;
  };
}

export interface StatusData {
  is_valid: boolean;
  watchpoints?: Array<Watchpoint>;
}

export const loadStatus = async (fetch: {
  (input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
  (input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
}): Promise<StatusData> => {
  try {
    const res = await fetch(`${env.PUBLIC_API_URL}/status`);
    if (res.status !== 200) {
      return {
        is_valid: false,
      };
    }

    const watchpoints = await res.json();

    return {
      is_valid: true,
      watchpoints,
    };
  } catch (_) {
    return {
      is_valid: false,
    };
  }
};
