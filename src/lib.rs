pub type Handle = *mut ::std::os::raw::c_void;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RendererApi {
    Vulkan = 0,
    D3D12 = 1,
    Metal = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum QueueType {
    Graphics = 0,
    Compute = 1,
    Transfer = 2,
    Count = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Format {
    Undefined = 0,
    R1Unorm = 1,
    R2Unorm = 2,
    R4Unorm = 3,
    R4G4Unorm = 4,
    G4R4Unorm = 5,
    A8Unorm = 6,
    R8Unorm = 7,
    R8Snorm = 8,
    R8Uint = 9,
    R8Sint = 10,
    R8Srgb = 11,
    B2G3R3Unorm = 12,
    R4G4B4A4Unorm = 13,
    R4G4B4X4Unorm = 14,
    B4G4R4A4Unorm = 15,
    B4G4R4X4Unorm = 16,
    A4R4G4B4Unorm = 17,
    X4R4G4B4Unorm = 18,
    A4B4G4R4Unorm = 19,
    X4B4G4R4Unorm = 20,
    R5G6B5Unorm = 21,
    B5G6R5Unorm = 22,
    R5G5B5A1Unorm = 23,
    B5G5R5A1Unorm = 24,
    A1B5G5R5Unorm = 25,
    A1R5G5B5Unorm = 26,
    B5G5B5X1Unorm = 27,
    B5G5R5X1Unorm = 28,
    X1R5G5B5Unorm = 29,
    X1B5G5R5Unorm = 30,
    B2G3R3A8Unorm = 31,
    R8G8Unorm = 32,
    R8G8Snorm = 33,
    G8R8Unorm = 34,
    G8R8Snorm = 35,
    R8G8Uint = 36,
    R8G8Sint = 37,
    R8G8Srgb = 38,
    R16Unorm = 39,
    R16Snorm = 40,
    R16Uint = 41,
    R16Sint = 42,
    R16Sfloat = 43,
    R16Sbfloat = 44,
    R8G8B8Unorm = 45,
    R8G8B8Snorm = 46,
    R8G8B8Uint = 47,
    R8G8B8Sint = 48,
    R8G8B8Srgb = 49,
    B8G8R8Unorm = 50,
    B8G8R8Snorm = 51,
    B8G8R8Uint = 52,
    B8G8R8Sint = 53,
    B8G8R8Srgb = 54,
    R8G8B8A8Unorm = 55,
    R8G8B8A8Snorm = 56,
    R8G8B8A8Uint = 57,
    R8G8B8A8Sint = 58,
    R8G8B8A8Srgb = 59,
    B8G8R8A8Unorm = 60,
    B8G8R8A8Snorm = 61,
    B8G8R8A8Uint = 62,
    B8G8R8A8Sint = 63,
    B8G8R8A8Srgb = 64,
    R8G8B8X8Unorm = 65,
    B8G8R8X8Unorm = 66,
    R16G16Unorm = 67,
    G16R16Unorm = 68,
    R16G16Snorm = 69,
    G16R16Snorm = 70,
    R16G16Uint = 71,
    R16G16Sint = 72,
    R16G16Sfloat = 73,
    R16G16Sbfloat = 74,
    R32Uint = 75,
    R32Sint = 76,
    R32Sfloat = 77,
    A2R10G10B10Unorm = 78,
    A2R10G10B10Uint = 79,
    A2R10G10B10Snorm = 80,
    A2R10G10B10Sint = 81,
    A2B10G10R10Unorm = 82,
    A2B10G10R10Uint = 83,
    A2B10G10R10Snorm = 84,
    A2B10G10R10Sint = 85,
    R10G10B10A2Unorm = 86,
    R10G10B10A2Uint = 87,
    R10G10B10A2Snorm = 88,
    R10G10B10A2Sint = 89,
    B10G10R10R10A2Unorm = 90,
    B10G10R10R10A2Uint = 91,
    B10G10R10R10A2Snorm = 92,
    B10G10R10R10A2Sint = 93,
    B10G10R11Ufloat = 94,
    E5B5G9R9Ufloat = 95,
    R16G16B16Unorm = 96,
    R16G16B16Snorm = 97,
    R16G16B16Uint = 98,
    R16G16B16Sint = 99,
    R16G16B16Sfloat = 100,
    R16G16B16Sbfloat = 101,
    R16G16B16A16Unorm = 102,
    R16G16B16A16Snorm = 103,
    R16G16B16A16Uint = 104,
    R16G16B16A16Sint = 105,
    R16G16B16A16Sfloat = 106,
    R16G16B16A16Sbfloat = 107,
    R32G32Uint = 108,
    R32G32Sint = 109,
    R32G32Sfloat = 110,
    R32G32B32Uint = 111,
    R32G32B32Sint = 112,
    R32G32B32Sfloat = 113,
    R32G32B32A32Uint = 114,
    R32G32B32A32Sint = 115,
    R32G32B32A32Sfloat = 116,
    R64Uint = 117,
    R64Sint = 118,
    R64Sfloat = 119,
    R64G64Uint = 120,
    R64G64Sint = 121,
    R64G64Sfloat = 122,
    R64G64B64Uint = 123,
    R64G64B64Sint = 124,
    R64G64B64Sfloat = 125,
    R64G64B64A64Uint = 126,
    R64G64B64A64Sint = 127,
    R64G64B64A64Sfloat = 128,
    D16Unorm = 129,
    X8D24Unorm = 130,
    D32Sfloat = 131,
    S8Uint = 132,
    D16UnormS8Uint = 133,
    D24UnormS8Uint = 134,
    D32SfloatS8Uint = 135,
    DXBC1RGBUnorm = 136,
    DXBC1RGBSrgb = 137,
    DXBC1RGBAUnorm = 138,
    DXBC1RGBASrgb = 139,
    DXBC2Unorm = 140,
    DXBC2Srgb = 141,
    DXBC3Unorm = 142,
    DXBC3Srgb = 143,
    DXBC4Unorm = 144,
    DXBC4Snorm = 145,
    DXBC5Unorm = 146,
    DXBC5Snorm = 147,
    DXBC6HUfloat = 148,
    DXBC6HSfloat = 149,
    DXBC7Unorm = 150,
    DXBC7Srgb = 151,
    PVRTC12BPPUnorm = 152,
    PVRTC14BPPUnorm = 153,
    PVRTC22BPPUnorm = 154,
    PVRTC24BPPUnorm = 155,
    PVRTC12BPPSrgb = 156,
    PVRTC14BPPSrgb = 157,
    PVRTC22BPPSrgb = 158,
    PVRTC24BPPSrgb = 159,
    ETC2R8G8B8Unorm = 160,
    ETC2R8G8B8Srgb = 161,
    ETC2R8G8B8A1Unorm = 162,
    ETC2R8G8B8A1Srgb = 163,
    ETC2R8G8B8A8Unorm = 164,
    ETC2R8G8B8A8Srgb = 165,
    Etc2EacR11Unorm = 166,
    Etc2EacR11Snorm = 167,
    Etc2EacR11G11Unorm = 168,
    Etc2EacR11G11Snorm = 169,
    Astc4x4Unorm = 170,
    Astc4x4Srgb = 171,
    Astc5x4Unorm = 172,
    Astc5x4Srgb = 173,
    Astc5x5Unorm = 174,
    Astc5x5Srgb = 175,
    Astc6x5Unorm = 176,
    Astc6x5Srgb = 177,
    Astc6x6Unorm = 178,
    Astc6x6Srgb = 179,
    Astc8x5Unorm = 180,
    Astc8x5Srgb = 181,
    Astc8x6Unorm = 182,
    Astc8x6Srgb = 183,
    Astc8x8Unorm = 184,
    Astc8x8Srgb = 185,
    Astc10x5Unorm = 186,
    Astc10x5Srgb = 187,
    Astc10x6Unorm = 188,
    Astc10x6Srgb = 189,
    Astc10x8Unorm = 190,
    Astc10x8Srgb = 191,
    Astc10x10Unorm = 192,
    Astc10x10Srgb = 193,
    Astc12x10Unorm = 194,
    Astc12x10Srgb = 195,
    Astc12x12Unorm = 196,
    Astc12x12Srgb = 197,
    ClutP4 = 198,
    ClutP4A4 = 199,
    ClutP8 = 200,
    ClutP8A8 = 201,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MemoryUsage {
    GpuOnly = 0,
    CpuOnly = 1,
    CpuToGpu = 2,
    GpuToCpu = 3,
    CpuCopy = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ResourceState {
    Undefined = 1,
    General = 2,
    ColorAttachment = 4,
    DepthStencilWrite = 8,
    DepthStencilReadOnly = 16,
    ShaderReadOnly = 32,
    TransferSrc = 64,
    TransferDst = 128,
    Present = 256,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DescriptorType {
    Undefined = 1,
    VertexBuffer = 2,
    IndexBuffer = 4,
    UniformBuffer = 8,
    Sampler = 16,
    SampledImage = 32,
    StorageImage = 64,
    UniformTexelBuffer = 128,
    StorageTexelBuffer = 256,
    StorageBuffer = 512,
    IndirectBuffer = 1024,
    UniformDynamicBuffer = 2048,
    StorageBufferDynamic = 4096,
    InputAttachment = 8192,
    DepthStencilAttachment = 16384,
    ColorAttachment = 32768,
    TransientAttachment = 65536,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PipelineType {
    Compute = 0,
    Graphics = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IndexType {
    U16 = 0,
    U32 = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FrontFace {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PolygonMode {
    Fill = 0,
    Line = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SamplerAddressMode {
    Repear = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex = 0,
    TesselationControl = 1,
    TesselationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    Compute = 5,
    Count = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Filter {
    Nearest = 0,
    Linear = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    SrcAlphaSaturate = 10,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendOp {
    Add = 0,
    Substract = 1,
    ReverseSubstract = 2,
    Min = 3,
    Max = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowInfo {
    pub title: *const ::std::os::raw::c_char,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub centered: bool,
    pub fullscreen: bool,
    pub grab_mouse: bool,
    pub renderer_api: RendererApi,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Window {
    pub handle: *mut ::std::os::raw::c_void,
}

pub type InitCallback = ::std::option::Option<unsafe extern "C" fn()>;
pub type UpdateCallback = ::std::option::Option<unsafe extern "C" fn(delta_time: f32)>;
pub type ShutdownCallback = ::std::option::Option<unsafe extern "C" fn()>;
pub type ResizeCallback = ::std::option::Option<unsafe extern "C" fn(width: u32, height: u32)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ft_application_config {
    pub argc: u32,
    pub argv: *const *const ::std::os::raw::c_char,
    pub window_info: WindowInfo,
    pub log_level: LogLevel,
    pub on_init: InitCallback,
    pub on_update: UpdateCallback,
    pub on_shutdown: ShutdownCallback,
    pub on_resize: ResizeCallback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WsiInfo {
    _unused: [u8; 0],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyCode {
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Enter = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equal = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    Semicolon = 51,
    Apostrophe = 52,
    GraveAccent = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLock = 83,
    KPDivide = 84,
    KPEnter = 88,
    KP1 = 89,
    KP2 = 90,
    KP3 = 91,
    KP4 = 92,
    KP5 = 93,
    KP6 = 94,
    KP7 = 95,
    KP8 = 96,
    KP9 = 97,
    KP0 = 98,
    Super = 101,
    KPEqual = 103,
    KPAdd = 211,
    KPSubstract = 212,
    KPMultiply = 213,
    KPDecimal = 220,
    LeftControl = 224,
    LeftShift = 225,
    LeftAlt = 226,
    RightControl = 228,
    RightShift = 229,
    RightAlt = 230,
    Menu = 348,
    Count = 350,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Button {
    Left = 1,
    Middle = 2,
    Right = 3,
    Count = 4,
}

pub type Float3 = [f32; 3usize];
pub type Float4 = [f32; 4usize];
pub type Float4x4 = [Float4; 4usize];

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CameraDirection {
    Forward = 0,
    Back = 1,
    Left = 2,
    Right = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraInfo {
    pub position: Float3,
    pub direction: Float3,
    pub up: Float3,
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub position: Float3,
    pub direction: Float3,
    pub up: Float3,
    pub world_up: Float3,
    pub right: Float3,
    pub projection: Float4x4,
    pub view: Float4x4,
    pub speed: f32,
    pub mouse_sensitivity: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CameraController {
    pub camera: *mut Camera,
    pub last_mouse_positon: [i32; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Timer {
    pub start: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RendererBackendInfo {
    pub api: RendererApi,
    pub wsi_info: *mut WsiInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RendererBackend {
    pub api: RendererApi,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceInfo {
    pub backend: *mut RendererBackend,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Device {
    pub api: RendererApi,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandPoolInfo {
    pub queue: *mut Queue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandPool {
    pub queue: *mut Queue,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommandBuffer {
    pub queue: *mut Queue,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QueueInfo {
    pub queue_type: QueueType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Queue {
    pub family_index: u32,
    pub type_: QueueType,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Semaphore {
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fence {
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SamplerInfo {
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: bool,
    pub max_anisotropy: f32,
    pub compare_enable: bool,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sampler {
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: Format,
    pub sample_count: u32,
    pub layer_count: u32,
    pub mip_levels: u32,
    pub descriptor_type: DescriptorType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: Format,
    pub sample_count: u32,
    pub mip_level_count: u32,
    pub layer_count: u32,
    pub descriptor_type: DescriptorType,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BufferInfo {
    pub size: u64,
    pub descriptor_type: DescriptorType,
    pub memory_usage: MemoryUsage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Buffer {
    pub size: u64,
    pub resource_state: ResourceState,
    pub descriptor_type: DescriptorType,
    pub memory_usage: MemoryUsage,
    pub mapped_memory: *mut ::std::os::raw::c_void,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SwapchainInfo {
    pub queue: *mut Queue,
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub vsync: bool,
    pub min_image_count: u32,
    pub wsi_info: *mut WsiInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Swapchain {
    pub min_image_count: u32,
    pub image_count: u32,
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub images: *mut *mut Image,
    pub queue: *mut Queue,
    pub vsync: bool,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QueueSubmitInfo {
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *mut *mut Semaphore,
    pub command_buffer_count: u32,
    pub command_buffers: *mut *mut CommandBuffer,
    pub signal_semaphore_count: u32,
    pub signal_semaphores: *mut *mut Semaphore,
    pub signal_fence: *mut Fence,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QueuePresentInfo {
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *mut *mut Semaphore,
    pub swapchain: *mut Swapchain,
    pub image_index: u32,
}

pub type ColorClearValue = [f32; 4usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DepthStencilClearValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClearValue {
    pub color: ColorClearValue,
    pub depth_stencil: DepthStencilClearValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AttachmentInfo {
    pub image: *const Image,
    pub load_op: AttachmentLoadOp,
    pub clear_value: ClearValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub width: u32,
    pub height: u32,
    pub color_attachment_count: u32,
    pub color_attachments: [AttachmentInfo; 10usize],
    pub depth_attachment: AttachmentInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryBarrier {
    pub allocation: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BufferBarrier {
    pub old_state: ResourceState,
    pub new_state: ResourceState,
    pub buffer: *mut Buffer,
    pub src_queue: *mut Queue,
    pub dst_queue: *mut Queue,
    pub size: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageBarrier {
    pub old_state: ResourceState,
    pub new_state: ResourceState,
    pub image: *const Image,
    pub src_queue: *const Queue,
    pub dst_queue: *const Queue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ShaderModuleInfo {
    pub bytecode_size: u32,
    pub bytecode: *const ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ShaderInfo {
    pub compute: ShaderModuleInfo,
    pub vertex: ShaderModuleInfo,
    pub tessellation_control: ShaderModuleInfo,
    pub tessellation_evaluation: ShaderModuleInfo,
    pub geometry: ShaderModuleInfo,
    pub fragment: ShaderModuleInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Binding {
    pub set: u32,
    pub binding: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub stage: ShaderStage,
}

pub type Bindings = *mut Binding;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BindingMapItem {
    pub name: [::std::os::raw::c_char; 20usize],
    pub value: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ReflectionData {
    pub binding_count: u32,
    pub bindings: Bindings,
    pub binding_map: *mut isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Shader {
    pub reflect_data: ReflectionData,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DescriptorSetLayout {
    pub reflection_data: ReflectionData,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VertexBindingInfo {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VertexAttributeInfo {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VertexLayout {
    pub binding_info_count: u32,
    pub binding_infos: [VertexBindingInfo; 15usize],
    pub attribute_info_count: u32,
    pub attribute_infos: [VertexAttributeInfo; 15usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RasterizerStateInfo {
    pub cull_mode: CullMode,
    pub front_face: FrontFace,
    pub polygon_mode: PolygonMode,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DepthStateInfo {
    pub depth_test: bool,
    pub depth_write: bool,
    pub compare_op: CompareOp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlendAttachmentState {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub src_alpha_factor: BlendFactor,
    pub dst_alpha_factor: BlendFactor,
    pub op: BlendOp,
    pub alpha_op: BlendOp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlendStateInfo {
    pub attachment_states: [BlendAttachmentState; 10usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PipelineInfo {
    pub type_: PipelineType,
    pub vertex_layout: VertexLayout,
    pub rasterizer_info: RasterizerStateInfo,
    pub topology: PrimitiveTopology,
    pub depth_state_info: DepthStateInfo,
    pub shader: *mut Shader,
    pub descriptor_set_layout: *mut DescriptorSetLayout,
    pub sample_count: u32,
    pub color_attachment_count: u32,
    pub color_attachment_formats: [Format; 10usize],
    pub depth_stencil_format: Format,
    pub blend_state_info: BlendStateInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pipeline {
    pub type_: PipelineType,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DescriptorSetInfo {
    pub set: u32,
    pub descriptor_set_layout: *mut DescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DescriptorSet {
    pub layout: *mut DescriptorSetLayout,
    pub handle: Handle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BufferDescriptor {
    pub buffer: *mut Buffer,
    pub offset: u64,
    pub range: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageDescriptor {
    pub image: *mut Image,
    pub resource_state: ResourceState,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SamplerDescriptor {
    pub sampler: *mut Sampler,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DescriptorWrite {
    pub descriptor_count: u32,
    pub descriptor_name: *const ::std::os::raw::c_char,
    pub sampler_descriptors: *mut SamplerDescriptor,
    pub image_descriptors: *mut ImageDescriptor,
    pub buffer_descriptors: *mut BufferDescriptor,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderPass {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderGraph {
    _unused: [u8; 0],
}

pub type GetClearColorCb =
    ::std::option::Option<unsafe extern "C" fn(arg1: u32, arg2: *mut [f32; 4usize]) -> bool>;
pub type GetClearDepthStencilCb =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut DepthStencilClearValue) -> bool>;
pub type CreateCb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const Device, arg2: *mut ::std::os::raw::c_void),
>;

pub type ExecuteCb = ::std::option::Option<
    unsafe extern "C" fn(
        device: *const Device,
        arg1: *mut CommandBuffer,
        arg2: *mut ::std::os::raw::c_void,
    ),
>;

pub type DestroyCb = ::std::option::Option<
    unsafe extern "C" fn(arg1: *const Device, arg2: *mut ::std::os::raw::c_void),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModelTexture {
    pub width: u32,
    pub height: u32,
    pub data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PbrMetallicRoughness {
    pub base_color_texture: ModelTexture,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub metallic_roughness: PbrMetallicRoughness,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AnimationInterpolation {
    Step = 0,
    Linear = 1,
    Spline = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TransformType {
    Translation = 0,
    Rotation = 1,
    Scale = 2,
    Weights = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnimationSampler {
    pub frame_count: u32,
    pub times: *mut f32,
    pub values: *mut f32,
    pub interpolation: AnimationInterpolation,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnimationChannel {
    pub sampler: *mut AnimationSampler,
    pub transform_type: TransformType,
    pub target: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Animation {
    pub duration: f32,
    pub sampler_count: u32,
    pub samplers: *mut AnimationSampler,
    pub channel_count: u32,
    pub channels: *mut AnimationChannel,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertex_count: u32,
    pub positions: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub texcoords: *mut f32,
    pub joints: *mut f32,
    pub weights: *mut f32,
    pub index_count: u32,
    pub is_32bit_indices: bool,
    pub indices: *mut ::std::os::raw::c_void,
    pub world: Float4x4,
    pub material: Material,
    pub has_rotation: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Model {
    pub mesh_count: u32,
    pub meshes: *mut Mesh,
    pub animation_count: u32,
    pub animations: *mut Animation,
}

extern "C" {
    pub fn ft_create_renderer_backend(
        info: *const RendererBackendInfo,
        backend: *mut *mut RendererBackend,
    );
    pub fn ft_destroy_renderer_backend(backend: *mut RendererBackend);
    pub fn ft_create_device(
        backend: *const RendererBackend,
        info: *const DeviceInfo,
        device: *mut *mut Device,
    );
    pub fn ft_destroy_device(device: *mut Device);
    pub fn ft_create_queue(device: *const Device, info: *const QueueInfo, queue: *mut *mut Queue);
    pub fn ft_destroy_queue(queue: *mut Queue);
    pub fn ft_queue_wait_idle(queue: *const Queue);
    pub fn ft_queue_submit(queue: *const Queue, info: *const QueueSubmitInfo);
    pub fn ft_immediate_submit(queue: *const Queue, cmd: *mut CommandBuffer);
    pub fn ft_queue_present(queue: *const Queue, info: *const QueuePresentInfo);
    pub fn ft_create_semaphore(device: *const Device, semaphore: *mut *mut Semaphore);
    pub fn ft_destroy_semaphore(device: *const Device, semaphore: *mut Semaphore);
    pub fn ft_create_fence(device: *const Device, fence: *mut *mut Fence);
    pub fn ft_destroy_fence(device: *const Device, fence: *mut Fence);
    pub fn ft_wait_for_fences(device: *const Device, count: u32, fences: *mut *mut Fence);
    pub fn ft_reset_fences(device: *const Device, count: u32, fences: *mut *mut Fence);
    pub fn ft_create_swapchain(
        device: *const Device,
        info: *const SwapchainInfo,
        swapchain: *mut *mut Swapchain,
    );
    pub fn ft_resize_swapchain(
        device: *const Device,
        swapchain: *mut Swapchain,
        width: u32,
        height: u32,
    );
    pub fn ft_destroy_swapchain(device: *const Device, swapchain: *mut Swapchain);
    pub fn ft_create_command_pool(
        device: *const Device,
        info: *const CommandPoolInfo,
        command_pool: *mut *mut CommandPool,
    );
    pub fn ft_destroy_command_pool(device: *const Device, command_pool: *mut CommandPool);
    pub fn ft_create_command_buffers(
        device: *const Device,
        command_pool: *const CommandPool,
        count: u32,
        command_buffers: *mut *mut CommandBuffer,
    );
    pub fn ft_free_command_buffers(
        device: *const Device,
        command_pool: *const CommandPool,
        count: u32,
        command_buffers: *mut *mut CommandBuffer,
    );
    pub fn ft_destroy_command_buffers(
        device: *const Device,
        command_pool: *const CommandPool,
        count: u32,
        command_buffers: *mut *mut CommandBuffer,
    );
    pub fn ft_begin_command_buffer(cmd: *const CommandBuffer);
    pub fn ft_end_command_buffer(cmd: *const CommandBuffer);
    pub fn ft_acquire_next_image(
        device: *const Device,
        swapchain: *const Swapchain,
        semaphore: *const Semaphore,
        fence: *const Fence,
        image_index: *mut u32,
    );
    pub fn ft_create_shader(device: *const Device, info: *mut ShaderInfo, shader: *mut *mut Shader);
    pub fn ft_destroy_shader(device: *const Device, shader: *mut Shader);
    pub fn ft_create_descriptor_set_layout(
        device: *const Device,
        shader: *mut Shader,
        descriptor_set_layout: *mut *mut DescriptorSetLayout,
    );
    pub fn ft_destroy_descriptor_set_layout(
        device: *const Device,
        layout: *mut DescriptorSetLayout,
    );
    pub fn ft_create_pipeline(
        device: *const Device,
        info: *const PipelineInfo,
        pipeline: *mut *mut Pipeline,
    );
    pub fn ft_destroy_pipeline(device: *const Device, pipeline: *mut Pipeline);
    pub fn ft_cmd_begin_render_pass(cmd: *const CommandBuffer, info: *const RenderPassBeginInfo);
    pub fn ft_cmd_end_render_pass(cmd: *const CommandBuffer);
    pub fn ft_cmd_barrier(
        cmd: *const CommandBuffer,
        memory_barriers_count: u32,
        memory_barrier: *const MemoryBarrier,
        buffer_barriers_count: u32,
        buffer_barriers: *const BufferBarrier,
        image_barriers_count: u32,
        image_barriers: *const ImageBarrier,
    );
    pub fn ft_cmd_set_scissor(cmd: *const CommandBuffer, x: i32, y: i32, width: u32, height: u32);
    pub fn ft_cmd_set_viewport(
        cmd: *const CommandBuffer,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    );
    pub fn ft_cmd_bind_pipeline(cmd: *const CommandBuffer, pipeline: *const Pipeline);
    pub fn ft_cmd_draw(
        cmd: *const CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    pub fn ft_cmd_draw_indexed(
        cmd: *const CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    );
    pub fn ft_cmd_bind_vertex_buffer(cmd: *const CommandBuffer, buffer: *const Buffer, offset: u64);
    pub fn ft_cmd_bind_index_buffer(
        cmd: *const CommandBuffer,
        buffer: *const Buffer,
        offset: u64,
        index_type: IndexType,
    );
    pub fn ft_cmd_copy_buffer(
        cmd: *const CommandBuffer,
        src: *const Buffer,
        src_offset: u64,
        dst: *mut Buffer,
        dst_offset: u64,
        size: u64,
    );
    pub fn ft_cmd_copy_buffer_to_image(
        cmd: *const CommandBuffer,
        src: *const Buffer,
        src_offset: u64,
        dst: *mut Image,
    );
    pub fn ft_cmd_bind_descriptor_set(
        cmd: *const CommandBuffer,
        first_set: u32,
        set: *const DescriptorSet,
        pipeline: *const Pipeline,
    );
    pub fn ft_cmd_dispatch(
        cmd: *const CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    pub fn ft_cmd_push_constants(
        cmd: *const CommandBuffer,
        pipeline: *const Pipeline,
        offset: u32,
        size: u32,
        data: *const ::std::os::raw::c_void,
    );
    pub fn ft_create_buffer(
        device: *const Device,
        info: *const BufferInfo,
        buffer: *mut *mut Buffer,
    );
    pub fn ft_destroy_buffer(device: *const Device, buffer: *mut Buffer);
    pub fn ft_map_memory(device: *const Device, buffer: *mut Buffer)
        -> *mut ::std::os::raw::c_void;
    pub fn ft_unmap_memory(device: *const Device, buffer: *mut Buffer);
    pub fn ft_cmd_draw_indexed_indirect(
        cmd: *const CommandBuffer,
        buffer: *const Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    );
    pub fn ft_create_sampler(
        device: *const Device,
        info: *const SamplerInfo,
        sampler: *mut *mut Sampler,
    );
    pub fn ft_destroy_sampler(device: *const Device, sampler: *mut Sampler);
    pub fn ft_create_image(device: *const Device, info: *const ImageInfo, image: *mut *mut Image);
    pub fn ft_destroy_image(device: *const Device, image: *mut Image);
    pub fn ft_create_descriptor_set(
        device: *const Device,
        info: *const DescriptorSetInfo,
        descriptor_set: *mut *mut DescriptorSet,
    );
    pub fn ft_destroy_descriptor_set(device: *const Device, set: *mut DescriptorSet);
    pub fn ft_update_descriptor_set(
        device: *const Device,
        set: *mut DescriptorSet,
        count: u32,
        writes: *const DescriptorWrite,
    );
    pub fn ft_begin_upload_batch();
    pub fn ft_end_upload_batch();
    pub fn ft_upload_buffer(
        buffer: *mut Buffer,
        offset: u64,
        size: u64,
        data: *const ::std::os::raw::c_void,
    );
    pub fn ft_upload_image(image: *mut Image, size: u64, data: *const ::std::os::raw::c_void);

    pub fn ft_create_window(info: *const WindowInfo) -> Window;
    pub fn ft_destroy_window(window: *mut Window);
    pub fn ft_window_get_size(window: *const Window, width: *mut u32, height: *mut u32);
    pub fn ft_window_get_framebuffer_size(window: *const Window, width: *mut u32, height: *mut u32);
    pub fn ft_window_get_framebuffer_width(window: *const Window) -> u32;
    pub fn ft_window_get_framebuffer_height(window: *const Window) -> u32;
    pub fn ft_window_get_width(window: *const Window) -> u32;
    pub fn ft_window_get_height(window: *const Window) -> u32;
    pub fn ft_window_get_aspect(window: *const Window) -> f32;
    pub fn ft_window_show_cursor(show: bool);
    pub fn ft_app_init(state: *const ft_application_config);
    pub fn ft_app_run();
    pub fn ft_app_shutdown();
    pub fn ft_app_request_exit();
    pub fn ft_get_app_window() -> *const Window;
    pub fn ft_get_time() -> u32;
    pub fn ft_get_delta_time() -> f32;
    pub fn ft_get_wsi_info() -> *mut WsiInfo;
    pub fn ft_get_mouse_pos_x() -> i32;
    pub fn ft_get_mouse_pos_y() -> i32;
    pub fn ft_get_mouse_position(x: *mut i32, y: *mut i32);
    pub fn ft_get_mouse_offset_x() -> i32;
    pub fn ft_get_mouse_offset_y() -> i32;
    pub fn ft_get_mouse_offset(x: *mut i32, y: *mut i32);
    pub fn ft_is_key_pressed(key: KeyCode) -> bool;
    pub fn ft_is_button_pressed(button: Button) -> bool;
    pub fn ft_camera_init(arg1: *mut Camera, info: *const CameraInfo);
    pub fn ft_camera_on_move(arg1: *mut Camera, direction: CameraDirection, delta_time: f32);
    pub fn ft_camera_on_rotate(arg1: *mut Camera, x_offset: f32, y_offset: f32);
    pub fn ft_camera_on_resize(arg1: *mut Camera, width: u32, height: u32);
    pub fn ft_camera_controller_init(arg1: *mut CameraController, arg2: *mut Camera);
    pub fn ft_camera_controller_update(arg1: *mut CameraController, delta_time: f32);
    pub fn ft_camera_controller_reset(arg1: *mut CameraController);
}
