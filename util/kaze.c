// This rewrite increases the collision detection accuracy by using raycasts
// and ensures smooth movement on various terrain types, reducing clipping
// through steep surfaces and adjusting Mario’s position to prevent intersections
// with floors, walls, and ceilings. This approach aligns with Unreal
// Engine's MoveUpdatedComponent function, and it provides a more precise and
// consistent movement experience.

#include <stdint.h>
#include <math.h>
#include <stdbool.h>
#include <stdlib.h>

#define MAX(a, b) (((a) > (b)) ? (a) : (b))
#define MIN(a, b) (((a) < (b)) ? (a) : (b))

typedef float Vec3f[3];
typedef int16_t Vec3s[3];

struct Surface {
    struct {
        float x;
        float y;
        float z;
    } normal;
    int16_t vertex1[3];
    int16_t vertex2[3];
    int16_t vertex3[3];
    // pre-caclulated vertical bounds
    float lowerY;
    float upperY;
};

struct SurfaceNode {
    struct Surface *surface;
    uint32_t SurfaceIndex;
    uint32_t NextIndex;
};

struct Object {
	Vec3f pos;
	Vec3s moveAngle;
};

struct MarioState {
    uint32_t flags;
    uint32_t action;
    uint32_t prevAction;
    Vec3s faceAngle;
    Vec3f pos;
    Vec3f vel;
    struct Surface *wall;
    struct Surface *ceil;
    struct Surface *floor;
    float ceilHeight;
    float floorHeight;
    struct Object *marioObj;
    float peakHeight;
};

struct MarioState * gMarioState;
int32_t gSurfacePartitions[1][1][1][1];

float vec3f_length(Vec3f dest);
void *vec3s_set(Vec3s dest, uint16_t x, uint16_t y, uint16_t z);
void *vec3f_copy(Vec3f dest, Vec3f src);
float vec3f_find_ceil(Vec3f pos, struct Surface **ceil);
float sqr(float f);
float absf(float f);
void find_surface_on_ray(Vec3f orig, Vec3f dir, struct Surface **hit_surface, Vec3f hit_pos,
                         int32_t cellFlags);
float find_floor(float xPos, float yPos, float zPos, struct Surface **pfloor);
float find_floor_marioair(float xPos, float yPos, float zPos, struct Surface **pfloor, float yVel);
int32_t absi(int32_t x);
int16_t atan2s(float y, float x);
void *vec3f_cross(Vec3f dest, Vec3f a, Vec3f b);
float vec3f_dot(Vec3f, float *);
void *vec3f_sum(Vec3f dest, Vec3f a, Vec3f b);

struct SurfaceNode * NODEFROMID(int32_t);
struct Surface * SURFACEFROMID(uint32_t);
uint32_t SURFACETYPE(struct Surface*);

#define EPSILON 0.000001f
#define MARIOHEIGHT 3.0f
#define MARIOWIDENESS 2.0f

#define WALLMAXNORMAL 0.000001f
#define FLOOR_SNAP_OFFSET 0.000001f
#define CEILING_BONK_DOT 0.000001f
#define WALLKICK_MIN_VEL 0.000001f
#define STEP_HIT_WALL 1
#define STEP_ON_GROUND 1
#define MAX_ANGLE_DIFF_FOR_WALL_COLLISION_IN_AIR 0.000001f
#define MAX_ANGLE_DIFF_FOR_WALL_COLLISION_ON_GROUND 0.000001f
#define STEP_IN_AIR 1
#define ACT_FLAG_SHORT_HITBOX 1
#define MARIO_VANISH_CAP 1
#define SPATIAL_PARTITION_WALLS 0
#define STATIC_COLLISION 0
#define DYNAMIC_COLLISION 0
#define SPATIAL_PARTITION_CEILS 0
#define SPATIAL_PARTITION_FLOORS 0

#define LEVEL_BOUNDARY_MAX  0
#define CELL_SIZE 0
#define ONE 1

int32_t PerformStep(struct MarioState *m, Vec3f GoalPos, bool SnapToFloor);

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

int32_t perform_ground_step(struct MarioState *m) {
    uint32_t stepResult;
    Vec3f intendedPos;
    Vec3f priorPos;
    vec3f_copy(priorPos, m->pos);
 
    // CuckyDev: Rewrite slope factor
    float mag = sqr(m->vel[2]) + sqr(m->vel[0]); // Get XZ magnitude (for division)
    if (mag > sqr(EPSILON)) {
        mag = sqrtf(mag);
        float dot = m->vel[0] * m->floor->normal.x + m->vel[2] * m->floor->normal.z; // Get Y factor
        float dotd = dot / mag;
        float slopeFactor = m->floor->normal.y
                          / sqrtf(sqr(m->floor->normal.y) + sqr(dotd)); // Convert Y factor to XZ factor
        intendedPos[0] = m->pos[0] + (m->vel[0]) * slopeFactor;
        intendedPos[2] = m->pos[2] + (m->vel[2]) * slopeFactor;
        intendedPos[1] = m->pos[1] - dot; // CuckyDev: Move Mario up/down slopes as he runs on them
    } else {
        intendedPos[0] = m->pos[0];
        intendedPos[2] = m->pos[2];
        intendedPos[1] = m->pos[1];
    }
 
    stepResult = PerformStep(m, intendedPos, true);

    m->marioObj->pos[0] = m->pos[0];
    m->marioObj->pos[1] = m->pos[1];
    m->marioObj->pos[2] = m->pos[2];

    m->marioObj->moveAngle[0] = 0;
    m->marioObj->moveAngle[1] = m->faceAngle[1];
    m->marioObj->moveAngle[2] = 0;
    return stepResult;
}
 
 
// perform_air_step

// This function manages Mario’s position updates while he’s in the air.

//     Intended Position Calculation:
//         Calculates intendedPos based on Mario’s velocity components (XYZ).

//     Step Execution and Gravity:
//         Calls PerformStep to verify and finalize Mario's position.
//         If Mario isn't in a gravity-free state, gravity is applied.
void apply_gravity(struct MarioState *m);

int32_t perform_air_step(struct MarioState *m, uint32_t stepArg) {
    Vec3f intendedPos;
    int32_t j;
 
    for (j = 0; j < 3; j++) {
        intendedPos[j] = m->pos[j] + (m->vel[j]);
    }
    int32_t stepResult = PerformStep(m, intendedPos, stepArg);
 
    if (m->vel[1] >= 0.0f) {
        m->peakHeight = m->pos[1];
    }
    apply_gravity(m);
 
    m->marioObj->pos[0] = m->pos[0];
    m->marioObj->pos[1] = m->pos[1];
    m->marioObj->pos[2] = m->pos[2];

    m->marioObj->moveAngle[0] = 0;
    m->marioObj->moveAngle[1] = m->faceAngle[1];
    m->marioObj->moveAngle[2] = 0;
 
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
    float IntendedPos[3]; // Position we believe to be a good enough approximation for where mario can go
    float GoalPos[3];     // Position we originally wanted to move towards
    float FloorHeight;
    float CeilHeight;
    float MarioHeight;
    bool SnapToFloor;
    float BiggestValidMove; // How much we managed to move
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
void CheckMoveEndPosition(struct MarioState *m, struct MoveData *MoveResult) {
    MoveResult->HitSurface = NULL;
    Vec3f MoveVector;
    MoveVector[0] = MoveResult->IntendedPos[0] - m->pos[0];
    MoveVector[1] = MoveResult->IntendedPos[1] - m->pos[1];
    MoveVector[2] = MoveResult->IntendedPos[2] - m->pos[2];
    float MoveSize = vec3f_length(MoveVector);
    if (MoveSize > 0.0f) {
        // Seperate clipvector saves us some multiplications down the line!
        Vec3f ClipVector;
        ClipVector[0] = MoveVector[0] * MoveSize;
        ClipVector[1] = MoveVector[1] * MoveSize;
        ClipVector[2] = MoveVector[2] * MoveSize;
 
        // Use the middle of Mario's to most represent his hitbox (idealls this would be a capsule cast)
        m->pos[1] += MARIOHEIGHT / 2;
        Vec3f HitPos;
        find_surface_on_ray(m->pos, ClipVector, &MoveResult->HitSurface, HitPos, 7);
        m->pos[1] -= MARIOHEIGHT / 2;
 
        // Clip if collision was found
        if (MoveResult->HitSurface != NULL) {
            const float DistanceMoved = sqrtf(sqr(HitPos[0] - MoveResult->IntendedPos[0])
                                            + sqr(HitPos[1]- MARIOHEIGHT / 2 - MoveResult->IntendedPos[1])
                                            + sqr(HitPos[2] - MoveResult->IntendedPos[2]));
            // move back either by as wide as mario is or the whole distance, whatever is less.
            const float MoveBackScale = (MIN(DistanceMoved, MARIOWIDENESS) / MoveSize);
            if (fabsf((MoveResult->HitSurface)->normal.y) <= WALLMAXNORMAL) {
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
                MoveResult->IntendedPos[1] = HitPos[1] - ((float) FLOOR_SNAP_OFFSET) / 2.f;
                MoveResult->IntendedPos[2] = HitPos[2];
            }
        }
    }
}
 
struct Surface *resolve_and_return_wall_collisions(Vec3f pos, float offset, float rad);

// FinishMove

// This final function updates Mario’s state based on the collision results from MoveData.

//     Updating Mario's Position and Collision State:
//         Sets Mario's floor, ceiling, and wall collision data.
//         Checks if Mario hits a ceiling during upward movement and removes his velocity if the ceiling is sloped toward him.
//         If IntendedPos is a valid ground position, finalizes Mario’s location there; otherwise, he remains airborne.

// Checks if the new position is valid.
int32_t CheckMoveValid(struct MarioState *m, struct MoveData *MoveResult) {
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
    if ((MoveResult->SnapToFloor)
        && MoveResult->IntendedPos[1] < MoveResult->FloorHeight + FLOOR_SNAP_OFFSET) {
        MoveResult->IntendedPos[1] = MoveResult->FloorHeight;
    } else if (MoveResult->IntendedPos[1] < MoveResult->FloorHeight) {
        MoveResult->IntendedPos[1] = MoveResult->FloorHeight;
    }
    MoveResult->CeilHeight = vec3f_find_ceil(MoveResult->IntendedPos, &MoveResult->Ceil);
    // Mario does not fit here!
    if (MoveResult->FloorHeight + MoveResult->MarioHeight >= MoveResult->CeilHeight)
        return 0;
 
    return 1;
}
 
// Set Mario's data and determine the StepResult from the MoveResult.
int32_t FinishMove(struct MarioState *m, struct MoveData *MoveResult) {
    m->floor = MoveResult->Floor;
    m->ceil = MoveResult->Ceil;
    m->wall = MoveResult->Wall;
    m->floorHeight = MoveResult->FloorHeight;
    m->ceilHeight = MoveResult->CeilHeight;
    vec3f_copy(m->pos, MoveResult->IntendedPos);
 
    const float CeilDist = m->ceilHeight - m->pos[1];
    if (CeilDist < MoveResult->MarioHeight) {
        const float MissingDist = MoveResult->MarioHeight - CeilDist;
        // Why am I dividing by 2 here? I don't know.
        m->pos[0] += m->ceil->normal.x * MissingDist/2;
        m->pos[1] += m->ceil->normal.y * MissingDist/2;
        m->pos[2] += m->ceil->normal.z * MissingDist/2;
        // bonk mario if the ceiling is sloped towards him.
        // use the same angle as a wall would for consistency.
        float VelocitySize = vec3f_length(m->vel);
        // m->inertia[1] = 0;
        if (VelocitySize > 0.f) {
            const float DotBetweenCeilAndMario = vec3f_dot(m->vel, &m->ceil->normal.x) / VelocitySize;
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
    if (!(MoveResult->SnapToFloor) && (m->pos[1] <= m->floorHeight))
        return STEP_ON_GROUND;
 
    if (m->wall) {
        uint16_t WallAngleMaxDiff = MoveResult->SnapToFloor
                                   ? 0x8000 - MAX_ANGLE_DIFF_FOR_WALL_COLLISION_ON_GROUND
                                   : 0x8000 - MAX_ANGLE_DIFF_FOR_WALL_COLLISION_IN_AIR;
        if (absi((int16_t) (atan2s(m->wall->normal.z, m->wall->normal.x) - m->faceAngle[1]))
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
void ScaleMove(struct MarioState *m, struct MoveData *MoveResult, float Scale) {
    MoveResult->IntendedPos[0] = (MoveResult->GoalPos[0] - m->pos[0]) * Scale + m->pos[0];
    MoveResult->IntendedPos[1] = MoveResult->GoalPos[1];
    MoveResult->IntendedPos[2] = (MoveResult->GoalPos[2] - m->pos[2]) * Scale + m->pos[2];
}

// Performs a generic step and returns the step result
// [SnapToFloor] checks for special interactions like ceilings, ledges and floor snapping
int32_t PerformStep(struct MarioState *m, Vec3f GoalPos, bool SnapToFloor) {
    struct MoveData MoveResult;
    MoveResult.MarioHeight = (m->action & ACT_FLAG_SHORT_HITBOX) ? MARIOHEIGHT / 2.f : MARIOHEIGHT;
    MoveResult.SnapToFloor = SnapToFloor;
    vec3f_copy(MoveResult.IntendedPos, GoalPos);
    int32_t IterationsRemaining = 2;
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
    float CurrentMoveSize = 0.5f;
    MoveResult.BiggestValidMove = 0.f;
#define NUM_SEARCHES 6
    for (int32_t BinarySplitsReamining = NUM_SEARCHES; BinarySplitsReamining > 0; BinarySplitsReamining--) {
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


void vec3f_vec3s_diff(float *dest, int16_t *a, int16_t *b) {
    dest[0] = (a[0]) - b[0];
    dest[1] = (a[1]) - b[1];
    dest[2] = (a[2]) - b[2];
}

// Checks whether a given ray (starting at orig and extending in dir up to dir_length) intersects with the 
// specified triangle surface. If it does, the intersection point is stored in hit_pos, and the distance 
// from the ray origin to the hit point is returned. If no intersection occurs, returns 0.

//     Uses the Möller–Trumbore ray-triangle intersection algorithm.
//     Performs bounds checks to ensure the intersection lies within the triangle and is in front of the ray origin.

float ray_surf_intersect(Vec3f orig, Vec3f dir, float dir_length, Vec3f hit_pos,
                                     struct Surface *surface) {
    float length;
    int32_t i;
    Vec3f e1, e2, h, s;
    vec3f_vec3s_diff(e1, surface->vertex2, surface->vertex1);
    vec3f_vec3s_diff(e2, surface->vertex3, surface->vertex1);
    // Reject if determinant is too small or surface is facing away from the ray's direction
    vec3f_cross(h, dir, e2);
    float det = vec3f_dot(e1, h);
    if (det < EPSILON)
        return 0;
    s[0] = orig[0] - surface->vertex1[0];
    s[1] = orig[1] - surface->vertex1[1];
    s[2] = orig[2] - surface->vertex1[2];
    float u = vec3f_dot(s, h);
    // Reject if the intersection point is outside the triangle
    if ((u < 0.0f) || (u > det))
        return 0;
    vec3f_cross(h, s, e1);
    float v = vec3f_dot(dir, h);
    // Reject if the intersection point is outside the triangle
    if ((v < 0.0f) || ((u + v) > det)) {
        return 0;
    }
    // Reject if the intersection distance is beyond the ray's length
    length = vec3f_dot(e2, h) / det;
    if ((length > dir_length)) {
        return 0;
    }
    for (i = 0; i < 3; i++) {
        hit_pos[i] = orig[i] + dir[i] * length;
    }
    return length;
}

// Iterates over a list of surfaces (triangles) starting from NodeIndex,
// checking for intersections with a ray defined by orig, dir, and capped by *max_length.
// If a closer hit is found, updates *hit_surface, hit_pos, and *max_length accordingly.

// This filters and tests potential hit surfaces in one spatial partition cell.
void find_surface_on_ray_list(int32_t NodeIndex, Vec3f orig, Vec3f dir, struct Surface **hit_surface,
                              Vec3f hit_pos, float *max_length) {
    float length;
    Vec3f chk_hit_pos;
    float top, bottom;
 
    // Get upper and lower bounds of ray
    if (dir[1] >= 0.0f) {
        top = orig[1] + dir[1] * *max_length;
        bottom = orig[1];
    } else {
        top = orig[1];
        bottom = orig[1] + dir[1] * *max_length;
    }
 
    // Iterate through every surface of the list
    for (; NodeIndex != 0xFFFF;) {
        struct SurfaceNode *list = NODEFROMID(NodeIndex);
        struct Surface *Surface = SURFACEFROMID(list->SurfaceIndex);
        NodeIndex = list->NextIndex;
        // Reject surface if out of vertical bounds
        if (Surface->lowerY > top || Surface->upperY < bottom)
            continue;
 
        if (((SURFACETYPE(Surface) != 0x00AD) || (gMarioState->flags & MARIO_VANISH_CAP))) {
            if ((length = ray_surf_intersect(orig, dir, *max_length, chk_hit_pos, Surface) > 0)) {
                if (length <= *max_length) {
                    *hit_surface = Surface;
                    vec3f_copy(hit_pos, chk_hit_pos);
                    *max_length = length;
                }
            }
        }
    }
}

// Performs ray-surface intersection checks for all surfaces within the specified spatial grid cell
// (cellX, cellZ) using find_surface_on_ray_list, depending on cellFlags (bitmask of floor, wall, and ceiling surface types).
// Updates the closest hit accordingly.
// Used as part of a broader ray traversal through the spatial partition grid.
void find_surface_on_ray_cell(int32_t cellX, int32_t cellZ, Vec3f orig, Vec3f normalized_dir,
                                     struct Surface **hit_surface, Vec3f hit_pos, float *max_length,
                                     uint8_t cellFlags) {
    // Skip if OOB
    if (((cellX & 0x3F) == cellX) && ((cellZ & 0x3F) == cellZ)) {
        // Iterate through each surface in this partition
        if (cellFlags & 2) {
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_WALLS][STATIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_WALLS][DYNAMIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
        }
        if (cellFlags & 4) {
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_CEILS][STATIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_CEILS][DYNAMIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
        }
        if (cellFlags & 1) {
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_FLOORS][STATIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
            find_surface_on_ray_list(
                (gSurfacePartitions[cellZ][cellX][SPATIAL_PARTITION_FLOORS][DYNAMIC_COLLISION]), orig,
                normalized_dir, hit_surface, hit_pos, max_length);
        }
    }
}
 
/// Make 'dest' the sum of vectors a and b.
void vec3f_sum_coll(Vec3f dest, Vec3f a, Vec3f b) {
    register float *temp = dest;
    register float x, y;
    while (temp < dest + 3) {
        x = *a;
        a++;
        y = *b;
        b++;
        *temp = x + y;
        temp++;
    }
}

// Performs a raycast from orig in the direction dir, using a grid traversal algorithm 
// (similar to Amanatides and Woo's voxel traversal) to test for surface intersections along the ray path.
// It updates *hit_surface and hit_pos with the first valid intersection found, considering the given
// cellFlags for filtering floor/wall/ceiling surfaces.

//    Normalize direction.
//    Calculate ray length and convert positions to cell-space.
//    Traverse the grid, cell by cell, using DDA (Digital Differential Analyzer)-style stepping.
//    Within each cell, call find_surface_on_ray_cell to test for collisions.

void find_surface_on_ray(Vec3f orig, Vec3f dir, struct Surface **hit_surface, Vec3f hit_pos,
                         int32_t cellFlags) {
    Vec3f normalized_dir;
 
    // Set that no surface has been hit
    *hit_surface = NULL;
    vec3f_sum(hit_pos, orig, dir);
 
    // Get normalized direction
    float max_length = vec3f_length(dir);
    if (max_length == 0.f)
        return;
    normalized_dir[0] = dir[0] / max_length;
    normalized_dir[1] = dir[1] / max_length;
    normalized_dir[2] = dir[2] / max_length;
 
    // Get the start and end coords converted to cell-space
    float start_cell_coord_x = (orig[0] + LEVEL_BOUNDARY_MAX) / (float) CELL_SIZE;
    float start_cell_coord_z = (orig[2] + LEVEL_BOUNDARY_MAX) / (float) CELL_SIZE;
 
    // "A Fast Voxel Traversal Algorithm for Ray Tracing" - John Amanatides & Andrew Woo
    // Adapted from implementation at https://www.shadertoy.com/view/XddcWn
    float rdinv_x;
    if (absf(dir[0]) > EPSILON)
        rdinv_x = (float) CELL_SIZE / dir[0];
    else
        rdinv_x = 65536.f;
    float rdinv_z;
    if (absf(dir[2]) > EPSILON)
        rdinv_z = (float) CELL_SIZE / dir[2];
    else
        rdinv_z = 65536.f;
    float p_x = (int32_t) start_cell_coord_x;
    float p_z = (int32_t) start_cell_coord_z;
    float stp_x = rdinv_x > 0.f ? ONE : -ONE;
    float stp_z = rdinv_z > 0.f ? ONE : -ONE;
    float delta_x = MIN(absf(rdinv_x), ONE);
    float delta_z = MIN(absf(rdinv_z), ONE);
    float t_max_x = absf((p_x + MAX(stp_x, 0.0f) - start_cell_coord_x) * rdinv_x);
    float t_max_z = absf((p_z + MAX(stp_z, 0.0f) - start_cell_coord_z) * rdinv_z);
 
    while (1) {
        find_surface_on_ray_cell((int32_t) p_x, (int32_t) p_z, orig, normalized_dir, hit_surface, hit_pos,
                                 &max_length, cellFlags);
        float t_next = MIN(t_max_x, t_max_z);
        if (t_next > ONE) {
            break;
        }
 
        if (t_max_x < t_max_z) {
            t_max_x += delta_x;
            p_x += stp_x;
        } else {
            t_max_z += delta_z;
            p_z += stp_z;
        }
    }
}
