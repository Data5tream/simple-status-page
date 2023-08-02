<script lang="ts">
  import type { Watchpoint } from '$lib/dataprovider';

  export let data: Array<Watchpoint>;

  let ok_count: number;
  let invalid_count: number;

  $: {
    let [ok, not_ok] = [0, 0];
    data?.forEach(wp => {
      if (wp.status === 200) {
        ok++;
      } else  {
        not_ok++;
      }
    })

    ok_count = ok;
    invalid_count = not_ok;
  }
</script>

<ul class='counters'>
  <li class='valid'><span>{ok_count}</span><span>OK</span></li>
  <li class='invalid'><span>{invalid_count}</span><span>with issues</span></li>
</ul>

<style lang='scss'>
  .counters {
    display: flex;
    list-style: none;
    padding: 0;
    margin: 0 0 16px;
    gap: 16px;

    li {
      color: var(--c-white);
      padding: 0;

      span {
        padding: 4px 8px;

        &:first-child {
          background: var(--bg-color);
          padding: 4px 12px;
        }

        &:last-child {
          background: var(--c-black);
        }
      }
    }

    .valid {
      --bg-color: var(--c-green);
    }

    .invalid span {
      --bg-color: var(--c-red);
    }
  }
</style>
