<script lang="ts">
  let toSend: string = $state("");
  let received: string | null = $state(null);
  // Todo: handle errors on websocket creation
  const ws = new WebSocket('ws://localhost:8080');
  ws.onmessage = (ev) => {received=ev.data};

  function sendMsg() {
    ws.send(toSend);
    toSend = '';
  }
</script>

<input type="text" bind:value={toSend} placeholder="type here" />

<button onclick={sendMsg}>Send message!</button>
{#if received !== null}
  <p>Answer: {received}</p>
{/if}
