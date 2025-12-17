// Take a look at the license at the top of the repository in the LICENSE file.

pub use crate::auto::functions::{
    cpuid_supports_arm_neon as supports_arm_neon, cpuid_supports_arm_neon64 as supports_arm_neon64,
    cpuid_supports_x86_3dnow as supports_x86_3dnow, cpuid_supports_x86_avx as supports_x86_avx,
    cpuid_supports_x86_avx2 as supports_x86_avx2, cpuid_supports_x86_mmx as supports_x86_mmx,
    cpuid_supports_x86_mmxext as supports_x86_mmxext, cpuid_supports_x86_sse2 as supports_x86_sse2,
    cpuid_supports_x86_sse3 as supports_x86_sse3, cpuid_supports_x86_sse4_1 as supports_x86_sse4_1,
    cpuid_supports_x86_sse4_2 as supports_x86_sse4_2,
    cpuid_supports_x86_ssse3 as supports_x86_ssse3,
};
