import { useEffect, useState } from "preact/hooks";
import type { Watcher } from "../../data.ts";

import "./styles.css";

interface StatusCardProps {
  data: Watcher;
}

/// Get status card text from code
const getStatusText = (status: number): string => {
  if (status === 200) {
    return "OK (200)";
  } else if (status === 404) {
    return `Not found (${status})`;
  } else if (status < 600) {
    return `${status}`;
  } else if (status === 600) {
    return "Invalid DNS";
  } else if (status === 601) {
    return "TLS validation error";
  } else if (status === 604) {
    return "Keyword not found";
  } else if (status === 610) {
    return "Unable to parse HTML content";
  }

  return `Unknown status (${status})`;
};

export function StatusCard({ data }: StatusCardProps) {
  const [statusText, setStatusText] = useState("Unknown status");
  const [statusClass, setStatusClass] = useState("");

  useEffect(() => {
    if (data.status === 200) {
      setStatusClass("green");
    } else if (data.status > 200 && data.status < 400) {
      setStatusClass("yellow");
    } else {
      setStatusClass("red");
    }

    setStatusText(getStatusText(data.status));
  }, [data]);

  return (
    <div class="watchpoint">
      <h4>{data.watchpoint.name}</h4>
      <span>
        Status: <span class={`status-icon ${statusClass}`}></span> {statusText}
      </span>
    </div>
  );
}
