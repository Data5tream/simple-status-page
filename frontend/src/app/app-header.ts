import { Component, computed } from "@angular/core";

@Component({
  selector: "ssp-header",
  template: `
    <h1>{{name()}}</h1>
  `,
  styles: `
    h1 {
      display: inline-block;
      background: var(--c-background);
      color: var(--c-foreground);
      padding: 8px 16px 8px 0;
      font-family: monospace;

      &:after {
        content: '_';
        animation: blink 2s infinite;
      }
    }
  `
})
export class AppHeader {
  name = computed(() => "Simple Status Page")
}

