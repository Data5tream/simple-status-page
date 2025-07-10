import { Component, computed, input } from "@angular/core";

@Component({
  selector: "ssp-status-icon",
  template: `<span [className]="statusClass()"></span>`,
  styles: `
    span {
      display: inline-block;
      height: 0.8em;
      width: 0.8em;
      border-radius: 50%;
      background: var(--c-gray);
    }

    .green {
      background: var(--c-green);
    }
    .yellow {
      background: var(--c-yellow);
    }
    .red {
      background: var(--c-red);
    }
  `,
})
export class StatusIcon {
  status = input.required<number>();

  statusClass = computed(() => {
    if (this.status() == 200) {
      return "green";
    }
    if (this.status() >= 201 && this.status() < 400) {
      return "yellow";
    }

    return "red";
  })
}
