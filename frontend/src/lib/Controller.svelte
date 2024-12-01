<script lang="ts">
    import {Direction} from "./CraneCommand";
    import CommandButtons from "./CommandButtons.svelte";

    let {craneState = $bindable()} = $props();

    // Todo: handle errors on websocket creation
    const ws = new WebSocket('ws://localhost:8080');
    ws.onmessage = (ev) => {
        craneState = JSON.parse(ev.data);
    };

    function sendCommand(type: string, direction: Direction, amount: number) {
        // Todo: would like to move this into the command buttons maybe
        const cmd = /Rotate.*/.test(type)
            ? {type: type, direction: direction, degrees: amount}
            : {type: type, direction: direction, millimeters: amount};
        console.log({cmd});
        console.log(JSON.stringify(cmd));
        ws.send(JSON.stringify(cmd));
    }
</script>

<CommandButtons name="RotateSwing" step={3} {sendCommand}/>
<CommandButtons name="MoveLift" step={10} {sendCommand}/>
<CommandButtons name="RotateElbow" step={3} {sendCommand}/>
<CommandButtons name="RotateWrist" step={3} {sendCommand}/>
<CommandButtons name="MoveGripper" step={10} {sendCommand}/>
