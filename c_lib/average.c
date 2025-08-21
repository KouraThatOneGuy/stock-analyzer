// filepath: /Users/admin/Documents/stock-analyzer/c_lib/average.c
#include "average.h"

float moving_average(float *data, int length) {
    if (length == 0) return 0.0;
    float sum = 0.0;
    for (int i = 0; i < length; i++) {
        sum += data[i];
    }
    return sum / length;
}