import { useMemo } from "preact/hooks";

import type { CurrentStatus } from "../../data.ts";

import "./styles.css";

type StatusStatus = "valid" | "invalid";

interface StatusEntryProps {
  status: StatusStatus;
  count: number;
  label: string;
}

interface StatusOverviewProps {
  data: CurrentStatus;
}

function StatusEntry({ status, count, label }: StatusEntryProps) {
  return (
    <li class={status}>
      <span>{count}</span>
      <span>{label}</span>
    </li>
  );
}

export function StatusOverview({ data }: StatusOverviewProps) {
  const valid = useMemo<number>(() => {
    return data.watchers.reduce(
      (p, c) => (c.status >= 200 && c.status < 400) ? ++p : p,
      0,
    );
  }, [data]);

  const invalid = useMemo<number>(() => {
    return data.watchers.reduce((p, c) => (c.status >= 400) ? ++p : p, 0);
  }, [data]);

  return (
    <ul class="counters">
      <StatusEntry status="valid" count={valid} label="OK" />
      <StatusEntry status="invalid" count={invalid} label="with issues" />
    </ul>
  );
}
