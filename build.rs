// adapted from https://github.com/kornelski/mozjpeg-sys/blob/efc8d99dcebc4d725608f782dc4fdaf7361d2b68/src/build.rs

use std::env;
use std::fs;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

fn compiler(config_dir: &Path, vendor: &Path) -> cc::Build {
    let mut c = cc::Build::new();
    c.include(&config_dir);
    c.include(&vendor);
    c.pic(true);
    c.cpp(true);
    c.warnings(false);

    if let Ok(target_cpu) = env::var("TARGET_CPU") {
        c.flag_if_supported(&format!("-march={}", target_cpu));
    }

    if cfg!(feature = "unwinding") {
        c.flag_if_supported("-fexceptions");
    }

    c
}

fn main() {
    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let root = dunce::canonicalize(root).expect("root dir");
    let vendor = root.join("vendor");
    let config_dir = vendor.join("include");

    // cc crate needs emscripten target to use correct `ar`
    if env::var("TARGET").map_or(false, |t| t == "wasm32-unknown-unknown") {
        println!(
            "cargo:warning=If the build fails, try using wasm32-unknown-emscripten target instead"
        );
        eprintln!("If the build fails, try using wasm32-unknown-emscripten target instead");
    }

    let _ = fs::create_dir_all(&config_dir);

    println!("cargo:include={}", &config_dir.to_str().expect("inc"));
    let mut c = compiler(&config_dir, &vendor);

    let sources = &[
        "vendor/src/body/CollisionBody.cpp",
        "vendor/src/body/RigidBody.cpp",
        "vendor/src/collision/broadphase/DynamicAABBTree.cpp",
        "vendor/src/collision/narrowphase/CollisionDispatch.cpp",
        "vendor/src/collision/narrowphase/GJK/VoronoiSimplex.cpp",
        "vendor/src/collision/narrowphase/GJK/GJKAlgorithm.cpp",
        "vendor/src/collision/narrowphase/SAT/SATAlgorithm.cpp",
        "vendor/src/collision/narrowphase/SphereVsSphereAlgorithm.cpp",
        "vendor/src/collision/narrowphase/CapsuleVsCapsuleAlgorithm.cpp",
        "vendor/src/collision/narrowphase/SphereVsCapsuleAlgorithm.cpp",
        "vendor/src/collision/narrowphase/SphereVsConvexPolyhedronAlgorithm.cpp",
        "vendor/src/collision/narrowphase/CapsuleVsConvexPolyhedronAlgorithm.cpp",
        "vendor/src/collision/narrowphase/ConvexPolyhedronVsConvexPolyhedronAlgorithm.cpp",
        "vendor/src/collision/narrowphase/NarrowPhaseInput.cpp",
        "vendor/src/collision/narrowphase/NarrowPhaseInfoBatch.cpp",
        "vendor/src/collision/narrowphase/SphereVsSphereNarrowPhaseInfoBatch.cpp",
        "vendor/src/collision/narrowphase/CapsuleVsCapsuleNarrowPhaseInfoBatch.cpp",
        "vendor/src/collision/narrowphase/SphereVsCapsuleNarrowPhaseInfoBatch.cpp",
        "vendor/src/collision/shapes/AABB.cpp",
        "vendor/src/collision/shapes/ConvexShape.cpp",
        "vendor/src/collision/shapes/ConvexPolyhedronShape.cpp",
        "vendor/src/collision/shapes/ConcaveShape.cpp",
        "vendor/src/collision/shapes/BoxShape.cpp",
        "vendor/src/collision/shapes/CapsuleShape.cpp",
        "vendor/src/collision/shapes/CollisionShape.cpp",
        "vendor/src/collision/shapes/ConvexMeshShape.cpp",
        "vendor/src/collision/shapes/SphereShape.cpp",
        "vendor/src/collision/shapes/TriangleShape.cpp",
        "vendor/src/collision/shapes/ConcaveMeshShape.cpp",
        "vendor/src/collision/shapes/HeightFieldShape.cpp",
        "vendor/src/collision/RaycastInfo.cpp",
        "vendor/src/collision/Collider.cpp",
        "vendor/src/collision/TriangleVertexArray.cpp",
        "vendor/src/collision/PolygonVertexArray.cpp",
        "vendor/src/collision/TriangleMesh.cpp",
        "vendor/src/collision/PolyhedronMesh.cpp",
        "vendor/src/collision/HalfEdgeStructure.cpp",
        "vendor/src/collision/ContactManifold.cpp",
        "vendor/src/constraint/BallAndSocketJoint.cpp",
        "vendor/src/constraint/ContactPoint.cpp",
        "vendor/src/constraint/FixedJoint.cpp",
        "vendor/src/constraint/HingeJoint.cpp",
        "vendor/src/constraint/Joint.cpp",
        "vendor/src/constraint/SliderJoint.cpp",
        "vendor/src/engine/PhysicsCommon.cpp",
        "vendor/src/systems/ConstraintSolverSystem.cpp",
        "vendor/src/systems/ContactSolverSystem.cpp",
        "vendor/src/systems/DynamicsSystem.cpp",
        "vendor/src/systems/CollisionDetectionSystem.cpp",
        "vendor/src/systems/SolveBallAndSocketJointSystem.cpp",
        "vendor/src/systems/SolveFixedJointSystem.cpp",
        "vendor/src/systems/SolveHingeJointSystem.cpp",
        "vendor/src/systems/SolveSliderJointSystem.cpp",
        "vendor/src/engine/PhysicsWorld.cpp",
        "vendor/src/engine/Island.cpp",
        "vendor/src/engine/Material.cpp",
        "vendor/src/engine/Timer.cpp",
        "vendor/src/engine/OverlappingPairs.cpp",
        "vendor/src/engine/Entity.cpp",
        "vendor/src/engine/EntityManager.cpp",
        "vendor/src/systems/BroadPhaseSystem.cpp",
        "vendor/src/components/Components.cpp",
        "vendor/src/components/CollisionBodyComponents.cpp",
        "vendor/src/components/RigidBodyComponents.cpp",
        "vendor/src/components/TransformComponents.cpp",
        "vendor/src/components/ColliderComponents.cpp",
        "vendor/src/components/JointComponents.cpp",
        "vendor/src/components/BallAndSocketJointComponents.cpp",
        "vendor/src/components/FixedJointComponents.cpp",
        "vendor/src/components/HingeJointComponents.cpp",
        "vendor/src/components/SliderJointComponents.cpp",
        "vendor/src/collision/CollisionCallback.cpp",
        "vendor/src/collision/OverlapCallback.cpp",
        "vendor/src/mathematics/mathematics_functions.cpp",
        "vendor/src/mathematics/Matrix2x2.cpp",
        "vendor/src/mathematics/Matrix3x3.cpp",
        "vendor/src/mathematics/Quaternion.cpp",
        "vendor/src/mathematics/Transform.cpp",
        "vendor/src/mathematics/Vector2.cpp",
        "vendor/src/mathematics/Vector3.cpp",
        "vendor/src/memory/PoolAllocator.cpp",
        "vendor/src/memory/SingleFrameAllocator.cpp",
        "vendor/src/memory/HeapAllocator.cpp",
        "vendor/src/memory/MemoryManager.cpp",
        "vendor/src/utils/Profiler.cpp",
        "vendor/src/utils/DefaultLogger.cpp",
        "vendor/src/utils/DebugRenderer.cpp",
    ];

    for file in sources.iter() {
        assert!(
            Path::new(file).exists(),
            ".cpp file is missing. Maybe you need to run `git submodule update --init`?"
        );
        c.file(file);
    }

    c.compile("reactphysics3d");
}
