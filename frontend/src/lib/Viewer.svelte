<script lang="ts">
    import * as THREE from 'three';
    import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
    import {onMount} from "svelte";

    let canvas: HTMLCanvasElement;

    class CraneModel {
        group: THREE.Group
        material: THREE.MeshBasicMaterial
        // Having trouble with getting camera right so base is nice and low.
        // Instead just shifting all objects.
        Y_SHIFT: number

        constructor(bodyHeight: number) {
            this.Y_SHIFT = -bodyHeight / 2;
            this.group = new THREE.Group()
            this.material = new THREE.MeshBasicMaterial({color: 0x00ff00});
            const bodyGeometry = new THREE.BoxGeometry(1, bodyHeight, 1);
            const body = new THREE.Mesh(bodyGeometry, this.material);

            const baseRotation = THREE.MathUtils.degToRad(45);
            body.position.set(0, bodyHeight / 2 - 0.5 + this.Y_SHIFT, 0);

            const liftHeight = 10;

            const upperArmLength = 3;
            const armPartGeometry = new THREE.BoxGeometry(upperArmLength, 1, 1);
            const upperArm = new THREE.Mesh(armPartGeometry, this.material);
            // Todo: figure out some internal representation for this relative stuff
            // Rotation happens in the xz-plane, meaning y won't change.
            // At 0 degrees, this is position (1, 0), a segment of length 1.
            // As we rotate, the center goes along the unit circle, i.e.
            // [x, y, z] = [cos(alpha), y, sin(alpha)]
            // To compensate for the thing flying around
            // object.rotation.y = pi-alpha
            upperArm.position.set(Math.cos(baseRotation), liftHeight + this.Y_SHIFT, Math.sin(baseRotation));
            upperArm.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation, 0));

            // To rotate the lower arm, we have a similar setup, with the
            // difference that it rotates an offset unit circle.
            // We take the joint to be twice as far from the origin as
            // the center of the upper arm. Some careful drawing gives the
            // formulas below.
            const armRotation = THREE.MathUtils.degToRad(90);
            const lowerArm = new THREE.Mesh(armPartGeometry, this.material);
            lowerArm.position.set(
                upperArm.position.x * 2 + Math.cos(armRotation + baseRotation),
                upperArm.position.y - 1,  // Shift one unit below
                upperArm.position.z * 2 + Math.sin(armRotation + baseRotation),
            );
            lowerArm.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation, 0));

            // The wrist center inscribes a unit circle with a slightly larger
            //  radius than the lower arm. Since the wrist is centered around
            //  the wrist joint, the position doesn't change when it rotates.
            const wristRotation = THREE.MathUtils.degToRad(60);
            const wristGeometry = new THREE.BoxGeometry(1.25, 2, 1.25);
            const wrist = new THREE.Mesh(wristGeometry, this.material);
            wrist.position.set(
                lowerArm.position.x + Math.cos(armRotation + baseRotation),
                lowerArm.position.y - 1.5,  // Shift 1.5 units below
                lowerArm.position.z + Math.sin(armRotation + baseRotation),
            );
            wrist.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation - wristRotation));

            // Attachment point for the gripper is the same as wrist, just lower.
            // Again, center inscribes a unit circle on top of this.
            const gripperExtension = 2;
            const gripperGeometry = new THREE.BoxGeometry(1 + gripperExtension, 0.25, 0.25);
            const gripper = new THREE.Mesh(gripperGeometry, this.material);
            gripper.position.set(
                wrist.position.x + gripperExtension * Math.cos(wristRotation + armRotation + baseRotation),
                wrist.position.y - 1.05,
                wrist.position.z + gripperExtension * Math.sin(wristRotation + armRotation + baseRotation),
            )
            gripper.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation - wristRotation));

            this.group.add(body, upperArm, lowerArm, wrist, gripper);
        }
    }

    let crane = new CraneModel(12);

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
        width: 800px;
        height: 800px;
        border: 1px solid blue;
    }
</style>