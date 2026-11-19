# RustOS Games Suite Manual

RustOS features three playable games accessible directly from the Visual Desktop Application Menu (or via the `games` command in the shell).

---

## 1. Snake Arcade

### Overview
A recreation of the classic arcade snake game built on VGA text mode characters and timed using hardware PIT timer ticks.

### Rules & Mechanics
- The snake starts in the center of the arena moving to the right.
- A food item (`*`) spawns at a pseudo-random coordinate inside the arena, seeded from the PIT tick counter and CMOS RTC seconds.
- Colliding with the food:
  - Increases the score by **+10 points**.
  - Increases the length of the snake by 1 segment.
  - Spawns a new food item at a free cell.
- The game runs on a steady tick loop (~80 ms per move).
- **Collision & Game Over**:
  - Hitting any of the outer boundary walls ends the game.
  - Running into any segment of the snake's own body ends the game.
- **Controls**:
  - `Up Arrow`: Steer Up (cannot reverse directly down).
  - `Down Arrow`: Steer Down.
  - `Left Arrow`: Steer Left.
  - `Right Arrow`: Steer Right.
  - `Enter`: Play again (when Game Over dialog appears).
  - `Esc`: Return to Desktop Menu at any time.

---

## 2. Snake and Ladders

### Overview
A turn-based board game between a **Human Player `[H]`** and a **Computer AI `[C]`** played across a 10x10 grid with numbered squares from 1 to 100.

### Rules & Mechanics
- **Board Grid**: 10 rows and 10 columns arranged in serpentine order (square 1 at bottom-left, square 10 at bottom-right, square 11 above 10 moving left, through square 100 at top-left).
- **Ladders (Green `^L^`)**:
  - Square 4  -> 14
  - Square 9  -> 31
  - Square 20 -> 38
  - Square 28 -> 84
  - Square 40 -> 59
  - Square 63 -> 81
- **Snakes (Red `vSv`)**:
  - Square 17 -> 7
  - Square 54 -> 34
  - Square 62 -> 19
  - Square 64 -> 60
  - Square 87 -> 24
  - Square 95 -> 75
- **Gameplay**:
  1. Human player rolls a 6-sided dice (values 1 to 6 from Xorshift32 PRNG) by pressing `Space` or `Enter`.
  2. Player marker `[H]` advances by the dice roll amount.
  3. If landing on a ladder foot, player climbs to the ladder top. If landing on a snake head, player slides down to the snake tail.
  4. Computer AI player `[C]` rolls automatically after a 600 ms delay.
  5. The right sidebar displays the active turn, the last dice roll, and a scrolling log of recent moves.
  6. **Victory**: The first player to reach or exceed square 100 wins the game.
- **Controls**:
  - `Space` or `Enter`: Roll dice.
  - `Esc`: Return to Desktop Menu.

---

## 3. Car Dodge (Highway Survival)

### Overview
A fast-paced reflex driving game featuring a 3-lane vertical highway and falling obstacles.

### Rules & Mechanics
- **The Road**: 3 lanes (Left, Middle, Right) divided by road markings and guardrails.
- **The Player Car**: Steers horizontally across the lanes at row 21:
  ```
   /==\
  |####|
  ```
- **Obstacles**: Roadblocks `[XXXX]` fall from the top of the screen toward the bottom.
- **Dynamic Speed**:
  - Obstacle fall speed accelerates as the player's score increases.
  - Each survived tick adds **+1 point**.
  - Each avoided obstacle that scrolls off the bottom of the screen adds **+10 points**.
- **Session High Score**: The game maintains the best score achieved across game sessions.
- **Collision & Game Over**:
  - If an obstacle reaches the player's car row in the same lane, a crash occurs.
  - The Game Over dialog displays the final score and session high score.
- **Controls**:
  - `Left Arrow`: Steer one lane left.
  - `Right Arrow`: Steer one lane right.
  - `Enter`: Race again (after crash).
  - `Esc`: Exit to Desktop Menu.
