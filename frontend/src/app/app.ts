import { Component, inject } from '@angular/core';

import { AppHeader } from './app-header';
import { DataService, Watchpoint } from './data-service';
import { StatusDash } from '../status/status-dash';
import { StatusList } from '../status/status-list';

@Component({
  selector: 'app-root',
  imports: [AppHeader, StatusDash, StatusList],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  private dataService = inject(DataService);

  protected title = 'frontend';
  watchpoints: Array<Watchpoint> = [];

  ngOnInit(): void {
    this.dataService.fetchData().subscribe(data => {
      this.watchpoints = data;
    });
  }
}
