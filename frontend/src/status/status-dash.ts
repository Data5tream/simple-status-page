import { Component, computed, input } from "@angular/core";
import { Watchpoint } from "../app/data-service";

@Component({
  selector: "ssp-status-dash",
  styleUrl: "./status-dash.scss",
  templateUrl: "./status-dash.html",
})
export class StatusDash {
  statusData = input<Array<Watchpoint>>([]);

  okCount = computed(() => this.statusData().filter(s => s.status >= 200 && s.status < 300).length);
  errorCount = computed(() => this.statusData().filter(s => s.status >= 301).length);
}
