#![feature(test)]

#[macro_use] extern crate lazy_static;
extern crate test;
extern crate rand;

extern crate crc;
extern crate twox_hash;
extern crate fnv;
extern crate siphasher;
extern crate seahash;
extern crate farmhash;
extern crate metrohash;
extern crate murmurhash3;
extern crate djb33;
extern crate jhash;
extern crate fxhash;

use test::{ Bencher, black_box };

lazy_static!{
    static ref INPUT16: [u8; 16] = {
        use rand::{ Rng, thread_rng };

        let mut input = [0; 16];
        thread_rng().fill_bytes(&mut input);
        input
    };
    static ref INPUT128: [u8; 128] = {
        use rand::{ Rng, thread_rng };

        let mut input = [0; 128];
        thread_rng().fill_bytes(&mut input);
        input
    };
    static ref INPUT1024: [u8; 1024] = {
        use rand::{ Rng, thread_rng };

        let mut input = [0; 1024];
        thread_rng().fill_bytes(&mut input);
        input
    };
}


macro_rules! bench_hasher {
    ( : $name:ident, ( $( $p:path );* ), $expr:expr ) => {
        mod $name {
            use ::*;
            $(
                use $p;
            )*

            #[bench] fn input_16(b: &mut Bencher) {
                b.bytes = INPUT16.len() as _;
                b.iter(|| ($expr)(&*INPUT16));
            }
            #[bench] fn input_128(b: &mut Bencher) {
                b.bytes = INPUT128.len() as _;
                b.iter(|| ($expr)(&*INPUT128));
            }
            #[bench] fn input_1024(b: &mut Bencher) {
                b.bytes = INPUT1024.len() as _;
                b.iter(|| ($expr)(&*INPUT1024));
            }
        }
    };
    ( $name:ident, ( $( $p:path );* ), $default:expr ) => {
        mod $name {
            use ::*;
            use std::hash::Hasher;
            $(
                use $p;
            )*

            #[bench]
            fn input_16(b: &mut Bencher) {
                b.bytes = INPUT16.len() as _;
                b.iter(|| {
                    let mut hasher = $default;
                    black_box(&mut hasher).write(black_box(&*INPUT16));
                    black_box(hasher).finish();
                });
            }

            #[bench]
            fn input_128(b: &mut Bencher) {
                b.bytes = INPUT128.len() as _;
                b.iter(|| {
                    let mut hasher = $default;
                    black_box(&mut hasher).write(black_box(&*INPUT128));
                    black_box(hasher).finish();
                });
            }

            #[bench]
            fn input_1024(b: &mut Bencher) {
                b.bytes = INPUT1024.len() as _;
                b.iter(|| {
                    let mut hasher = $default;
                    black_box(&mut hasher).write(black_box(&*INPUT1024));
                    black_box(hasher).finish();
                });
            }
        }
    };
}

bench_hasher!(: bench_crc32, (crc::crc32; crc::Hasher32), |input| {
    let mut hasher = crc32::Digest::new(crc32::IEEE);
    black_box(&mut hasher).write(black_box(input));
    black_box(hasher).sum32();
});
bench_hasher!(: bench_crc64, (crc::crc64; crc::Hasher64), |input| {
    let mut hasher = crc64::Digest::new(crc64::ECMA);
    black_box(&mut hasher).write(black_box(input));
    black_box(hasher).sum64();
});

bench_hasher!(bench_xxhash32, (twox_hash::XxHash32), XxHash32::default());
bench_hasher!(bench_xxhash64, (twox_hash::XxHash), XxHash::default());

bench_hasher!(bench_siphasher, (siphasher::sip::SipHasher), SipHasher::new());
bench_hasher!(: bench_siphasher128, (std::hash::Hasher; siphasher::sip128::SipHasher; siphasher::sip128::Hasher128), |input| {
    let mut hasher = SipHasher::new();
    black_box(&mut hasher).write(black_box(input));
    black_box(hasher).finish128();
});

bench_hasher!(bench_farmhash, (farmhash::FarmHasher), FarmHasher::default());
bench_hasher!(: bench_farmhash32, (farmhash::fingerprint32), |input| fingerprint32(black_box(input)));
bench_hasher!(: bench_farmhash64, (farmhash::fingerprint64), |input| fingerprint64(black_box(input)));

bench_hasher!(bench_metrohash64, (metrohash::MetroHash64), MetroHash64::default());
bench_hasher!(bench_metrohash128, (metrohash::MetroHash128), MetroHash128::default());

bench_hasher!(: bench_murmurhash3_32, (murmurhash3::murmurhash3_x86_32), |input| murmurhash3_x86_32(black_box(input), 42));
bench_hasher!(: bench_murmurhash3_128, (murmurhash3::murmurhash3_x64_128), |input| murmurhash3_x64_128(black_box(input), 42));

bench_hasher!(: bench_djb33, (djb33::djb33), |input| djb33(42, black_box(input)));
bench_hasher!(bench_fnv, (fnv::FnvHasher), FnvHasher::default());
bench_hasher!(bench_seahash, (seahash::SeaHasher), SeaHasher::new());
bench_hasher!(bench_jhash, (jhash::JHasher), JHasher::default());
bench_hasher!(bench_fxhash, (fxhash::FxHasher), FxHasher::default());
