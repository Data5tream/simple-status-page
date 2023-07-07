<script lang="ts">
  let iconClass = '';

  export let status;

  const getStatusClass = (status: number): string => {
    let classString = '';

    if (status === 200) {
      classString += ' green';
    } else if (status >= 201 && status < 400) {
      classString += ' yellow';
    } else if (status >= 400) {
      classString += ' red';
    }

    return classString;
  };

  $: if (status) iconClass = getStatusClass(status);
</script>

<span class={iconClass} />

<style>
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
</style>
