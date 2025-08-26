import { Component, inject, OnInit } from '@angular/core';

import { AppHeader } from './app-header';
import { DataService, Watchpoint } from './data-service';
import { StatusDash } from '../status/status-dash';
import { StatusList } from '../status/status-list';

@Component({
  selector: 'ssp-root',
  imports: [AppHeader, StatusDash, StatusList],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App implements OnInit {
  private dataService = inject(DataService);

  protected title = 'frontend';
  watchpoints: Array<Watchpoint> = [];

  ngOnInit(): void {
    this.dataService.fetchData().subscribe(data => {
      this.watchpoints = data.sort(this.sortWatchpoints);
    });
  }

  /**
   * Sort watchpoints by name, fallback to ID
   *
   * @param entryA - Entry A
   * @param entryB - Entry B
   **/
  sortWatchpoints(entryA: Watchpoint, entryB: Watchpoint): number {
    const a = entryA.watchpoint;
    const b = entryB.watchpoint;

    if (a.name > b.name) {
      return 1;
    }

    if (a.name < b.name) {
      return -1;
    }

    if (a.id > b.id) {
      return 1;
    }

    if (a.id < b.id) {
      return -1;
    }

    return 0;
  }
}
