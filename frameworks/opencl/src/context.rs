use {cl, core, parenchyma};
use std::hash::{Hash, Hasher};
use super::{OpenCLDevice, OpenCLError, OpenCLMemory, OpenCL, OpenCLQueue};

#[derive(Debug)]
pub struct OpenCLContext {
    id: core::Context,
    /// <sup>*</sup>Multi-platforms contexts are not supported in OpenCL.
    platform_id: core::PlatformId,
    selected_devices: Vec<OpenCLDevice>,
    queue: Option<OpenCLQueue>,
}

impl parenchyma::Context for OpenCLContext {
    /// The framework associated with this context.
    type F = OpenCL;

    /// Constructs a context from a selection of devices.
    ///
    /// Contexts are used by the OpenCL runtime for managing objects such as command-queues,
    /// memory, program and kernel objects and for executing kernels on one or more devices
    /// specified in the context.
    fn new(devices: Vec<OpenCLDevice>) -> Result<Self, OpenCLError> {

        let selected = cl::Device::list_from_core(devices.iter().map(|d| d.id).collect());

        // > Thread safety and destruction for any enclosed pointers are all handled 
        // > automatically. Clone, store, and share between threads to your heart's content.
        let cl_context = cl::builders::ContextBuilder::new().devices(&selected).build().unwrap();

        let id = cl_context.core_as_ref().clone(); // TODO `into_core` method
        let platform_id = *cl_context.platform().unwrap().as_core();

        let context = OpenCLContext {
            id: id,
            platform_id: platform_id,
            selected_devices: devices,
            queue: None,
        };

        Ok(context)
    }

    /// Allocates memory
    fn allocate_memory(&self, _: usize) -> Result<OpenCLMemory, OpenCLError> {

        unimplemented!()
    }
}

impl PartialEq for OpenCLContext {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OpenCLContext { }

impl Hash for OpenCLContext {

    fn hash<H>(&self, state: &mut H) where H: Hasher {
        (unsafe { self.id.as_ptr() as isize }).hash(state);
    }
}