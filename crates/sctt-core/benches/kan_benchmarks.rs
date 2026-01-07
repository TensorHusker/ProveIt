//! Performance benchmarks for Kan operations
//!
//! Measures composition, coercion, and homogeneous composition performance
//! across different input sizes and face system complexities.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sctt_core::{
    kan::{comp, coe, hcomp},
    syntax::{Dim, DimVar, Face},
    value::Value,
};

fn bench_comp_empty_faces(c: &mut Criterion) {
    let ty = Value::VType(0);
    let base = Value::VType(0);

    c.bench_function("comp_empty_faces", |b| {
        b.iter(|| {
            comp(
                black_box(&ty),
                black_box(&base),
                black_box(&[]),
                black_box(Dim::One),
            )
        })
    });
}

fn bench_comp_face_count_scaling(c: &mut Criterion) {
    let ty = Value::VType(0);
    let base = Value::VType(0);

    let mut group = c.benchmark_group("comp_face_scaling");

    for n_faces in [0, 1, 5, 10, 20, 50, 100] {
        let faces: Vec<(Face, Value)> = (0..n_faces)
            .map(|i| (Face::Eq(DimVar(i as u32), true), Value::VType(0)))
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(n_faces), &faces, |b, faces| {
            b.iter(|| {
                comp(
                    black_box(&ty),
                    black_box(&base),
                    black_box(faces),
                    black_box(Dim::One),
                )
            })
        });
    }

    group.finish();
}

fn bench_comp_face_complexity(c: &mut Criterion) {
    let ty = Value::VType(0);
    let base = Value::VType(0);

    let mut group = c.benchmark_group("comp_face_complexity");

    // Simple face: Eq(0, true)
    let simple_face = vec![(Face::Eq(DimVar(0), true), Value::VType(0))];

    group.bench_with_input(
        BenchmarkId::from_parameter("simple"),
        &simple_face,
        |b, faces| {
            b.iter(|| comp(black_box(&ty), black_box(&base), black_box(faces), Dim::One))
        },
    );

    // Compound face: And(Eq(0, true), Eq(1, false))
    let compound_face = vec![(
        Face::And(
            Box::new(Face::Eq(DimVar(0), true)),
            Box::new(Face::Eq(DimVar(1), false)),
        ),
        Value::VType(0),
    )];

    group.bench_with_input(
        BenchmarkId::from_parameter("compound"),
        &compound_face,
        |b, faces| {
            b.iter(|| comp(black_box(&ty), black_box(&base), black_box(faces), Dim::One))
        },
    );

    // Nested And faces
    let nested_face = vec![(
        Face::And(
            Box::new(Face::And(
                Box::new(Face::Eq(DimVar(0), true)),
                Box::new(Face::Eq(DimVar(1), false)),
            )),
            Box::new(Face::Eq(DimVar(2), true)),
        ),
        Value::VType(0),
    )];

    group.bench_with_input(
        BenchmarkId::from_parameter("nested"),
        &nested_face,
        |b, faces| {
            b.iter(|| comp(black_box(&ty), black_box(&base), black_box(faces), Dim::One))
        },
    );

    group.finish();
}

fn bench_coe_dimensions(c: &mut Criterion) {
    let ty_fam = Value::VType(0); // Constant family
    let base = Value::VType(0);

    let mut group = c.benchmark_group("coe_dimensions");

    // Identity coercion (r to r)
    group.bench_function("identity", |b| {
        b.iter(|| {
            coe(
                black_box(&ty_fam),
                black_box(Dim::Zero),
                black_box(Dim::Zero),
                black_box(&base),
            )
        })
    });

    // Zero to One
    group.bench_function("zero_to_one", |b| {
        b.iter(|| {
            coe(
                black_box(&ty_fam),
                black_box(Dim::Zero),
                black_box(Dim::One),
                black_box(&base),
            )
        })
    });

    // One to Zero (reverse)
    group.bench_function("one_to_zero", |b| {
        b.iter(|| {
            coe(
                black_box(&ty_fam),
                black_box(Dim::One),
                black_box(Dim::Zero),
                black_box(&base),
            )
        })
    });

    // Variable dimension
    group.bench_function("var_to_one", |b| {
        b.iter(|| {
            coe(
                black_box(&ty_fam),
                black_box(Dim::Var(DimVar(0))),
                black_box(Dim::One),
                black_box(&base),
            )
        })
    });

    group.finish();
}

fn bench_hcomp_vs_comp(c: &mut Criterion) {
    let ty = Value::VType(0);
    let base = Value::VType(0);
    let faces: Vec<(Face, Value)> = vec![];

    let mut group = c.benchmark_group("hcomp_vs_comp");

    group.bench_function("hcomp", |b| {
        b.iter(|| hcomp(black_box(&ty), black_box(&base), black_box(&faces)))
    });

    group.bench_function("comp_at_one", |b| {
        b.iter(|| comp(black_box(&ty), black_box(&base), black_box(&faces), Dim::One))
    });

    group.finish();
}

fn bench_face_satisfaction_checking(c: &mut Criterion) {
    use sctt_core::kan::face_satisfied;

    let mut group = c.benchmark_group("face_satisfaction");

    let dims = vec![(0, true), (1, false), (2, true)];

    // Simple Eq face
    let simple_face = Face::Eq(DimVar(0), true);
    group.bench_function("simple_eq", |b| {
        b.iter(|| face_satisfied(black_box(&simple_face), black_box(&dims)))
    });

    // And face
    let and_face = Face::And(
        Box::new(Face::Eq(DimVar(0), true)),
        Box::new(Face::Eq(DimVar(1), false)),
    );
    group.bench_function("and_2", |b| {
        b.iter(|| face_satisfied(black_box(&and_face), black_box(&dims)))
    });

    // Nested And
    let nested_and = Face::And(
        Box::new(Face::And(
            Box::new(Face::Eq(DimVar(0), true)),
            Box::new(Face::Eq(DimVar(1), false)),
        )),
        Box::new(Face::Eq(DimVar(2), true)),
    );
    group.bench_function("and_nested", |b| {
        b.iter(|| face_satisfied(black_box(&nested_and), black_box(&dims)))
    });

    // True face (trivial)
    let true_face = Face::True;
    group.bench_function("true_face", |b| {
        b.iter(|| face_satisfied(black_box(&true_face), black_box(&dims)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_comp_empty_faces,
    bench_comp_face_count_scaling,
    bench_comp_face_complexity,
    bench_coe_dimensions,
    bench_hcomp_vs_comp,
    bench_face_satisfaction_checking
);

criterion_main!(benches);
