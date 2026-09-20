// LLVM uses opaque pointers for everything
pub enum LLVMModule {}
pub enum LLVMContext {}
pub enum LLVMBuilder {}
pub enum LLVMBasicBlock {}
pub enum LLVMType {}
pub enum LLVMValue {}

// These are what you actually pass around
pub type LLVMModuleRef = *mut LLVMModule;
pub type LLVMContextRef = *mut LLVMContext;
pub type LLVMBuilderRef = *mut LLVMBuilder;
pub type LLVMBasicBlockRef = *mut LLVMBasicBlock;
pub type LLVMTypeRef = *mut LLVMType;
pub type LLVMValueRef = *mut LLVMValue;

pub enum LLVMTargetMachine {}
pub enum LLVMTarget {}
// pub enum LLVMTargetData {}

pub type LLVMTargetMachineRef = *mut LLVMTargetMachine;
pub type LLVMTargetRef = *mut LLVMTarget;
pub type LLVMBool = std::ffi::c_int;
// pub type LLVMTargetDataRef = *mut LLVMTargetData;
