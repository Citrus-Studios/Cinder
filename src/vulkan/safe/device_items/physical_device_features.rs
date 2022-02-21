#![allow(non_snake_case)]
use mira::vulkan::VkPhysicalDeviceFeatures;

// 🤢🤮🤮🤮🤮

/// All of the possible physical device features in a struct, best described by the dev comment above
pub struct PhysicalDeviceFeaturesBuilder {
    pub robustBufferAccess: Option<bool>,
    pub fullDrawIndexUint32: Option<bool>,
    pub imageCubeArray: Option<bool>,
    pub independentBlend: Option<bool>,
    pub geometryShader: Option<bool>,
    pub tessellationShader: Option<bool>,
    pub sampleRateShading: Option<bool>,
    pub dualSrcBlend: Option<bool>,
    pub logicOp: Option<bool>,
    pub multiDrawIndirect: Option<bool>,
    pub drawIndirectFirstInstance: Option<bool>,
    pub depthClamp: Option<bool>,
    pub depthBiasClamp: Option<bool>,
    pub fillModeNonSolid: Option<bool>,
    pub depthBounds: Option<bool>,
    pub wideLines: Option<bool>,
    pub largePoints: Option<bool>,
    pub alphaToOne: Option<bool>,
    pub multiViewport: Option<bool>,
    pub samplerAnisotropy: Option<bool>,
    pub textureCompressionETC2: Option<bool>,
    pub textureCompressionASTC_LDR: Option<bool>,
    pub textureCompressionBC: Option<bool>,
    pub occlusionQueryPrecise: Option<bool>,
    pub pipelineStatisticsQuery: Option<bool>,
    pub vertexPipelineStoresAndAtomics: Option<bool>,
    pub fragmentStoresAndAtomics: Option<bool>,
    pub shaderTessellationAndGeometryPointSize: Option<bool>,
    pub shaderImageGatherExtended: Option<bool>,
    pub shaderStorageImageExtendedFormats: Option<bool>,
    pub shaderStorageImageMultisample: Option<bool>,
    pub shaderStorageImageReadWithoutFormat: Option<bool>,
    pub shaderStorageImageWriteWithoutFormat: Option<bool>,
    pub shaderUniformBufferArrayDynamicIndexing: Option<bool>,
    pub shaderSampledImageArrayDynamicIndexing: Option<bool>,
    pub shaderStorageBufferArrayDynamicIndexing: Option<bool>,
    pub shaderStorageImageArrayDynamicIndexing: Option<bool>,
    pub shaderClipDistance: Option<bool>,
    pub shaderCullDistance: Option<bool>,
    pub shaderFloat64: Option<bool>,
    pub shaderInt64: Option<bool>,
    pub shaderInt16: Option<bool>,
    pub shaderResourceResidency: Option<bool>,
    pub shaderResourceMinLod: Option<bool>,
    pub sparseBinding: Option<bool>,
    pub sparseResidencyBuffer: Option<bool>,
    pub sparseResidencyImage2D: Option<bool>,
    pub sparseResidencyImage3D: Option<bool>,
    pub sparseResidency2Samples: Option<bool>,
    pub sparseResidency4Samples: Option<bool>,
    pub sparseResidency8Samples: Option<bool>,
    pub sparseResidency16Samples: Option<bool>,
    pub sparseResidencyAliased: Option<bool>,
    pub variableMultisampleRate: Option<bool>,
    pub inheritedQueries: Option<bool>,
}

impl PhysicalDeviceFeaturesBuilder {
    /// Puts our PhysicalDeviceFeaturesBuilder into a VkPhysicalDeviceFeatures
    pub fn into_raw(&self) -> VkPhysicalDeviceFeatures {
        VkPhysicalDeviceFeatures { 
            robustBufferAccess: match self.robustBufferAccess {
                Some(_) => 1u32,
                None => 0u32,
            },
            fullDrawIndexUint32: match self.fullDrawIndexUint32 {
                Some(_) => 1u32,
                None => 0u32,
            },
            imageCubeArray: match self.imageCubeArray {
                Some(_) => 1u32,
                None => 0u32,
            },
            independentBlend: match self.independentBlend {
                Some(_) => 1u32,
                None => 0u32,
            },
            geometryShader: match self.geometryShader {
                Some(_) => 1u32,
                None => 0u32,
            },
            tessellationShader: match self.tessellationShader {
                Some(_) => 1u32,
                None => 0u32,
            },
            sampleRateShading: match self.sampleRateShading {
                Some(_) => 1u32,
                None => 0u32,
            },
            dualSrcBlend: match self.dualSrcBlend {
                Some(_) => 1u32,
                None => 0u32,
            },
            logicOp: match self.logicOp {
                Some(_) => 1u32,
                None => 0u32,
            },
            multiDrawIndirect: match self.multiDrawIndirect {
                Some(_) => 1u32,
                None => 0u32,
            },
            drawIndirectFirstInstance: match self.drawIndirectFirstInstance {
                Some(_) => 1u32,
                None => 0u32,
            },
            depthClamp: match self.depthClamp {
                Some(_) => 1u32,
                None => 0u32,
            },
            depthBiasClamp: match self.depthBiasClamp {
                Some(_) => 1u32,
                None => 0u32,
            },
            fillModeNonSolid: match self.fillModeNonSolid {
                Some(_) => 1u32,
                None => 0u32,
            },
            depthBounds: match self.depthBounds {
                Some(_) => 1u32,
                None => 0u32,
            },
            wideLines: match self.wideLines {
                Some(_) => 1u32,
                None => 0u32,
            },
            largePoints: match self.largePoints {
                Some(_) => 1u32,
                None => 0u32,
            },
            alphaToOne: match self.alphaToOne {
                Some(_) => 1u32,
                None => 0u32,
            },
            multiViewport: match self.multiViewport {
                Some(_) => 1u32,
                None => 0u32,
            },
            samplerAnisotropy: match self.samplerAnisotropy {
                Some(_) => 1u32,
                None => 0u32,
            },
            textureCompressionETC2: match self.textureCompressionETC2 {
                Some(_) => 1u32,
                None => 0u32,
            },
            textureCompressionASTC_LDR: match self.textureCompressionASTC_LDR {
                Some(_) => 1u32,
                None => 0u32,
            },
            textureCompressionBC: match self.textureCompressionBC {
                Some(_) => 1u32,
                None => 0u32,
            },
            occlusionQueryPrecise: match self.occlusionQueryPrecise {
                Some(_) => 1u32,
                None => 0u32,
            },
            pipelineStatisticsQuery: match self.pipelineStatisticsQuery {
                Some(_) => 1u32,
                None => 0u32,
            },
            vertexPipelineStoresAndAtomics: match self.vertexPipelineStoresAndAtomics {
                Some(_) => 1u32,
                None => 0u32,
            },
            fragmentStoresAndAtomics: match self.fragmentStoresAndAtomics {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderTessellationAndGeometryPointSize: match self.shaderTessellationAndGeometryPointSize {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderImageGatherExtended: match self.shaderImageGatherExtended {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageImageExtendedFormats: match self.shaderStorageImageExtendedFormats {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageImageMultisample: match self.shaderStorageImageMultisample {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageImageReadWithoutFormat: match self.shaderStorageImageReadWithoutFormat {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageImageWriteWithoutFormat: match self.shaderStorageImageWriteWithoutFormat {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderUniformBufferArrayDynamicIndexing: match self.shaderUniformBufferArrayDynamicIndexing {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderSampledImageArrayDynamicIndexing: match self.shaderSampledImageArrayDynamicIndexing {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageBufferArrayDynamicIndexing: match self.shaderStorageBufferArrayDynamicIndexing {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderStorageImageArrayDynamicIndexing: match self.shaderStorageImageArrayDynamicIndexing {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderClipDistance: match self.shaderClipDistance {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderCullDistance: match self.shaderCullDistance {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderFloat64: match self.shaderFloat64 {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderInt64: match self.shaderInt64 {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderInt16: match self.shaderInt16 {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderResourceResidency: match self.shaderResourceResidency {
                Some(_) => 1u32,
                None => 0u32,
            },
            shaderResourceMinLod: match self.shaderResourceMinLod {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseBinding: match self.sparseBinding {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidencyBuffer: match self.sparseResidencyBuffer {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidencyImage2D: match self.sparseResidencyImage2D {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidencyImage3D: match self.sparseResidencyImage3D {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidency2Samples: match self.sparseResidency2Samples {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidency4Samples: match self.sparseResidency4Samples {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidency8Samples: match self.sparseResidency8Samples {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidency16Samples: match self.sparseResidency16Samples {
                Some(_) => 1u32,
                None => 0u32,
            },
            sparseResidencyAliased: match self.sparseResidencyAliased {
                Some(_) => 1u32,
                None => 0u32,
            },
            variableMultisampleRate: match self.variableMultisampleRate {
                Some(_) => 1u32,
                None => 0u32,
            },
            inheritedQueries: match self.inheritedQueries {
                Some(_) => 1u32,
                None => 0u32,
            },
         }
    }
    /// An empty PhysicalDeviceFeaturesBuilder which we can put information into through the functions defined below
    /// 
    /// # Examples
    /// 
    /// ```rs
    /// let physical_device_features = PhysicalDeviceFeaturesBuilder::new()
    ///     .samplerAnisotropy(true)
    ///     .shaderCullDistance(true)
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            robustBufferAccess: None,
            fullDrawIndexUint32: None,
            imageCubeArray: None,
            independentBlend: None,
            geometryShader: None,
            tessellationShader: None,
            sampleRateShading: None,
            dualSrcBlend: None,
            logicOp: None,
            multiDrawIndirect: None,
            drawIndirectFirstInstance: None,
            depthClamp: None,
            depthBiasClamp: None,
            fillModeNonSolid: None,
            depthBounds: None,
            wideLines: None,
            largePoints: None,
            alphaToOne: None,
            multiViewport: None,
            samplerAnisotropy: None,
            textureCompressionETC2: None,
            textureCompressionASTC_LDR: None,
            textureCompressionBC: None,
            occlusionQueryPrecise: None,
            pipelineStatisticsQuery: None,
            vertexPipelineStoresAndAtomics: None,
            fragmentStoresAndAtomics: None,
            shaderTessellationAndGeometryPointSize: None,
            shaderImageGatherExtended: None,
            shaderStorageImageExtendedFormats: None,
            shaderStorageImageMultisample: None,
            shaderStorageImageReadWithoutFormat: None,
            shaderStorageImageWriteWithoutFormat: None,
            shaderUniformBufferArrayDynamicIndexing: None,
            shaderSampledImageArrayDynamicIndexing: None,
            shaderStorageBufferArrayDynamicIndexing: None,
            shaderStorageImageArrayDynamicIndexing: None,
            shaderClipDistance: None,
            shaderCullDistance: None,
            shaderFloat64: None,
            shaderInt64: None,
            shaderInt16: None,
            shaderResourceResidency: None,
            shaderResourceMinLod: None,
            sparseBinding: None,
            sparseResidencyBuffer: None,
            sparseResidencyImage2D: None,
            sparseResidencyImage3D: None,
            sparseResidency2Samples: None,
            sparseResidency4Samples: None,
            sparseResidency8Samples: None,
            sparseResidency16Samples: None,
            sparseResidencyAliased: None,
            variableMultisampleRate: None,
            inheritedQueries: None
        }
    }
    pub fn robustBufferAccess(mut self, robustBufferAccess: bool) -> Self {
        self.robustBufferAccess = Some(robustBufferAccess);
        self
    }
    pub fn fullDrawIndexUint32(mut self, fullDrawIndexUint32: bool) -> Self {
        self.fullDrawIndexUint32 = Some(fullDrawIndexUint32);
        self
    }
    pub fn imageCubeArray(mut self, imageCubeArray: bool) -> Self {
        self.imageCubeArray = Some(imageCubeArray);
        self
    }
    pub fn independentBlend(mut self, independentBlend: bool) -> Self {
        self.independentBlend = Some(independentBlend);
        self
    }
    pub fn geometryShader(mut self, geometryShader: bool) -> Self {
        self.geometryShader = Some(geometryShader);
        self
    }
    pub fn tessellationShader(mut self, tessellationShader: bool) -> Self {
        self.tessellationShader = Some(tessellationShader);
        self
    }
    pub fn sampleRateShading(mut self, sampleRateShading: bool) -> Self {
        self.sampleRateShading = Some(sampleRateShading);
        self
    }
    pub fn dualSrcBlend(mut self, dualSrcBlend: bool) -> Self {
        self.dualSrcBlend = Some(dualSrcBlend);
        self
    }
    pub fn logicOp(mut self, logicOp: bool) -> Self {
        self.logicOp = Some(logicOp);
        self
    }
    pub fn multiDrawIndirect(mut self, multiDrawIndirect: bool) -> Self {
        self.multiDrawIndirect = Some(multiDrawIndirect);
        self
    }
    pub fn drawIndirectFirstInstance(mut self, drawIndirectFirstInstance: bool) -> Self {
        self.drawIndirectFirstInstance = Some(drawIndirectFirstInstance);
        self
    }
    pub fn depthClamp(mut self, depthClamp: bool) -> Self {
        self.depthClamp = Some(depthClamp);
        self
    }
    pub fn depthBiasClamp(mut self, depthBiasClamp: bool) -> Self {
        self.depthBiasClamp = Some(depthBiasClamp);
        self
    }
    pub fn fillModeNonSolid(mut self, fillModeNonSolid: bool) -> Self {
        self.fillModeNonSolid = Some(fillModeNonSolid);
        self
    }
    pub fn depthBounds(mut self, depthBounds: bool) -> Self {
        self.depthBounds = Some(depthBounds);
        self
    }
    pub fn wideLines(mut self, wideLines: bool) -> Self {
        self.wideLines = Some(wideLines);
        self
    }
    pub fn largePoints(mut self, largePoints: bool) -> Self {
        self.largePoints = Some(largePoints);
        self
    }
    pub fn alphaToOne(mut self, alphaToOne: bool) -> Self {
        self.alphaToOne = Some(alphaToOne);
        self
    }
    pub fn multiViewport(mut self, multiViewport: bool) -> Self {
        self.multiViewport = Some(multiViewport);
        self
    }
    pub fn samplerAnisotropy(mut self, samplerAnisotropy: bool) -> Self {
        self.samplerAnisotropy = Some(samplerAnisotropy);
        self
    }
    pub fn textureCompressionETC2(mut self, textureCompressionETC2: bool) -> Self {
        self.textureCompressionETC2 = Some(textureCompressionETC2);
        self
    }
    pub fn textureCompressionASTC_LDR(mut self, textureCompressionASTC_LDR: bool) -> Self {
        self.textureCompressionASTC_LDR = Some(textureCompressionASTC_LDR);
        self
    }
    pub fn textureCompressionBC(mut self, textureCompressionBC: bool) -> Self {
        self.textureCompressionBC = Some(textureCompressionBC);
        self
    }
    pub fn occlusionQueryPrecise(mut self, occlusionQueryPrecise: bool) -> Self {
        self.occlusionQueryPrecise = Some(occlusionQueryPrecise);
        self
    }
    pub fn pipelineStatisticsQuery(mut self, pipelineStatisticsQuery: bool) -> Self {
        self.pipelineStatisticsQuery = Some(pipelineStatisticsQuery);
        self
    }
    pub fn vertexPipelineStoresAndAtomics(mut self, vertexPipelineStoresAndAtomics: bool) -> Self {
        self.vertexPipelineStoresAndAtomics = Some(vertexPipelineStoresAndAtomics);
        self
    }
    pub fn fragmentStoresAndAtomics(mut self, fragmentStoresAndAtomics: bool) -> Self {
        self.fragmentStoresAndAtomics = Some(fragmentStoresAndAtomics);
        self
    }
    pub fn shaderTessellationAndGeometryPointSize(mut self, shaderTessellationAndGeometryPointSize: bool) -> Self {
        self.shaderTessellationAndGeometryPointSize = Some(shaderTessellationAndGeometryPointSize);
        self
    }
    pub fn shaderImageGatherExtended(mut self, shaderImageGatherExtended: bool) -> Self {
        self.shaderImageGatherExtended = Some(shaderImageGatherExtended);
        self
    }
    pub fn shaderStorageImageExtendedFormats(mut self, shaderStorageImageExtendedFormats: bool) -> Self {
        self.shaderStorageImageExtendedFormats = Some(shaderStorageImageExtendedFormats);
        self
    }
    pub fn shaderStorageImageMultisample(mut self, shaderStorageImageMultisample: bool) -> Self {
        self.shaderStorageImageMultisample = Some(shaderStorageImageMultisample);
        self
    }
    pub fn shaderStorageImageReadWithoutFormat(mut self, shaderStorageImageReadWithoutFormat: bool) -> Self {
        self.shaderStorageImageReadWithoutFormat = Some(shaderStorageImageReadWithoutFormat);
        self
    }
    pub fn shaderStorageImageWriteWithoutFormat(mut self, shaderStorageImageWriteWithoutFormat: bool) -> Self {
        self.shaderStorageImageWriteWithoutFormat = Some(shaderStorageImageWriteWithoutFormat);
        self
    }
    pub fn shaderUniformBufferArrayDynamicIndexing(mut self, shaderUniformBufferArrayDynamicIndexing: bool) -> Self {
        self.shaderUniformBufferArrayDynamicIndexing = Some(shaderUniformBufferArrayDynamicIndexing);
        self
    }
    pub fn shaderSampledImageArrayDynamicIndexing(mut self, shaderSampledImageArrayDynamicIndexing: bool) -> Self {
        self.shaderSampledImageArrayDynamicIndexing = Some(shaderSampledImageArrayDynamicIndexing);
        self
    }
    pub fn shaderStorageBufferArrayDynamicIndexing(mut self, shaderStorageBufferArrayDynamicIndexing: bool) -> Self {
        self.shaderStorageBufferArrayDynamicIndexing = Some(shaderStorageBufferArrayDynamicIndexing);
        self
    }
    pub fn shaderStorageImageArrayDynamicIndexing(mut self, shaderStorageImageArrayDynamicIndexing: bool) -> Self {
        self.shaderStorageImageArrayDynamicIndexing = Some(shaderStorageImageArrayDynamicIndexing);
        self
    }
    pub fn shaderClipDistance(mut self, shaderClipDistance: bool) -> Self {
        self.shaderClipDistance = Some(shaderClipDistance);
        self
    }
    pub fn shaderCullDistance(mut self, shaderCullDistance: bool) -> Self {
        self.shaderCullDistance = Some(shaderCullDistance);
        self
    }
    pub fn shaderFloat64(mut self, shaderFloat64: bool) -> Self {
        self.shaderFloat64 = Some(shaderFloat64);
        self
    }
    pub fn shaderInt64(mut self, shaderInt64: bool) -> Self {
        self.shaderInt64 = Some(shaderInt64);
        self
    }
    pub fn shaderInt16(mut self, shaderInt16: bool) -> Self {
        self.shaderInt16 = Some(shaderInt16);
        self
    }
    pub fn shaderResourceResidency(mut self, shaderResourceResidency: bool) -> Self {
        self.shaderResourceResidency = Some(shaderResourceResidency);
        self
    }
    pub fn shaderResourceMinLod(mut self, shaderResourceMinLod: bool) -> Self {
        self.shaderResourceMinLod = Some(shaderResourceMinLod);
        self
    }
    pub fn sparseBinding(mut self, sparseBinding: bool) -> Self {
        self.sparseBinding = Some(sparseBinding);
        self
    }
    pub fn sparseResidencyBuffer(mut self, sparseResidencyBuffer: bool) -> Self {
        self.sparseResidencyBuffer = Some(sparseResidencyBuffer);
        self
    }
    pub fn sparseResidencyImage2D(mut self, sparseResidencyImage2D: bool) -> Self {
        self.sparseResidencyImage2D = Some(sparseResidencyImage2D);
        self
    }
    pub fn sparseResidencyImage3D(mut self, sparseResidencyImage3D: bool) -> Self {
        self.sparseResidencyImage3D = Some(sparseResidencyImage3D);
        self
    }
    pub fn sparseResidency2Samples(mut self, sparseResidency2Samples: bool) -> Self {
        self.sparseResidency2Samples = Some(sparseResidency2Samples);
        self
    }
    pub fn sparseResidency4Samples(mut self, sparseResidency4Samples: bool) -> Self {
        self.sparseResidency4Samples = Some(sparseResidency4Samples);
        self
    }
    pub fn sparseResidency8Samples(mut self, sparseResidency8Samples: bool) -> Self {
        self.sparseResidency8Samples = Some(sparseResidency8Samples);
        self
    }
    pub fn sparseResidency16Samples(mut self, sparseResidency16Samples: bool) -> Self {
        self.sparseResidency16Samples = Some(sparseResidency16Samples);
        self
    }
    pub fn sparseResidencyAliased(mut self, sparseResidencyAliased: bool) -> Self {
        self.sparseResidencyAliased = Some(sparseResidencyAliased);
        self
    }
    pub fn variableMultisampleRate(mut self, variableMultisampleRate: bool) -> Self {
        self.variableMultisampleRate = Some(variableMultisampleRate);
        self
    }
    pub fn inheritedQueries(mut self, inheritedQueries: bool) -> Self {
        self.inheritedQueries = Some(inheritedQueries);
        self
    }
    pub fn build(self) -> VkPhysicalDeviceFeatures {
        return self.into_raw();
    }
}