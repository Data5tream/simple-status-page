import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, switchMap, timer } from 'rxjs';

export interface Watchpoint {
  status: number;
  watchpoint: {
    id: string;
    name: string;
    kind: "url" | "keyword";
    target: string;
    keyword?: string;
  };
}

export interface StatusData {
  watchpoints?: Array<Watchpoint>;
}

export async function loadStatus(): Promise<StatusData> {
  try {
    // TODO: replace with correct URL
    const result = await fetch("http:/127.0.0.1:8000/status");
    if (result.status !== 200) {
      return Promise.reject(new Error("Fetch error"));
    }

    const watchpoints = await result.json();
    return { watchpoints };
  } catch (_) {
    return Promise.reject(new Error("Unknown error"));
  }
}
@Injectable({
  providedIn: 'root'
})
export class DataService {
  private http = inject(HttpClient);

  private url = "http://127.0.0.1:8000/status";

  fetchData(): Observable<Array<Watchpoint>> {
    return timer(0, 2000).pipe(switchMap(() => this.http.get<Array<Watchpoint>>(this.url)));
  }
}

