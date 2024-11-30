<script lang="ts">
    import * as THREE from 'three';
    import { onMount} from "svelte";

    let canvas: HTMLCanvasElement;

    onMount(() => {
        const scene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(
            75,
            canvas.clientWidth / canvas.clientHeight,
            0.1,
            1000,
        )
        camera.position.z = 5;
        const renderer = new THREE.WebGLRenderer({ canvas });
        renderer.setSize(canvas.clientWidth, canvas.clientHeight);

        // const material = new THREE.LineBasicMaterial({color: 0x00ff00 });
        // const points = [
        //     new THREE.Vector3(-1, 0, 0),
        //     new THREE.Vector3(0, 1, 0),
        //     new THREE.Vector3(1, 0, 0),
        // ]
        // const geometry = new THREE.BufferGeometry().setFromPoints(points);
        // const line = new THREE.Line(geometry, material);
        // scene.add(line);

        const meshMaterial = new THREE.MeshBasicMaterial({color: 0x00ff00});
        const baseGeometry = new THREE.CylinderGeometry(1, 1, 1);
        const base = new THREE.Mesh(baseGeometry, meshMaterial);
        base.position.set(0, -2, 0);
        scene.add(base);


        function animate() {
            requestAnimationFrame(animate);

            base.rotation.x += 0.01;
            base.rotation.y += 0.01;

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
        border: 1px solid blue;
    }
</style>