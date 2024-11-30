<script lang="ts">
    import * as THREE from 'three';
    import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
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
        camera.position.z = 10;
        // Having trouble with getting camera right so base is nice and low. Brute-forcing with YSHIFT.
        const YSHIFT = -6;
        const renderer = new THREE.WebGLRenderer({ canvas });
        renderer.setSize(canvas.clientWidth, canvas.clientHeight);

        let controls = new OrbitControls( camera, renderer.domElement );
        controls.addEventListener( 'change', function() { renderer.render(scene, camera); } );

        const bodyHeight = 12;
        const meshMaterial = new THREE.MeshBasicMaterial({color: 0x00ff00});
        const bodyGeometry = new THREE.BoxGeometry(1, bodyHeight, 1);
        const body = new THREE.Mesh(bodyGeometry, meshMaterial);
        body.position.set(0, bodyHeight/2-0.5 + YSHIFT, 0);
        scene.add(body);

        const armPartGeometry = new THREE.BoxGeometry(3, 1, 1);
        const upperArm = new THREE.Mesh(armPartGeometry, meshMaterial);
        // Todo: figure out some internal representation for this relative stuff
        upperArm.position.set(1, 4 + YSHIFT, 0);
        scene.add(upperArm);

        const lowerArm = new THREE.Mesh(armPartGeometry, meshMaterial);
        lowerArm.position.set(3, 3 + YSHIFT, 0);
        scene.add(lowerArm);

        const wristGeometry  = new THREE.BoxGeometry(1.25, 2, 1.25);
        const wrist = new THREE.Mesh(wristGeometry, meshMaterial);
        wrist.position.set(4, 1.5 + YSHIFT, 0);
        scene.add(wrist);

        const jawGeometry = new THREE.BoxGeometry(3, 0.25, 0.25);
        const jaw = new THREE.Mesh(jawGeometry, meshMaterial);
        jaw.position.set(5, 0.45 + YSHIFT, 0);
        scene.add(jaw);

        function animate() {
            requestAnimationFrame(animate);

            // column.rotation.x += 0.01;
            // column.rotation.y += 0.01;
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
        width: 600px;
        height: 800px;
        border: 1px solid blue;
    }
</style>