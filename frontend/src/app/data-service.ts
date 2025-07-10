import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, switchMap, timer } from 'rxjs';

import { environment } from '../environments/environment';

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

@Injectable({
  providedIn: 'root'
})
export class DataService {
  private http = inject(HttpClient);

  private url = environment.apiUrl;

  fetchData(): Observable<Array<Watchpoint>> {
    return timer(0, 2000).pipe(switchMap(() => this.http.get<Array<Watchpoint>>(this.url)));
  }
}

