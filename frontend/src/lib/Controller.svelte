<script lang="ts">
  let toSend: string = $state("");
  let received: string | null = $state(null);

  let { craneState = $bindable() } = $props();
  $inspect(craneState);

  // Todo: handle errors on websocket creation
  const ws = new WebSocket('ws://localhost:8080');
  ws.onmessage = (ev) => {
    console.log(ev);
    received = ev.data;
    craneState = JSON.parse(ev.data);
  };

  function sendMsg() {
    ws.send(toSend);
    toSend = '';
  }

  function changeCrane() {
    console.log('changing crane???');
    craneState = {
      swing_rotation: 0,
      lift_elevation: 1000,
      elbow_rotation: 45,
      wrist_rotation: 0,
      gripper_open: 0,
    };
  }
</script>

<button onclick={changeCrane}>click</button>
<input type="text" bind:value={toSend} placeholder="type here" />

<button onclick={sendMsg}>Send message!</button>
{#if received !== null}
  <p>Answer: {received}</p>
{/if}
