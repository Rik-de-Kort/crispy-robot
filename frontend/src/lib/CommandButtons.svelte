<script lang="ts">
    import {Direction} from "./CraneCommand";

    // Todo: would like to dispatch on type exclusively here.
    //   Is that even possible?
    let {name, step, sendCommand} = $props();

</script>
<div>
    {name}
    <button onclick={() => sendCommand(name, Direction.Positive, step)}>+</button>
    <button onclick={() => sendCommand(name, Direction.Negative, step)}>-</button>
</div>

