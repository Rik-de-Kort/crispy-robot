<script lang="ts">
  import Controller from './lib/Controller.svelte'
  import Viewer from "./lib/Viewer.svelte";
  import { type CraneState} from "./lib/CraneModel";
  import { INITIAL_STATE } from "./lib/CraneModel";

  let craneState: CraneState = $state(INITIAL_STATE);
</script>

<main>
  <h1>Websockets!</h1>
  <Controller bind:craneState />
  <Viewer {craneState} />
</main>
