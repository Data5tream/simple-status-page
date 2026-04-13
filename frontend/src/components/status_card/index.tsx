import type { Watcher } from "../../data.ts";

import "./styles.css";

interface StatusCardProps {
  data: Watcher;
}

export function StatusCard({ data }: StatusCardProps) {
  return (
    <div class="watchpoint">
      <h4>{data.watchpoint.name}</h4>
      <span>Status: {data.status}</span>
    </div>
  );
}
