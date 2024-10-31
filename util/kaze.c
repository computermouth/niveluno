// This rewrite increases the collision detection accuracy by using raycasts
// and ensures smooth movement on various terrain types, reducing clipping
// through steep surfaces and adjusting Mario’s position to prevent intersections
// with floors, walls, and ceilings. This approach aligns with Unreal
// Engine's MoveUpdatedComponent function, and it provides a more precise and
// consistent movement experience.


// perform_ground_step

// This function is responsible for moving Mario while he's grounded, factoring in slopes to adjust his movement.

//     Slope Factor Calculation:
//         Calculates Mario’s horizontal (XZ) velocity magnitude.
//         Uses a dot product to find the effect of the slope on his velocity.
//         Applies a slopeFactor, which adjusts Mario's movement based on the slope's angle, ensuring he moves up or down appropriately on sloped surfaces.

//     Updating Mario's Position:
//         After determining the slopeFactor, the code calculates intendedPos, adjusting Mario’s position along the XZ plane and moving him up or down the slope.

//     Step Execution:
//         Calls PerformStep to determine if Mario's intendedPos is valid based on collision checks. If it's not, adjustments will be made.
//         Copies m->pos to update the actual game object position.

s32 perform_ground_step(struct MarioState *m) {
    u32 stepResult;
    Vec3f intendedPos;
    Vec3f priorPos;
    vec3f_copy(priorPos, m->pos);
 
    // CuckyDev: Rewrite slope factor
    f32 mag = sqr(m->vel[2]) + sqr(m->vel[0]); // Get XZ magnitude (for division)
    if (mag > sqr(EPSILON)) {
        mag = sqrtf(mag);
        f32 dot = m->vel[0] * m->floor->normal.x + m->vel[2] * m->floor->normal.z; // Get Y factor
        f32 dotd = dot / mag;
        f32 slopeFactor = m->floor->normal.y
                          / sqrtf(sqr(m->floor->normal.y) + sqr(dotd)); // Convert Y factor to XZ factor
        intendedPos[0] = m->pos[0] + (m->vel[0]) * slopeFactor;
        intendedPos[2] = m->pos[2] + (m->vel[2]) * slopeFactor;
        intendedPos[1] = m->pos[1] - dot; // CuckyDev: Move Mario up/down slopes as he runs on them
    } else {
        intendedPos[0] = m->pos[0];
        intendedPos[2] = m->pos[2];
        intendedPos[1] = m->pos[1];
    }
 
    stepResult = PerformStep(m, intendedPos, STEP_SNAP_TO_FLOOR);
 
    vec3f_copy(((f32 *) &m->marioObj->OBJECT_FIELD_F32(O_POS_INDEX)), m->pos);
    vec3s_set(((s16 *) &m->marioObj->OBJECT_FIELD_S16(O_MOVE_ANGLE_INDEX, 3)), 0, m->faceAngle[1], 0);
    return stepResult;
}
 
 
// perform_air_step

// This function manages Mario’s position updates while he’s in the air.

//     Intended Position Calculation:
//         Calculates intendedPos based on Mario’s velocity components (XYZ).

//     Step Execution and Gravity:
//         Calls PerformStep to verify and finalize Mario's position.
//         If Mario isn't in a gravity-free state, gravity is applied.


s32 perform_air_step(struct MarioState *m, u32 stepArg) {
    Vec3f intendedPos;
    s32 j;
 
    for (j = 0; j < 3; j++) {
        intendedPos[j] = m->pos[j] + (m->vel[j]);
    }
    s32 stepResult = PerformStep(m, intendedPos, stepArg);
 
    if (m->vel[1] >= 0.0f) {
        m->peakHeight = m->pos[1];
    }
    if (!(stepArg & STEP_NO_GRAVITY)) {
        if (m->action != ACT_FLYING) {
            apply_gravity(m);
        }
    }
 
    vec3f_copy(((f32 *) &m->marioObj->OBJECT_FIELD_F32(O_POS_INDEX)), m->pos);
    vec3s_set(((s16 *) &m->marioObj->OBJECT_FIELD_S16(O_MOVE_ANGLE_INDEX, 3)), 0, m->faceAngle[1], 0);
 
    return stepResult;
}
 
 
 
 
 
// New system to verify mario's moves. Inspired by UE5's MoveUpdatedComponent function.
// Advantage:
// 1. Can no longer clip ceilings and steep floors
// 2. No more high speed clips
// 3. Consistently lands on steep floors
// 4. SM64 has an error up to 25% for moving mario. This has an error up to 1.56%.
// 5. Runs 4 collision calls per tick instead of 16 (95% of the time)
// 6. Consistent between swimming, aerial and ground step
// 7. Gets rid of quarterstep oddities
// todo: mario warps down into ceilings

//  MoveData

// This struct encapsulates information relevant to Mario’s movement:

//     Contains surfaces for walls, floor, and ceiling collisions.
//     GoalPos stores Mario’s desired destination.
//     IntendedPos holds the final valid position Mario can move to.
//     BiggestValidMove records how much Mario successfully moved.

// Movedata lets us pass by struct to reduce arg passing overhead
struct MoveData {
    struct Surface *HitSurface; // Raycast hit result
    struct Surface *Wall;
    struct Surface *Floor;
    struct Surface *Ceil;
    f32 IntendedPos[3]; // Position we believe to be a good enough approximation for where mario can go
    f32 GoalPos[3];     // Position we originally wanted to move towards
    f32 FloorHeight;
    f32 CeilHeight;
    f32 MarioHeight;
    s32 StepArgs;
    f32 BiggestValidMove; // How much we managed to move
};


// CheckMoveEndPosition

// This function checks Mario's final position for collisions.

//     Collision Adjustment:
//         Calculates MoveVector, the vector from Mario’s position to IntendedPos.
//         If MoveVector is non-zero, scales it to account for Mario's hitbox width.
//         Adjusts Mario’s Y position for accurate collision checking based on Mario’s height.
//     Raycasting for Collision:
//         Raycasts from Mario's position to ClipVector to detect potential collisions.
//         If a collision is detected, adjusts IntendedPos based on the collision normal to prevent Mario from clipping through walls or floors.

// Snap to the first collision in direction
ALWAYS_INLINE void CheckMoveEndPosition(struct MarioState *m, struct MoveData *MoveResult) {
    MoveResult->HitSurface = 0;
    Vec3f MoveVector;
    MoveVector[0] = MoveResult->IntendedPos[0] - m->pos[0];
    MoveVector[1] = MoveResult->IntendedPos[1] - m->pos[1];
    MoveVector[2] = MoveResult->IntendedPos[2] - m->pos[2];
    f32 MoveSize = vec3f_length(MoveVector);
    if (MoveSize > 0.0f) {
        // Scale up move size to account for mario's size
        f32 ScaledMoveSize = ((MoveSize + MARIOWIDENESS) / MoveSize);
        // Seperate clipvector saves us some multiplications down the line!
        Vec3f ClipVector;
        ClipVector[0] = MoveVector[0] * ScaledMoveSize;
        ClipVector[1] = MoveVector[1] * ScaledMoveSize;
        ClipVector[2] = MoveVector[2] * ScaledMoveSize;
 
        // Use the middle of Mario's to most represent his hitbox (idealls this would be a capsule cast)
        m->pos[1] += MARIOHEIGHT / 2;
        Vec3f HitPos;
        find_surface_on_ray(m->pos, ClipVector, &MoveResult->HitSurface, HitPos, 7);
        m->pos[1] -= MARIOHEIGHT / 2;
 
        // Clip if collision was found
        if (MoveResult->HitSurface != NULL) {
            const f32 DistanceMoved = sqrtf(sqr(HitPos[0] - MoveResult->IntendedPos[0])
                                            + sqr(HitPos[1]- MARIOHEIGHT / 2 - MoveResult->IntendedPos[1])
                                            + sqr(HitPos[2] - MoveResult->IntendedPos[2]));
            // move back either by as wide as mario is or the whole distance, whatever is less.
            const f32 MoveBackScale = (MIN(DistanceMoved, MARIOWIDENESS) / MoveSize);
            if (absf((MoveResult->HitSurface)->normal.y) <= WALLMAXNORMAL) {
                MoveResult->IntendedPos[0] = HitPos[0] - MoveVector[0] * MoveBackScale;
                MoveResult->IntendedPos[1] =
                    HitPos[1] - MoveVector[1] * MoveBackScale - MARIOHEIGHT / 2;
                MoveResult->IntendedPos[2] = HitPos[2] - MoveVector[2] * MoveBackScale;
            } else if ((MoveResult->HitSurface)->normal.y < 0.f) {
                // let the binary search find a good position towards mario's direction
                MoveResult->IntendedPos[0] = HitPos[0] + MoveResult->HitSurface->normal.x;
                MoveResult->IntendedPos[1] = HitPos[1] - MARIOHEIGHT / 2;
                MoveResult->IntendedPos[2] = HitPos[2] + MoveResult->HitSurface->normal.z;
            } else {
                MoveResult->IntendedPos[0] = HitPos[0];
                // Snap far enough down to guarantee find_floor will find a bigger value.
                MoveResult->IntendedPos[1] = HitPos[1] - ((f32) FLOOR_SNAP_OFFSET) / 2.f;
                MoveResult->IntendedPos[2] = HitPos[2];
            }
        }
    }
}

// FinishMove

// This final function updates Mario’s state based on the collision results from MoveData.

//     Updating Mario's Position and Collision State:
//         Sets Mario's floor, ceiling, and wall collision data.
//         Checks if Mario hits a ceiling during upward movement and removes his velocity if the ceiling is sloped toward him.
//         If IntendedPos is a valid ground position, finalizes Mario’s location there; otherwise, he remains airborne.
 
// Checks if the new position is valid.
s32 CheckMoveValid(struct MarioState *m, struct MoveData *MoveResult) {
    // Wall collisino happens first since walls will never prevent a move.
    MoveResult->Wall =
        resolve_and_return_wall_collisions(MoveResult->IntendedPos, (60.0f), MARIOWIDENESS);
    MoveResult->FloorHeight =
        find_floor_marioair(MoveResult->IntendedPos[0], MoveResult->IntendedPos[1],
                            MoveResult->IntendedPos[2], &MoveResult->Floor, m->vel[1]);
    // oob is invalid
    if (!MoveResult->Floor)
        return 0;
    // snap up early to make sure ceiling test happens from the right spot
    if ((MoveResult->StepArgs & STEP_SNAP_TO_FLOOR)
        && MoveResult->IntendedPos[1] < MoveResult->FloorHeight + FLOOR_SNAP_OFFSET) {
        MoveResult->IntendedPos[1] = MoveResult->FloorHeight;
    } else if (MoveResult->IntendedPos[1] < MoveResult->FloorHeight) {
        MoveResult->IntendedPos[1] = MoveResult->FloorHeight;
    }
    // ensure ice cap and shell works
    if (((m->action & ACT_FLAG_RIDING_SHELL) || (m->flags & MARIO_VANISH_CAP))
        && MoveResult->FloorHeight < m->waterLevel) {
        MoveResult->FloorHeight = m->waterLevel + ICEFLOWERWALKOFFSTE;
        MoveResult->Floor = &gWaterSurfacePseudoFloor;
        MoveResult->Floor->normal.w = m->waterLevel + ICEFLOWERWALKOFFSTE;
    }
    MoveResult->CeilHeight = vec3f_find_ceil(MoveResult->IntendedPos, &MoveResult->Ceil);
    // Mario does not fit here!
    if (MoveResult->FloorHeight + MoveResult->MarioHeight >= MoveResult->CeilHeight)
        return 0;
 
    return 1;
}
 
// Set Mario's data and determine the StepResult from the MoveResult.
s32 FinishMove(struct MarioState *m, struct MoveData *MoveResult) {
    m->floor = MoveResult->Floor;
    m->ceil = MoveResult->Ceil;
    m->wall = MoveResult->Wall;
    m->floorHeight = MoveResult->FloorHeight;
    m->ceilHeight = MoveResult->CeilHeight;
    vec3f_copy(m->pos, MoveResult->IntendedPos);
    m->TerrainSoundID = mario_get_terrain_sound_addend(m);
 
    const float CeilDist = m->ceilHeight - m->pos[1];
    if (CeilDist < MoveResult->MarioHeight) {
        const float MissingDist = MoveResult->MarioHeight - CeilDist;
        // Why am I dividing by 2 here? I don't know.
        m->pos[0] += m->ceil->normal.x * MissingDist/2;
        m->pos[1] += m->ceil->normal.y * MissingDist/2;
        m->pos[2] += m->ceil->normal.z * MissingDist/2;
        if ((MoveResult->StepArgs & STEP_CHECK_HANG) && m->ceil != NULL
            && ((m->ceil->type & (SPECFLAG_HANGABLE << 8)))) {
            m->vel[1] = 0.0f;
            return STEP_GRAB_CEILING;
        }
        // bonk mario if the ceiling is sloped towards him.
        // use the same angle as a wall would for consistency.
        f32 VelocitySize = vec3f_length(m->vel);
        m->inertia[1] = 0;
        if (VelocitySize > 0.f) {
            const f32 DotBetweenCeilAndMario = vec3f_dot(m->vel, &m->ceil->normal.x) / VelocitySize;
            float DotProduct = m->vel[0] * m->ceil->normal.x + m->vel[1] * m->ceil->normal.y
                               + m->vel[2] * m->ceil->normal.z;
            m->vel[0] -= DotProduct * m->ceil->normal.x;
            m->vel[1] -= MAX(0, DotProduct * m->ceil->normal.y);
            m->vel[2] -= DotProduct * m->ceil->normal.z;
            if (DotBetweenCeilAndMario <= CEILING_BONK_DOT && VelocitySize >= WALLKICK_MIN_VEL) {
                // if hitting a ceiling, just remove velocity
                return STEP_HIT_WALL;
            }
        }
    }
    // if we are not set to snap to the floor but landed despite that, on ground takes priority!
    if (!(MoveResult->StepArgs & STEP_SNAP_TO_FLOOR) && (m->pos[1] <= m->floorHeight))
        return STEP_ON_GROUND;
 
    if (m->wall) {
        if (m->wall->type & (SPECFLAG_BURNING << 8)) {
            return STEP_HIT_LAVA;
        }
        if (MoveResult->StepArgs & STEP_CHECK_LEDGE_GRAB) {
            if (check_ledge_grab(m, m->wall, MoveResult->GoalPos, MoveResult->IntendedPos)) {
                return STEP_GRAB_LEDGE;
            }
        }
        u16 WallAngleMaxDiff = MoveResult->StepArgs & STEP_SNAP_TO_FLOOR
                                   ? 0x8000 - MAX_ANGLE_DIFF_FOR_WALL_COLLISION_ON_GROUND
                                   : 0x8000 - MAX_ANGLE_DIFF_FOR_WALL_COLLISION_IN_AIR;
        if (absi((s16) (atan2s(m->wall->normal.z, m->wall->normal.x) - m->faceAngle[1]))
            >= WallAngleMaxDiff) {
            return STEP_HIT_WALL;
        }
    }
 
    // If we haven't moved, we hit either oob or a ceiling.
#define ZERO_POINT_FIVE_TO_THE_POWER_OF_MINUS_NUM_SEARCHES 0.015625f
    if (MoveResult->BiggestValidMove < ZERO_POINT_FIVE_TO_THE_POWER_OF_MINUS_NUM_SEARCHES) {
        return STEP_HIT_WALL;
    }
 
    return m->pos[1] <= m->floorHeight ? STEP_ON_GROUND : STEP_IN_AIR;
}
// Scales the move. The Y is assumed to always be valid (if not, we are ceiling bonking anyway)
s32 ScaleMove(struct MarioState *m, struct MoveData *MoveResult, f32 Scale) {
    MoveResult->IntendedPos[0] = (MoveResult->GoalPos[0] - m->pos[0]) * Scale + m->pos[0];
    MoveResult->IntendedPos[1] = MoveResult->GoalPos[1];
    MoveResult->IntendedPos[2] = (MoveResult->GoalPos[2] - m->pos[2]) * Scale + m->pos[2];
}
// Performs a generic step and returns the step result
// [StepArgs] checks for special interactions like ceilings, ledges and floor snapping
s32 PerformStep(struct MarioState *m, Vec3f GoalPos, const s32 StepArgs) {
    struct MoveData MoveResult;
    MoveResult.MarioHeight = (m->action & ACT_FLAG_SHORT_HITBOX) ? MARIOHEIGHT / 2.f : MARIOHEIGHT;
    MoveResult.StepArgs = StepArgs;
    vec3f_copy(MoveResult.IntendedPos, GoalPos);
    s32 IterationsRemaining = 2;
DoItAgain:
    CheckMoveEndPosition(m, &MoveResult);
    vec3f_copy(MoveResult.GoalPos, MoveResult.IntendedPos);
 
    // If the move is outright valid (VAST MAJORITY OF MOVES), just exit instantly.
    if (CheckMoveValid(m, &MoveResult)) {
        if (MoveResult.HitSurface) {
            struct Surface *HitSurface;
            Vec3f HitPos;
            Vec3f ClipVector;
            ClipVector[0] = MoveResult.GoalPos[0] - m->pos[0];
            // move back up because floors in HitSurface move mario down (ensures snapping)
            ClipVector[1] =
                MoveResult.GoalPos[1] - m->pos[1]
                + (MoveResult.HitSurface->normal.y > WALLMAXNORMAL ? FLOOR_SNAP_OFFSET / 2.f + 4.f
                                                                   : 0.f);
            ClipVector[2] = MoveResult.GoalPos[2] - m->pos[2];
            find_surface_on_ray(m->pos, ClipVector, &HitSurface, HitPos, 7);
            // Ensure nothing moved mario's feet through a surface.
            // (Ledgegrabs may teleport mario, but they happen in FinishMove)
            if (HitSurface) {
                // Give it another try, we do want to move as much as possible.
                vec3f_copy(MoveResult.GoalPos, HitPos);
                IterationsRemaining--;
                if (IterationsRemaining)
                    goto DoItAgain;
                // No valid moves managed to be made. Emergency exit!
                return STEP_HIT_WALL;
            }
        }
        // Full move happened
        MoveResult.BiggestValidMove = 1.f;
        return FinishMove(m, &MoveResult);
    }
    // Move was unsuccessful. Scale it down to a precision of 2^-NUM_SEARCHES
    f32 CurrentMoveSize = 0.5f;
    MoveResult.BiggestValidMove = 0.f;
#define NUM_SEARCHES 6
    for (s32 BinarySplitsReamining = NUM_SEARCHES; BinarySplitsReamining > 0; BinarySplitsReamining--) {
        ScaleMove(m, &MoveResult, MoveResult.BiggestValidMove + CurrentMoveSize);
        if (CheckMoveValid(m, &MoveResult)) {
            MoveResult.BiggestValidMove += CurrentMoveSize;
        }
        CurrentMoveSize *= 0.5f;
    }
    ScaleMove(m, &MoveResult, MoveResult.BiggestValidMove);
    // No valid move can be made. We are stuck OOB.
    // This should only happen if a platform OOB teleported away.
    // Mario should die here.
    if (!CheckMoveValid(m, &MoveResult)) {
        return STEP_HIT_WALL;
    }
    // We've moved, but not the full distance.
    return FinishMove(m, &MoveResult);
}
