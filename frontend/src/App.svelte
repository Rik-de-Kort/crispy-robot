<script lang="ts">
  import Controller from './lib/Controller.svelte'
  import Viewer from "./lib/Viewer.svelte";
  import { type CraneState} from "./lib/crane";
  import { INITIAL_STATE } from "./lib/crane";

  let craneState: CraneState = $state(INITIAL_STATE);
  $inspect(craneState);
</script>

<main>
  <h1>Websockets!</h1>
  <Controller bind:craneState />
  <Viewer {craneState} />
</main>
