// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
Benchmarks. FEE beam code benchmarks rely on the HDF5 file being present in the
project's root directory.
 */

use criterion::*;

use mwa_hyperbeam::*;

// These benchmarks rely on aspects of `FEEBeam` being made public. These should
// normally be left as private.
fn fee(c: &mut Criterion) {
    let mut beam = FEEBeam::new("mwa_full_embedded_element_pattern.h5").unwrap();

    c.bench_function("calc_jones", |b| {
        let az = 45.0_f64.to_radians();
        let za = 80.0_f64.to_radians();
        let freq = 51200000;
        let delays = [0; 16];
        let gains = [1.0; 16];
        let norm_to_zenith = false;
        // Prime the cache.
        let _result = beam.get_modes(freq, &delays, &gains).unwrap();
        b.iter(|| {
            // By calling calc_modes, we skip the cache.
            let _result = beam
                .calc_jones(az, za, freq, &delays, &gains, norm_to_zenith)
                .unwrap();
        })
    });

    c.bench_function("calc_jones_array", |b| {
        let mut az = vec![];
        let mut za = vec![];
        for d in 5..85 {
            let rad = (d as f64).to_radians();
            az.push(rad);
            za.push(rad);
        }
        let freq = 51200000;
        let delays = [0; 16];
        let gains = [1.0; 16];
        let norm_to_zenith = false;
        // Prime the cache.
        let _result = beam.get_modes(freq, &delays, &gains).unwrap();
        b.iter(|| {
            // By calling calc_modes, we skip the cache.
            let _result = beam
                .calc_jones_array(&az, &za, freq, &delays, &gains, norm_to_zenith)
                .unwrap();
        })
    });

    c.bench_function("calculating coefficients", |b| {
        let freq = 51200000;
        let delays = [0; 16];
        let gains = [1.0; 16];
        b.iter(|| {
            // By calling calc_modes, we skip the cache.
            let _result = beam.calc_modes(freq, &delays, &gains).unwrap();
        })
    });

    c.bench_function("getting coefficients from cache", |b| {
        let freq = 51200000;
        let delays = [0; 16];
        let gains = [1.0; 16];
        // Prime the cache.
        let _result = beam.get_modes(freq, &delays, &gains).unwrap();
        b.iter(|| {
            // By calling get_modes, we hit the cache.
            let _result = beam.get_modes(freq, &delays, &gains).unwrap();
        })
    });
}

criterion_group!(benches, fee);
criterion_main!(benches);
