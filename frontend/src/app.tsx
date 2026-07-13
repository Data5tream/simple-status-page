import { useEffect, useRef, useState } from "preact/hooks";

import { AppTitle } from "./components/app_title.tsx";
import { StatusCard } from "./components/status_card/index.tsx";
import { StatusOverview } from "./components/status_overview/index.tsx";
import { LoadingOverlay } from "./components/loading_overlay/index.tsx";
import { LoadingSpinner } from "./components/loading_spinner/index.tsx";

import { type CurrentStatus } from "./data.ts";

import "./app.css";

const url =
  (import.meta.env.PUBLIC_API_URL != undefined
    ? import.meta.env.PUBLIC_API_URL
    : "http://127.0.0.1:8000") + "/status";
const updateMs = 2000;

export function App() {
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<CurrentStatus | null>(null);
  const [loading, setLoading] = useState(false);

  const abortRef = useRef<AbortController | null>(null);
  const timerRef = useRef<number | null>(null);

  const fetchOnce = async () => {
    if (abortRef.current) {
      abortRef.current.abort();
    }

    const ac = new AbortController();
    abortRef.current = ac;

    setLoading(true);
    setError(null);

    try {
      const res = await fetch(url, { signal: ac.signal });
      if (!res.ok) {
        throw new Error(`${res.status} ${res.statusText}`);
      }

      const json = await res.json();
      setStatus({ watchers: json } as CurrentStatus);
    } catch (err: unknown) {
      if (err instanceof DOMException && err.name !== "AbortError") {
        setError(err.message ?? String(err));
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOnce();

    timerRef.current = setInterval(fetchOnce, updateMs);

    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
      if (abortRef.current) {
        abortRef.current.abort();
      }
    };
  }, [url, updateMs]);

  if (!status) {
    return (
      <>
        <AppTitle />
        <LoadingOverlay />
      </>
    );
  }

  const content = () => {
    if (error) {
      return <span>Error</span>;
    }

    return (
      <main>
        <div class="status-bar">
          <StatusOverview data={status} />
          {loading && <LoadingSpinner color="#333" />}
        </div>
        <div class="status-list">
          {status.watchers.map((w) => (
            <StatusCard key={w.watchpoint.id} data={w} />
          ))}
        </div>
      </main>
    );
  };

  return (
    <>
      <AppTitle />
      {content()}
    </>
  );
}
