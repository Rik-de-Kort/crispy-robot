<script lang="ts">
    import * as THREE from 'three';
    import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
    import {onMount} from "svelte";
    import {CraneModel} from "./CraneModel";

    let canvas: HTMLCanvasElement;
    let crane = new CraneModel();

    let {craneState} = $props();
    $effect(() => {
        crane.update(craneState);
    })

    onMount(() => {
        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(
            75,
            canvas.clientWidth / canvas.clientHeight,
            0.1,
            1000,
        )
        camera.position.z = 10;
        const renderer = new THREE.WebGLRenderer({canvas});
        renderer.setSize(canvas.clientWidth, canvas.clientHeight);

        let controls = new OrbitControls(camera, renderer.domElement);
        controls.addEventListener('change', function () {
            renderer.render(scene, camera);
        });

        scene.add(crane.group);

        function animate() {
            requestAnimationFrame(animate);
            controls.update();
            renderer.render(scene, camera);
        }
        animate();

        return () => {
            renderer.dispose();
        }
    });

</script>

<canvas bind:this={canvas}></canvas>

<style>
    canvas {
        display: block;
        width: 800px;
        height: 800px;
        border: 1px solid blue;
    }
</style>