import { Component, input } from "@angular/core";
import { Watchpoint } from "../app/data-service";
import { StatusEntry } from "./status-entry";

@Component({
  selector: "ssp-status-list",
  imports: [StatusEntry],
  templateUrl: "./status-list.html",
  styleUrl: "./status-list.scss",
})
export class StatusList {
  statusData = input<Array<Watchpoint>>([]);
}
