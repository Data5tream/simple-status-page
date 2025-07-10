import { Component, computed, input } from "@angular/core";

import { Watchpoint } from "../app/data-service";
import { StatusIcon } from "./status-icon";

@Component({
  selector: "ssp-status-entry",
  imports: [StatusIcon],
  templateUrl: "./status-entry.html",
  styleUrl: "./status-entry.scss",
})
export class StatusEntry {
  watchpoint = input.required<Watchpoint>();

  statusMsg = computed(() => {
    const data = this.watchpoint();

    if (!data) {
      return "";
    }

    if (data.status === 200) {
      return 'OK (200)';
    } else if (data.status === 404) {
      return `Not found (${data.status})`;
    } else if (data.status < 600) {
      return `${data.status}`;
    } else if (data.status === 600) {
      return 'Invalid DNS';
    } else if (data.status === 601) {
      return 'TLS validation error';
    } else if (data.status === 604) {
      return 'Keyword not found';
    } else if (data.status === 610) {
      return 'Unable to parse HTML content';
    }

    return "Unkown status";
  });
}
