pub trait MatchErrorCode<'a> {
    fn match_vulkan_error_code(self) -> &'a str;
}

impl<'a> MatchErrorCode<'a> for i32 {
    fn match_vulkan_error_code(self) -> &'a str {
        match self {
            0 => "Success",
            1 => "Not Ready",
            2 => "Timeout",
            3 => "Event Set",
            4 => "Event Reset",
            5 => "Incomplete",
            -1 => "Out of Host Memory",
            -2 => "Out of Device Memory",
            -3 => "Initialization Failed",
            -4 => "Device Lost",
            -5 => "Memory Map Failed",
            -6 => "Layer Not Present",
            -7 => "Extension Not Present",
            -8 => "Feature Not Present",
            -9 => "Incompatible Driver",
            -10 => "Too Many Objects",
            -11 => "Format Not Supported",
            -12 => "Fragmented Pool",
            // The poor souls that get this one
            -13 => "Unknown",
            -1000069000 => "Out of Pool Memory",
            -1000072003 => "Invalid External Handle",
            -1000161000 => "Fragmentation",
            -1000257000 => "Invalid Opaque Capture Address",
            1000297000 => "Pipeline Compile Required",
            -1000000000 => "Surface Lost KHR",
            -1000000001 => "Native Window In Use KHR",
            1000001003 => "Suboptimal KHR",
            -1000001004 => "Out of Date KHR",
            -1000003001 => "Incompatible Display KHR",
            -1000011001 => "Validation Failed EXT",
            -1000012000 => "Invalid Shader NV",
            -1000158000 => "Invalid Drm Format ModifierPlaneLayoutEXT",
            -1000174001 => "Not PermittedEXT",
            -1000255000 => "Full Screen Exclusive Mode Lost EXT",
            1000268000 => "Thread Idle KHR",
            1000268001 => "Thread Done KHR",
            1000268002 => "Operation Deferred KHR",
            1000268003 => "Operation Deferred KHR",
            _ => "Unknown Error Code",
        }
    }
}