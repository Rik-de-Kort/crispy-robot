<script lang="ts">
    import * as THREE from 'three';
    import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
    import {onMount} from "svelte";

    let canvas: HTMLCanvasElement;

    interface CraneState {
        baseRotation: number, // degrees
        liftHeight: number, // mm
        armRotation: number, // degrees
        wristRotation: number, // degrees
        gripperExtension: number, // mm
    }

    const INITIAL_STATE = {
        baseRotation: 45,
        liftHeight: 500,
        armRotation: 90,
        wristRotation: 45,
        gripperExtension: 300,
    }

    class CraneModel {
        group: THREE.Group
        material: THREE.MeshBasicMaterial
        // Having trouble with getting camera right so base is nice and low.
        // Instead just shifting all objects.
        Y_SHIFT: number

        get upperArm() {
            return this.group.children[1];
        }

        get lowerArm() {
            return this.group.children[2];
        }

        get wrist() {
            return this.group.children[3];
        }

        get gripper() {
            return this.group.children[4];
        }

        constructor(bodyHeight: number) {
            this.Y_SHIFT = -bodyHeight / 2;
            this.group = new THREE.Group()
            this.material = new THREE.MeshBasicMaterial({color: 0x00ff00});
            const bodyGeometry = new THREE.BoxGeometry(1, bodyHeight, 1);
            const body = new THREE.Mesh(bodyGeometry, this.material);

            body.position.set(0, bodyHeight / 2 - 0.5 + this.Y_SHIFT, 0);

            const armPartGeometry = new THREE.BoxGeometry(3, 1, 1);
            const upperArm = new THREE.Mesh(armPartGeometry, this.material);
            const lowerArm = new THREE.Mesh(armPartGeometry, this.material);
            const wristGeometry = new THREE.BoxGeometry(1.25, 2, 1.25);
            const wrist = new THREE.Mesh(wristGeometry, this.material);
            const gripperGeometry = new THREE.BoxGeometry(3, 0.25, 0.25);
            const gripper = new THREE.Mesh(gripperGeometry, this.material);
            this.group.add(body, upperArm, lowerArm, wrist, gripper);
            this.update(INITIAL_STATE);
        }

        converter(minReal: number, maxReal: number, minViz: number, maxViz: number) {
            return (x: number) => {
                if (x < minReal || maxReal < x) throw new Error(`${x} out of range for conversion!`);
                const scale = (maxViz - minViz) / (maxReal - minReal);
                return minViz + scale * x;
            }
        }

        convertLiftUnit = this.converter(0, 2000, 4, 11);
        convertGripperUnit = this.converter(0, 500, 0, 2);
        convertDegrees = this.converter(0, 360, 0, 2*Math.PI);

        update(state: CraneState) {
            const baseRotation = this.convertDegrees(state.baseRotation);
            const liftHeight = this.convertLiftUnit(state.liftHeight);
            const armRotation = this.convertDegrees(state.armRotation);
            const wristRotation = this.convertDegrees(state.wristRotation);
            const gripperExtension = this.convertGripperUnit(state.gripperExtension);

            // Rotation happens in the xz-plane, meaning y won't change.
            // At 0 degrees, this is position (1, 0), a segment of length 1.
            // As we rotate, the center goes along the unit circle, i.e.
            // [x, y, z] = [cos(alpha), y, sin(alpha)]
            // To compensate for the thing flying around
            // object.rotation.y = pi-alpha
            this.upperArm.position.set(Math.cos(baseRotation), liftHeight + this.Y_SHIFT, Math.sin(baseRotation));
            this.upperArm.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation, 0));

            // To rotate the lower arm, we have a similar setup, with the
            // difference that it rotates an offset unit circle.
            // We take the joint to be twice as far from the origin as
            // the center of the upper arm. Some careful drawing gives the
            // formulas below.
            this.lowerArm.position.set(
                this.upperArm.position.x * 2 + Math.cos(armRotation + baseRotation),
                this.upperArm.position.y - 1,  // Shift one unit below
                this.upperArm.position.z * 2 + Math.sin(armRotation + baseRotation),
            );
            this.lowerArm.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation, 0));

            // The wrist center inscribes a unit circle with a slightly larger
            //  radius than the lower arm. Since the wrist is centered around
            //  the wrist joint, the position doesn't change when it rotates.
            this.wrist.position.set(
                this.lowerArm.position.x + Math.cos(armRotation + baseRotation),
                this.lowerArm.position.y - 1.5,  // Shift 1.5 units below
                this.lowerArm.position.z + Math.sin(armRotation + baseRotation),
            );
            this.wrist.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation - wristRotation));

            // Attachment point for the gripper is the same as wrist, just lower.
            // Again, center inscribes a unit circle on top of this.
            // Todo: make the gripper extension variable work nicely.
            this.gripper.position.set(
                this.wrist.position.x + gripperExtension * Math.cos(wristRotation + armRotation + baseRotation),
                this.wrist.position.y - 1.05,
                this.wrist.position.z + gripperExtension * Math.sin(wristRotation + armRotation + baseRotation),
            )
            this.gripper.setRotationFromEuler(new THREE.Euler(0, Math.PI - baseRotation - armRotation - wristRotation));
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