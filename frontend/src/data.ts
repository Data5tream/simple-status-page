export interface Watchpoint {
  id: string;
  name: string;
  kind: "url" | "keyword";
  target: URL;
  keyword: string | undefined;
}

export interface Watcher {
  watchpoint: Watchpoint;
  status: number;
}

export interface CurrentStatus {
  watchers: Array<Watcher>;
}
