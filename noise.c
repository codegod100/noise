// main.c
#include "fenster.h"
#define W 320
#define H 240
#include <math.h>

#define NOISE_W 512
#define NOISE_H 512

int main() {
  uint32_t buf[W * H];
  struct fenster f = { .title = "hello", .width = W, .height = H, .buf = buf };
  uint32_t noise_map[NOISE_W][NOISE_H];
  for (int i = 0; i < NOISE_W; i++) {
    for (int j = 0; j < NOISE_H; j++) {
      noise_map[i][j] = rand();
    }
  }
  double t = 0;
  fenster_open(&f);
  while (fenster_loop(&f) == 0) {
    t += 0.01;
    for (int i = 0; i < W; i++) {
      for (int j = 0; j < H; j++) {
        if (i > W / 2 - 30 && i < W / 2 + 30 && j > H / 2 - 30 && j < H / 2 + 30) {
          double x = i - W / 2.0;
          double y = j - H / 2.0;
          double r = sqrt(x * x + y * y);
          double angle = atan2(y, x);
          double strength = 80.0;
          double new_angle = angle + strength / r + t;
          int u = (int)(W / 2.0 + r * cos(new_angle)) % NOISE_W;
          int v = (int)(H / 2.0 + r * sin(new_angle)) % NOISE_H;
          fenster_pixel(&f, i, j) = noise_map[u][v];
        } else {
          fenster_pixel(&f, i, j) = rand();
        }
      }
    }
  }
  fenster_close(&f);
  return 0;
}
