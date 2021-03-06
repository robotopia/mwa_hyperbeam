// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Build and run with something like:
// gcc -O3 -I ../include/ -L ../target/release/ -l mwa_hyperbeam ./beam_calcs_many.c -o beam_calcs_many
// LD_LIBRARY_PATH=../target/release ./beam_calcs_many ../mwa_full_embedded_element_pattern.h5

#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "mwa_hyperbeam.h"

int main(int argc, char *argv[]) {
    if (argc == 1) {
        fprintf(stderr, "Expected one argument - the path to the HDF5 file.\n");
        exit(1);
    }

    // Get a new beam from hyperbeam.
    FEEBeam *beam = new_fee_beam(argv[1]);

    // Set up the pointings to test.
    int num_pointings = 10000;
    double *az = malloc(num_pointings * sizeof(double));
    double *za = malloc(num_pointings * sizeof(double));
    for (int i = 0; i < num_pointings; i++) {
        double coord_deg = 5.0 + (double)i * 80.0 / (double)num_pointings;
        double coord_rad = coord_deg * M_PI / 180.0;
        az[i] = coord_rad;
        za[i] = coord_rad;
    }
    unsigned delays[16] = {3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0, 3, 2, 1, 0};
    double amps[16] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1};
    int freq_hz = 51200000;
    int zenith_norm = 0;

    // Calculate the Jones matrices for all pointings. Rust will do this in
    // parallel.
    double **jones = calc_jones_array(beam, num_pointings, az, za, freq_hz, delays, amps, zenith_norm);
    printf("The first Jones matrix:\n");
    printf("[[%.8f %.8fi,", jones[0][0], jones[0][1]);
    printf("  %.8f %.8fi]\n", jones[0][2], jones[0][3]);
    printf(" [%.8f %.8fi,", jones[0][4], jones[0][5]);
    printf("  %.8f %.8fi]]\n", jones[0][6], jones[0][7]);

    // Free the Jones matrices.
    free(jones);
    // Free the beam - we must use a special function to do this.
    free_fee_beam(beam);

    return 0;
}
