#![allow(unused_parens)]
//! # Clustering and Search in Multi-Dimensional Spaces
//! 
//! This section documents OpenCV's interface to the FLANN library. FLANN (Fast Library for Approximate
//! Nearest Neighbors) is a library that contains a collection of algorithms optimized for fast nearest
//! neighbor search in large datasets and for high dimensional features. More information about FLANN
//! can be found in [Muja2009](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Muja2009) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::IndexParamsTrait, super::KDTreeIndexParamsTrait, super::LinearIndexParamsTrait, super::CompositeIndexParamsTrait, super::AutotunedIndexParamsTrait, super::HierarchicalClusteringIndexParamsTrait, super::KMeansIndexParamsTrait, super::LshIndexParamsTrait, super::SavedIndexParamsTrait, super::SearchParamsTrait, super::IndexTrait };
}

pub const AUTOTUNED: i32 = 255;
pub const BITS_PER_CHAR: i32 = 8;
pub const BLOCKSIZE: u32 = 8192;
pub const CENTERS_GONZALES: i32 = 1;
pub const CENTERS_KMEANSPP: i32 = 2;
pub const CENTERS_RANDOM: i32 = 0;
pub const COMPOSITE: i32 = 3;
pub const CS: i32 = 7;
pub const EUCLIDEAN: i32 = 1;
pub const FLANN_CENTERS_GONZALES: i32 = 1;
pub const FLANN_CENTERS_GROUPWISE: i32 = 3;
pub const FLANN_CENTERS_KMEANSPP: i32 = 2;
pub const FLANN_CENTERS_RANDOM: i32 = 0;
pub const FLANN_CHECKS_AUTOTUNED: i32 = -2;
pub const FLANN_CHECKS_UNLIMITED: i32 = -1;
pub const FLANN_DIST_CHI_SQUARE: i32 = 7;
pub const FLANN_DIST_CS: i32 = 7;
pub const FLANN_DIST_EUCLIDEAN: i32 = 1;
pub const FLANN_DIST_HAMMING: i32 = 9;
pub const FLANN_DIST_HELLINGER: i32 = 6;
pub const FLANN_DIST_HIST_INTERSECT: i32 = 5;
pub const FLANN_DIST_KL: i32 = 8;
pub const FLANN_DIST_KULLBACK_LEIBLER: i32 = 8;
pub const FLANN_DIST_L1: i32 = 2;
pub const FLANN_DIST_L2: i32 = 1;
pub const FLANN_DIST_MANHATTAN: i32 = 2;
pub const FLANN_DIST_MAX: i32 = 4;
pub const FLANN_DIST_MINKOWSKI: i32 = 3;
pub const FLANN_FLOAT32: i32 = 8;
pub const FLANN_FLOAT64: i32 = 9;
pub const FLANN_INDEX_AUTOTUNED: i32 = 255;
pub const FLANN_INDEX_COMPOSITE: i32 = 3;
pub const FLANN_INDEX_HIERARCHICAL: i32 = 5;
pub const FLANN_INDEX_KDTREE: i32 = 1;
pub const FLANN_INDEX_KDTREE_SINGLE: i32 = 4;
pub const FLANN_INDEX_KMEANS: i32 = 2;
pub const FLANN_INDEX_LINEAR: i32 = 0;
pub const FLANN_INDEX_LSH: i32 = 6;
pub const FLANN_INDEX_SAVED: i32 = 254;
pub const FLANN_INDEX_TYPE_16S: i32 = 3;
pub const FLANN_INDEX_TYPE_16U: i32 = 2;
pub const FLANN_INDEX_TYPE_32F: i32 = 5;
pub const FLANN_INDEX_TYPE_32S: i32 = 4;
pub const FLANN_INDEX_TYPE_64F: i32 = 6;
pub const FLANN_INDEX_TYPE_8S: i32 = 1;
pub const FLANN_INDEX_TYPE_8U: i32 = 0;
pub const FLANN_INDEX_TYPE_ALGORITHM: i32 = 9;
pub const FLANN_INDEX_TYPE_BOOL: i32 = 8;
pub const FLANN_INDEX_TYPE_STRING: i32 = 7;
pub const FLANN_INT16: i32 = 1;
pub const FLANN_INT32: i32 = 2;
pub const FLANN_INT64: i32 = 3;
pub const FLANN_INT8: i32 = 0;
pub const FLANN_LOG_ERROR: i32 = 2;
pub const FLANN_LOG_FATAL: i32 = 1;
pub const FLANN_LOG_INFO: i32 = 4;
pub const FLANN_LOG_NONE: i32 = 0;
pub const FLANN_LOG_WARN: i32 = 3;
pub const FLANN_SIGNATURE_: &'static str = "FLANN_INDEX";
pub const FLANN_UINT16: i32 = 5;
pub const FLANN_UINT32: i32 = 6;
pub const FLANN_UINT64: i32 = 7;
pub const FLANN_UINT8: i32 = 4;
pub const FLANN_USE_BOOST: i32 = 0;
pub const FLANN_VERSION_: &'static str = "1.6.10";
pub const HELLINGER: i32 = 6;
pub const HIST_INTERSECT: i32 = 5;
pub const KDTREE: i32 = 1;
pub const KDTREE_SINGLE: i32 = 4;
pub const KL: i32 = 8;
pub const KMEANS: i32 = 2;
pub const KULLBACK_LEIBLER: i32 = 8;
pub const LAST_VALUE_FLANN_INDEX_TYPE: i32 = 9;
pub const LINEAR: i32 = 0;
pub const MANHATTAN: i32 = 2;
pub const MAX_DIST: i32 = 4;
pub const MINKOWSKI: i32 = 3;
pub const SAVED: i32 = 254;
pub const USE_UNORDERED_MAP: i32 = 1;
/// Pooled storage allocator
/// 
/// The following routines allow for the efficient allocation of storage in
/// small chunks from a specified pool.  Rather than allowing each structure
/// to be freed individually, an entire pool of storage is freed at once.
/// This method has two advantages over just using malloc() and free().  First,
/// it is far more efficient for allocating small objects, as there is
/// no overhead for remembering all the information needed to free each
/// object or consolidating fragmented memory.  Second, the decision about
/// how long to keep an object is made at the time of allocation, and there
/// is no need to track down all the objects to free them.
pub const WORDSIZE: u32 = 16;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlannIndexType {
	FLANN_INDEX_TYPE_8U = 0,
	FLANN_INDEX_TYPE_8S = 1,
	FLANN_INDEX_TYPE_16U = 2,
	FLANN_INDEX_TYPE_16S = 3,
	FLANN_INDEX_TYPE_32S = 4,
	FLANN_INDEX_TYPE_32F = 5,
	FLANN_INDEX_TYPE_64F = 6,
	FLANN_INDEX_TYPE_STRING = 7,
	FLANN_INDEX_TYPE_BOOL = 8,
	FLANN_INDEX_TYPE_ALGORITHM = 9,
	// LAST_VALUE_FLANN_INDEX_TYPE = 9 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::FlannIndexType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_algorithm_t {
	FLANN_INDEX_LINEAR = 0,
	FLANN_INDEX_KDTREE = 1,
	FLANN_INDEX_KMEANS = 2,
	FLANN_INDEX_COMPOSITE = 3,
	FLANN_INDEX_KDTREE_SINGLE = 4,
	FLANN_INDEX_HIERARCHICAL = 5,
	FLANN_INDEX_LSH = 6,
	FLANN_INDEX_SAVED = 254,
	FLANN_INDEX_AUTOTUNED = 255,
	// LINEAR = 0 as isize, // duplicate discriminant
	// KDTREE = 1 as isize, // duplicate discriminant
	// KMEANS = 2 as isize, // duplicate discriminant
	// COMPOSITE = 3 as isize, // duplicate discriminant
	// KDTREE_SINGLE = 4 as isize, // duplicate discriminant
	// SAVED = 254 as isize, // duplicate discriminant
	// AUTOTUNED = 255 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_algorithm_t }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_centers_init_t {
	FLANN_CENTERS_RANDOM = 0,
	FLANN_CENTERS_GONZALES = 1,
	FLANN_CENTERS_KMEANSPP = 2,
	FLANN_CENTERS_GROUPWISE = 3,
	// CENTERS_RANDOM = 0 as isize, // duplicate discriminant
	// CENTERS_GONZALES = 1 as isize, // duplicate discriminant
	// CENTERS_KMEANSPP = 2 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_centers_init_t }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_datatype_t {
	FLANN_INT8 = 0,
	FLANN_INT16 = 1,
	FLANN_INT32 = 2,
	FLANN_INT64 = 3,
	FLANN_UINT8 = 4,
	FLANN_UINT16 = 5,
	FLANN_UINT32 = 6,
	FLANN_UINT64 = 7,
	FLANN_FLOAT32 = 8,
	FLANN_FLOAT64 = 9,
}

opencv_type_enum! { crate::flann::flann_datatype_t }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_distance_t {
	FLANN_DIST_EUCLIDEAN = 1,
	// FLANN_DIST_L2 = 1 as isize, // duplicate discriminant
	FLANN_DIST_MANHATTAN = 2,
	// FLANN_DIST_L1 = 2 as isize, // duplicate discriminant
	FLANN_DIST_MINKOWSKI = 3,
	FLANN_DIST_MAX = 4,
	FLANN_DIST_HIST_INTERSECT = 5,
	FLANN_DIST_HELLINGER = 6,
	FLANN_DIST_CHI_SQUARE = 7,
	// FLANN_DIST_CS = 7 as isize, // duplicate discriminant
	FLANN_DIST_KULLBACK_LEIBLER = 8,
	// FLANN_DIST_KL = 8 as isize, // duplicate discriminant
	FLANN_DIST_HAMMING = 9,
	// EUCLIDEAN = 1 as isize, // duplicate discriminant
	// MANHATTAN = 2 as isize, // duplicate discriminant
	// MINKOWSKI = 3 as isize, // duplicate discriminant
	// MAX_DIST = 4 as isize, // duplicate discriminant
	// HIST_INTERSECT = 5 as isize, // duplicate discriminant
	// HELLINGER = 6 as isize, // duplicate discriminant
	// CS = 7 as isize, // duplicate discriminant
	// KL = 8 as isize, // duplicate discriminant
	// KULLBACK_LEIBLER = 8 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_distance_t }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_log_level_t {
	FLANN_LOG_NONE = 0,
	FLANN_LOG_FATAL = 1,
	FLANN_LOG_ERROR = 2,
	FLANN_LOG_WARN = 3,
	FLANN_LOG_INFO = 4,
}

opencv_type_enum! { crate::flann::flann_log_level_t }

/// The id from which we can get a bucket back in an LSH table
pub type bucket_key = u32;
/// What is stored in an LSH bucket
pub type feature_index = u32;
pub fn flann_distance_type() -> Result<crate::flann::flann_distance_t> {
	unsafe { sys::cvflann_flann_distance_type() }.into_result()
}

pub fn set_distance_type(distance_type: crate::flann::flann_distance_t, order: i32) -> Result<()> {
	unsafe { sys::cvflann_set_distance_type_flann_distance_t_int(distance_type, order) }.into_result()
}

pub trait AutotunedIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_AutotunedIndexParams(&self) -> *const c_void;
	fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void;

}

pub struct AutotunedIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { AutotunedIndexParams }

impl Drop for AutotunedIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_AutotunedIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_AutotunedIndexParams_delete(self.as_raw_mut_AutotunedIndexParams()) };
	}
}

impl AutotunedIndexParams {
	#[inline] pub fn as_raw_AutotunedIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for AutotunedIndexParams {}

impl crate::flann::AutotunedIndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_AutotunedIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::IndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AutotunedIndexParams {
	/// ## C++ default parameters
	/// * target_precision: 0.8f
	/// * build_weight: 0.01f
	/// * memory_weight: 0
	/// * sample_fraction: 0.1f
	pub fn new(target_precision: f32, build_weight: f32, memory_weight: f32, sample_fraction: f32) -> Result<crate::flann::AutotunedIndexParams> {
		unsafe { sys::cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(target_precision, build_weight, memory_weight, sample_fraction) }.into_result().map(|r| unsafe { crate::flann::AutotunedIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait CompositeIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_CompositeIndexParams(&self) -> *const c_void;
	fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void;

}

pub struct CompositeIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { CompositeIndexParams }

impl Drop for CompositeIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_CompositeIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_CompositeIndexParams_delete(self.as_raw_mut_CompositeIndexParams()) };
	}
}

impl CompositeIndexParams {
	#[inline] pub fn as_raw_CompositeIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CompositeIndexParams {}

impl crate::flann::CompositeIndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_CompositeIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::IndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CompositeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	pub fn new(trees: i32, branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::CompositeIndexParams> {
		unsafe { sys::cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(trees, branching, iterations, centers_init, cb_index) }.into_result().map(|r| unsafe { crate::flann::CompositeIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait HierarchicalClusteringIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void;
	fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void;

}

pub struct HierarchicalClusteringIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { HierarchicalClusteringIndexParams }

impl Drop for HierarchicalClusteringIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_HierarchicalClusteringIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_HierarchicalClusteringIndexParams_delete(self.as_raw_mut_HierarchicalClusteringIndexParams()) };
	}
}

impl HierarchicalClusteringIndexParams {
	#[inline] pub fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for HierarchicalClusteringIndexParams {}

impl crate::flann::HierarchicalClusteringIndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::IndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HierarchicalClusteringIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * trees: 4
	/// * leaf_size: 100
	pub fn new(branching: i32, centers_init: crate::flann::flann_centers_init_t, trees: i32, leaf_size: i32) -> Result<crate::flann::HierarchicalClusteringIndexParams> {
		unsafe { sys::cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(branching, centers_init, trees, leaf_size) }.into_result().map(|r| unsafe { crate::flann::HierarchicalClusteringIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait IndexTrait {
	fn as_raw_Index(&self) -> *const c_void;
	fn as_raw_mut_Index(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	fn build(&mut self, features: &dyn core::ToInputArray, params: &crate::flann::IndexParams, dist_type: crate::flann::flann_distance_t) -> Result<()> {
		input_array_arg!(features);
		unsafe { sys::cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(self.as_raw_mut_Index(), features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * params: SearchParams()
	fn knn_search(&mut self, query: &dyn core::ToInputArray, indices: &mut dyn core::ToOutputArray, dists: &mut dyn core::ToOutputArray, knn: i32, params: &crate::flann::SearchParams) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		unsafe { sys::cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, params.as_raw_SearchParams()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * params: SearchParams()
	fn radius_search(&mut self, query: &dyn core::ToInputArray, indices: &mut dyn core::ToOutputArray, dists: &mut dyn core::ToOutputArray, radius: f64, max_results: i32, params: &crate::flann::SearchParams) -> Result<i32> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		unsafe { sys::cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), radius, max_results, params.as_raw_SearchParams()) }.into_result()
	}
	
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		unsafe { sys::cv_flann_Index_save_const_const_StringR(self.as_raw_Index(), filename.opencv_as_extern()) }.into_result()
	}
	
	fn load(&mut self, features: &dyn core::ToInputArray, filename: &str) -> Result<bool> {
		input_array_arg!(features);
		extern_container_arg!(filename);
		unsafe { sys::cv_flann_Index_load_const__InputArrayR_const_StringR(self.as_raw_mut_Index(), features.as_raw__InputArray(), filename.opencv_as_extern()) }.into_result()
	}
	
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_flann_Index_release(self.as_raw_mut_Index()) }.into_result()
	}
	
	fn get_distance(&self) -> Result<crate::flann::flann_distance_t> {
		unsafe { sys::cv_flann_Index_getDistance_const(self.as_raw_Index()) }.into_result()
	}
	
	fn get_algorithm(&self) -> Result<crate::flann::flann_algorithm_t> {
		unsafe { sys::cv_flann_Index_getAlgorithm_const(self.as_raw_Index()) }.into_result()
	}
	
}

pub struct Index {
	ptr: *mut c_void
}

opencv_type_boxed! { Index }

impl Drop for Index {
	fn drop(&mut self) {
		extern "C" { fn cv_Index_delete(instance: *mut c_void); }
		unsafe { cv_Index_delete(self.as_raw_mut_Index()) };
	}
}

impl Index {
	#[inline] pub fn as_raw_Index(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_Index(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Index {}

impl crate::flann::IndexTrait for Index {
	#[inline] fn as_raw_Index(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Index(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Index {
	pub fn default() -> Result<crate::flann::Index> {
		unsafe { sys::cv_flann_Index_Index() }.into_result().map(|r| unsafe { crate::flann::Index::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	pub fn new(features: &dyn core::ToInputArray, params: &crate::flann::IndexParams, dist_type: crate::flann::flann_distance_t) -> Result<crate::flann::Index> {
		input_array_arg!(features);
		unsafe { sys::cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type) }.into_result().map(|r| unsafe { crate::flann::Index::opencv_from_extern(r) } )
	}
	
}

pub trait IndexParamsTrait {
	fn as_raw_IndexParams(&self) -> *const c_void;
	fn as_raw_mut_IndexParams(&mut self) -> *mut c_void;

	fn params(&mut self) -> *mut c_void {
		unsafe { sys::cv_flann_IndexParams_getPropParams(self.as_raw_mut_IndexParams()) }.into_result().expect("Infallible function failed: params")
	}
	
	fn set_params(&mut self, val: *mut c_void) -> () {
		unsafe { sys::cv_flann_IndexParams_setPropParams_voidX(self.as_raw_mut_IndexParams(), val) }.into_result().expect("Infallible function failed: set_params")
	}
	
	/// ## C++ default parameters
	/// * default_val: String()
	fn get_string(&self, key: &str, default_val: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(default_val);
		unsafe { sys::cv_flann_IndexParams_getString_const_const_StringR_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val.opencv_as_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * default_val: -1
	fn get_int(&self, key: &str, default_val: i32) -> Result<i32> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_getInt_const_const_StringR_int(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * default_val: -1
	fn get_double(&self, key: &str, default_val: f64) -> Result<f64> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_getDouble_const_const_StringR_double(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val) }.into_result()
	}
	
	fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		unsafe { sys::cv_flann_IndexParams_setString_const_StringR_const_StringR(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value.opencv_as_extern()) }.into_result()
	}
	
	fn set_int(&mut self, key: &str, value: i32) -> Result<()> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_setInt_const_StringR_int(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value) }.into_result()
	}
	
	fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_setDouble_const_StringR_double(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value) }.into_result()
	}
	
	fn set_float(&mut self, key: &str, value: f32) -> Result<()> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_setFloat_const_StringR_float(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value) }.into_result()
	}
	
	fn set_bool(&mut self, key: &str, value: bool) -> Result<()> {
		extern_container_arg!(key);
		unsafe { sys::cv_flann_IndexParams_setBool_const_StringR_bool(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value) }.into_result()
	}
	
	fn set_algorithm(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_flann_IndexParams_setAlgorithm_int(self.as_raw_mut_IndexParams(), value) }.into_result()
	}
	
	fn get_all(&self, names: &mut core::Vector::<String>, types: &mut core::Vector::<crate::flann::FlannIndexType>, str_values: &mut core::Vector::<String>, num_values: &mut core::Vector::<f64>) -> Result<()> {
		unsafe { sys::cv_flann_IndexParams_getAll_const_vector_String_R_vector_FlannIndexType_R_vector_String_R_vector_double_R(self.as_raw_IndexParams(), names.as_raw_mut_VectorOfString(), types.as_raw_mut_VectorOfFlannIndexType(), str_values.as_raw_mut_VectorOfString(), num_values.as_raw_mut_VectorOff64()) }.into_result()
	}
	
}

pub struct IndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { IndexParams }

impl Drop for IndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_IndexParams_delete(instance: *mut c_void); }
		unsafe { cv_IndexParams_delete(self.as_raw_mut_IndexParams()) };
	}
}

impl IndexParams {
	#[inline] pub fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for IndexParams {}

impl crate::flann::IndexParamsTrait for IndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl IndexParams {
	pub fn default() -> Result<crate::flann::IndexParams> {
		unsafe { sys::cv_flann_IndexParams_IndexParams() }.into_result().map(|r| unsafe { crate::flann::IndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait KDTreeIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_KDTreeIndexParams(&self) -> *const c_void;
	fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void;

}

pub struct KDTreeIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { KDTreeIndexParams }

impl Drop for KDTreeIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_KDTreeIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_KDTreeIndexParams_delete(self.as_raw_mut_KDTreeIndexParams()) };
	}
}

impl KDTreeIndexParams {
	#[inline] pub fn as_raw_KDTreeIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for KDTreeIndexParams {}

impl crate::flann::IndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::KDTreeIndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_KDTreeIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KDTreeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	pub fn new(trees: i32) -> Result<crate::flann::KDTreeIndexParams> {
		unsafe { sys::cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(trees) }.into_result().map(|r| unsafe { crate::flann::KDTreeIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait KMeansIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_KMeansIndexParams(&self) -> *const c_void;
	fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void;

}

pub struct KMeansIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { KMeansIndexParams }

impl Drop for KMeansIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_KMeansIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_KMeansIndexParams_delete(self.as_raw_mut_KMeansIndexParams()) };
	}
}

impl KMeansIndexParams {
	#[inline] pub fn as_raw_KMeansIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for KMeansIndexParams {}

impl crate::flann::IndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::KMeansIndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_KMeansIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KMeansIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	pub fn new(branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::KMeansIndexParams> {
		unsafe { sys::cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(branching, iterations, centers_init, cb_index) }.into_result().map(|r| unsafe { crate::flann::KMeansIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait LinearIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_LinearIndexParams(&self) -> *const c_void;
	fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void;

}

pub struct LinearIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LinearIndexParams }

impl Drop for LinearIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LinearIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_LinearIndexParams_delete(self.as_raw_mut_LinearIndexParams()) };
	}
}

impl LinearIndexParams {
	#[inline] pub fn as_raw_LinearIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LinearIndexParams {}

impl crate::flann::IndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::LinearIndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_LinearIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LinearIndexParams {
	pub fn default() -> Result<crate::flann::LinearIndexParams> {
		unsafe { sys::cv_flann_LinearIndexParams_LinearIndexParams() }.into_result().map(|r| unsafe { crate::flann::LinearIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait LshIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_LshIndexParams(&self) -> *const c_void;
	fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void;

}

pub struct LshIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LshIndexParams }

impl Drop for LshIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LshIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_LshIndexParams_delete(self.as_raw_mut_LshIndexParams()) };
	}
}

impl LshIndexParams {
	#[inline] pub fn as_raw_LshIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for LshIndexParams {}

impl crate::flann::IndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::LshIndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_LshIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LshIndexParams {
	pub fn new(table_number: i32, key_size: i32, multi_probe_level: i32) -> Result<crate::flann::LshIndexParams> {
		unsafe { sys::cv_flann_LshIndexParams_LshIndexParams_int_int_int(table_number, key_size, multi_probe_level) }.into_result().map(|r| unsafe { crate::flann::LshIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait SavedIndexParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_SavedIndexParams(&self) -> *const c_void;
	fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void;

}

pub struct SavedIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { SavedIndexParams }

impl Drop for SavedIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_SavedIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_SavedIndexParams_delete(self.as_raw_mut_SavedIndexParams()) };
	}
}

impl SavedIndexParams {
	#[inline] pub fn as_raw_SavedIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SavedIndexParams {}

impl crate::flann::IndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::SavedIndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_SavedIndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SavedIndexParams {
	pub fn new(filename: &str) -> Result<crate::flann::SavedIndexParams> {
		extern_container_arg!(filename);
		unsafe { sys::cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(filename.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::flann::SavedIndexParams::opencv_from_extern(r) } )
	}
	
}

pub trait SearchParamsTrait: crate::flann::IndexParamsTrait {
	fn as_raw_SearchParams(&self) -> *const c_void;
	fn as_raw_mut_SearchParams(&mut self) -> *mut c_void;

}

pub struct SearchParams {
	ptr: *mut c_void
}

opencv_type_boxed! { SearchParams }

impl Drop for SearchParams {
	fn drop(&mut self) {
		extern "C" { fn cv_SearchParams_delete(instance: *mut c_void); }
		unsafe { cv_SearchParams_delete(self.as_raw_mut_SearchParams()) };
	}
}

impl SearchParams {
	#[inline] pub fn as_raw_SearchParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for SearchParams {}

impl crate::flann::IndexParamsTrait for SearchParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::SearchParamsTrait for SearchParams {
	#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SearchParams {
	/// ## C++ default parameters
	/// * checks: 32
	/// * eps: 0
	/// * sorted: true
	pub fn new(checks: i32, eps: f32, sorted: bool) -> Result<crate::flann::SearchParams> {
		unsafe { sys::cv_flann_SearchParams_SearchParams_int_float_bool(checks, eps, sorted) }.into_result().map(|r| unsafe { crate::flann::SearchParams::opencv_from_extern(r) } )
	}
	
}
