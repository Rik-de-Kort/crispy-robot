// Todo: this has to be autogeneratable somehow.
export enum Direction {
    Positive="Positive",
    Negative="Negative",
}

export type RotateSwing = {
    type: "RotateSwing";
    direction: Direction,
    degrees: number,
};

export type MoveLift = {
    type: "MoveLift";
    direction: Direction,
    millimeters: number,
}

export type RotateElbow = {
    type: "RotateElbow";
    direction: Direction,
    degrees: number,
};

export type RotateWrist = {
    type: "RotateWrist";
    direction: Direction,
    degrees: number,
};

export type MoveGripper = {
    type: "MoveGripper";
    direction: Direction,
    millimeters: number,
}

export type CraneCommand = RotateSwing | MoveLift | RotateElbow | RotateWrist | MoveGripper;