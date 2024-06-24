#[repr(u64)]
pub enum ModifierVendor
{
    None = 0,
    Intel = 0x01,
    AMD = 0x02,
    Nvidia = 0x03,
    Samsung = 0x04,
    QCom = 0x05,
    Vivante = 0x06,
    BroadCom = 0x07,
    Arm = 0x08,
    AllWinner = 0x09,
}

// private static Func<DRM_FORMAT_MOD_VENDOR, ulong, ulong> fourcc_mod_code = (vendor, val)= ((ulong)vendor << 56) | ((val) & (ulong)0x00ffffffffffffff),

const fn fourcc_mod_code(vendor: ModifierVendor, val: libc::c_ulonglong) -> libc::c_ulonglong {
    ((vendor as libc::c_ulonglong) << 56) | ((val) & 0x00ffffffffffffff)
}
const DRM_FORMAT_RESERVED: libc::c_ulonglong = ((1 as libc::c_ulonglong) << 56) - 1;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u64)]
pub enum FormatModifier {
    DRM_FORMAT_MOD_INVALID = fourcc_mod_code(ModifierVendor::None, DRM_FORMAT_RESERVED),
    DRM_FORMAT_MOD_LINEAR = fourcc_mod_code(ModifierVendor::None, 0),
    I915_FORMAT_MOD_X_TILED= fourcc_mod_code(ModifierVendor::Intel, 1),
    I915_FORMAT_MOD_Y_TILED= fourcc_mod_code(ModifierVendor::Intel, 2),
    I915_FORMAT_MOD_Yf_TILED= fourcc_mod_code(ModifierVendor::Intel, 3),
    I915_FORMAT_MOD_Y_TILED_CCS= fourcc_mod_code(ModifierVendor::Intel, 4),
    I915_FORMAT_MOD_Yf_TILED_CCS= fourcc_mod_code(ModifierVendor::Intel, 5),
    I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS= fourcc_mod_code(ModifierVendor::Intel, 6),
    I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS= fourcc_mod_code(ModifierVendor::Intel, 7),
    DRM_FORMAT_MOD_SAMSUNG_64_32_TILE= fourcc_mod_code(ModifierVendor::Samsung, 1),
    DRM_FORMAT_MOD_SAMSUNG_16_16_TILE= fourcc_mod_code(ModifierVendor::Samsung, 2),
    DRM_FORMAT_MOD_QCOM_COMPRESSED= fourcc_mod_code(ModifierVendor::QCom, 1),
    DRM_FORMAT_MOD_VIVANTE_TILED= fourcc_mod_code(ModifierVendor::Vivante, 1),
    DRM_FORMAT_MOD_VIVANTE_SUPER_TILED= fourcc_mod_code(ModifierVendor::Vivante, 2),
    DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED= fourcc_mod_code(ModifierVendor::Vivante, 3),
    DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED= fourcc_mod_code(ModifierVendor::Vivante, 4),
    DRM_FORMAT_MOD_NVIDIA_TEGRA_TILED= fourcc_mod_code(ModifierVendor::Nvidia, 1),
}