use super::{
    CompilerData, DispError, DispResult, Function, FunctionType, LLVMCompiler, LLVMTypeCache,
    NativeFunction,
};
/// the builder is responsible for building LLVM code.
/// this is a separate layer from the codegen portion as it enables
/// behavior such as:
/// * setting return types dynamically (LLVM builder does not support modifying return types)
/// * forward declaration of function signatures for type checking
/// * reduces scope of unsafe calls to this module.
use libc::c_char;
use llvm_sys::{analysis::*, core::*, execution_engine::*, prelude::*, support::*, target::*, *};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::{mem, ptr};
pub type LLVMFunction = extern "C" fn();

pub struct Builder {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl Builder {
    pub fn new(llvm: &LLVMCompiler) -> Builder {
        Builder {
            context: llvm.context,
            module: llvm.module,
            builder: llvm.builder,
        }
    }

    pub fn build(&mut self, compiler: &CompilerData, types: &mut LLVMTypeCache) {
        let mut functions_to_build = vec![];
        let mut built_functions = HashSet::new();
        for function in compiler.functions.values() {
            if !built_functions.contains(function.name()) {
                match function {
                    FunctionType::Disp(f) => unsafe {
                        let mut args = Vec::with_capacity(f.arg_types.len());
                        for a in &f.arg_types {
                            args.push(types.get(&a));
                        }
                        let return_type = match f.return_type {
                            Some(ref return_type) => types.get(&return_type),
                            None => LLVMVoidType(),
                        };
                        let function_type =
                            LLVMFunctionType(return_type, args.as_mut_ptr(), args.len() as u32, 0);
                        let llvm_function =
                            LLVMAddFunction(self.module, to_ptr(&f.name), function_type);
                        functions_to_build.push((llvm_function, f.clone()));
                    },
                    FunctionType::Native(f) => self.build_native_function(&f, types),
                }
                built_functions.insert(function.name().to_owned());
            }
        }
        for (llvm_function, function) in &functions_to_build {
            self.build_function(*llvm_function, &function);
        }
        // this builds the function in question for now.
        if cfg!(feature = "debug") {
            println!("llvm module:");
            unsafe {
                LLVMDumpModule(self.module);
                let mut debug_output: *mut c_char = mem::zeroed();
                if LLVMVerifyModule(
                    self.module,
                    LLVMVerifierFailureAction::LLVMPrintMessageAction,
                    &mut debug_output,
                ) != 0
                {
                    println!(
                        "llvm module verification failed\n{:?}",
                        CStr::from_ptr(debug_output),
                    );
                }
            }
        }
    }

    pub fn build_native_function(&mut self, function: &NativeFunction, types: &mut LLVMTypeCache) {
        let mut llvm_args = Vec::with_capacity(function.arg_types.len());
        for arg in &function.arg_types {
            llvm_args.push(types.get(&arg));
        }
        unsafe {
            let llvm_function = LLVMGetNamedFunction(self.module, to_ptr(&function.name));
            if llvm_function.is_null() {
                LLVMAddFunction(
                    self.module,
                    to_ptr(&function.name),
                    LLVMFunctionType(
                        types.get(&function.return_type),
                        llvm_args.as_mut_ptr(),
                        llvm_args.len() as u32,
                        0,
                    ),
                );
            }
        }
    }

    pub fn build_function(&mut self, llvm_function: LLVMValueRef, function: &Function) {
        // TODO: modify for iterating through basic blocks.
        unsafe {
            let mut objects = vec![ptr::null_mut(); function.objects];
            let mut basic_blocks = vec![];
            for block in &function.basic_blocks {
                basic_blocks.push(LLVMAppendBasicBlockInContext(
                    self.context,
                    llvm_function,
                    to_ptr(&block.name),
                ));
            }
            for idx in 0..basic_blocks.len() {
                LLVMPositionBuilderAtEnd(self.builder, basic_blocks[idx]);
                for i in &function.basic_blocks[idx].instructions {
                    match i {
                        LLVMInstruction::AddCase {
                            switch,
                            value,
                            block,
                        } => {
                            LLVMAddCase(objects[*switch], objects[*value], basic_blocks[*block]);
                        }
                        LLVMInstruction::AppendBasicBlock { name, target } => {
                            basic_blocks[*target] = LLVMAppendBasicBlockInContext(
                                self.context,
                                llvm_function,
                                to_ptr(&name),
                            );
                        }
                        LLVMInstruction::BuildAlloca { llvm_type, target } => {
                            objects[*target] =
                                LLVMBuildAlloca(self.builder, *llvm_type, to_ptr("alloca"));
                        }
                        LLVMInstruction::BuildBinOp {
                            opcode,
                            lhs,
                            rhs,
                            target,
                        } => {
                            objects[*target] = LLVMBuildBinOp(
                                self.builder,
                                *opcode,
                                objects[*lhs],
                                objects[*rhs],
                                to_ptr("binop"),
                            );
                        }
                        LLVMInstruction::BuildBr { block } => {
                            LLVMBuildBr(self.builder, basic_blocks[*block]);
                        }
                        LLVMInstruction::BuildCondBr {
                            value,
                            true_block,
                            false_block,
                        } => {
                            LLVMBuildCondBr(
                                self.builder,
                                objects[*value],
                                basic_blocks[*true_block],
                                basic_blocks[*false_block],
                            );
                        }
                        LLVMInstruction::BuildGEP {
                            value,
                            indices,
                            target,
                        } => {
                            let mut value_indices = vec![];
                            for i in indices {
                                value_indices.push(objects[*i]);
                            }
                            objects[*target] = LLVMBuildGEP(
                                self.builder,
                                objects[*value],
                                value_indices.as_mut_ptr(),
                                value_indices.len() as u32,
                                to_ptr("gep"),
                            );
                        }
                        LLVMInstruction::BuildLoad { source, target } => {
                            objects[*target] =
                                LLVMBuildLoad(self.builder, objects[*source], to_ptr("load"));
                        }
                        LLVMInstruction::BuildNot { source, target } => {
                            objects[*target] =
                                LLVMBuildNot(self.builder, objects[*source], to_ptr("not"));
                        }
                        LLVMInstruction::BuildRet { source } => {
                            LLVMBuildRet(self.builder, objects[*source]);
                        }
                        LLVMInstruction::BuildRetVoid => {
                            LLVMBuildRetVoid(self.builder);
                        }
                        LLVMInstruction::BuildSwitch {
                            value,
                            post_switch_block,
                            num_cases,
                            target,
                        } => {
                            objects[*target] = LLVMBuildSwitch(
                                self.builder,
                                objects[*value],
                                basic_blocks[*post_switch_block],
                                *num_cases,
                            );
                        }
                        LLVMInstruction::BuildICmp { lhs, rhs, target } => {
                            objects[*target] = LLVMBuildICmp(
                                self.builder,
                                LLVMIntPredicate::LLVMIntEQ,
                                objects[*lhs],
                                objects[*rhs],
                                to_ptr("eqtemp"),
                            );
                        }
                        LLVMInstruction::BuildStore { source, target } => {
                            LLVMBuildStore(self.builder, objects[*source], objects[*target]);
                        }
                        LLVMInstruction::ConstBool { value, target } => {
                            objects[*target] =
                                LLVMConstInt(LLVMInt1Type(), if *value { 1 } else { 0 } as u64, 0);
                        }
                        LLVMInstruction::ConstInt { value, target } => {
                            objects[*target] = LLVMConstInt(
                                // TODO: switch back to 64
                                // once GEP is figured out
                                LLVMInt32TypeInContext(self.context),
                                *value as u64,
                                0,
                            );
                        }
                        LLVMInstruction::ConstI32 { value, target } => {
                            objects[*target] = LLVMConstInt(
                                LLVMInt32TypeInContext(self.context),
                                *value as u64,
                                0,
                            );
                        }
                        LLVMInstruction::BuildCall { name, args, target } => {
                            let function = LLVMGetNamedFunction(self.module, to_ptr(&name));
                            let mut llvm_args = vec![];
                            for a in args {
                                llvm_args.push(objects[*a]);
                            }
                            // in the case where the object type is None,
                            // (i.e. void), LLVM IR should not capture the result.
                            // passing an empty string results in call without a
                            // captured result
                            let result_ptr = match *target != 0 {
                                true => to_ptr("result"),
                                false => to_ptr(""),
                            };
                            objects[*target] = LLVMBuildCall(
                                self.builder,
                                function,
                                llvm_args.as_mut_ptr(),
                                llvm_args.len() as u32,
                                result_ptr,
                            );
                        }
                        LLVMInstruction::BuildGlobalString { value, target } => {
                            objects[*target] = LLVMBuildGlobalStringPtr(
                                self.builder,
                                to_ptr(&value),
                                to_ptr("string"),
                            );
                        }
                        LLVMInstruction::GetParam { arg_num, target } => {
                            objects[*target] = LLVMGetParam(llvm_function, *arg_num);
                        }
                        LLVMInstruction::PositionBuilderAtEnd { block } => {
                            LLVMPositionBuilderAtEnd(self.builder, basic_blocks[*block]);
                        }
                    }
                }
            }
        }
    }

    pub fn get_function(&self, func_name: &str) -> DispResult<LLVMFunction> {
        let f = unsafe {
            let mut ee = mem::uninitialized();
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();
            let mut debug_output: *mut c_char = mem::zeroed();
            if LLVMCreateExecutionEngineForModule(&mut ee, self.module, &mut debug_output) != 0 {
                return Err(DispError::new(&format!(
                    "something went wrong with module initialization\n{:?}",
                    CStr::from_ptr(debug_output),
                )));
            }
            let addr = LLVMGetFunctionAddress(ee, to_ptr(func_name));
            let f: LLVMFunction = mem::transmute(addr);
            f
        };
        Ok(f)
    }
}

/// the llvm instruction to build.
#[derive(Clone, Debug)]
pub enum LLVMInstruction {
    AddCase {
        switch: usize,
        value: usize,
        block: usize,
    },
    AppendBasicBlock {
        name: String,
        target: usize,
    },
    BuildAlloca {
        llvm_type: LLVMTypeRef,
        target: usize,
    },
    BuildBinOp {
        opcode: LLVMOpcode,
        lhs: usize,
        rhs: usize,
        target: usize,
    },
    BuildBr {
        block: usize,
    },
    BuildCall {
        name: String,
        args: Vec<usize>,
        target: usize,
    },
    BuildCondBr {
        value: usize,
        true_block: usize,
        false_block: usize,
    },
    BuildGEP {
        value: usize,
        indices: Vec<usize>,
        target: usize,
    },
    BuildGlobalString {
        value: String,
        target: usize,
    },
    BuildICmp {
        lhs: usize,
        rhs: usize,
        target: usize,
    },
    BuildStore {
        source: usize,
        target: usize,
    },
    BuildSwitch {
        value: usize,
        post_switch_block: usize,
        num_cases: u32,
        target: usize,
    },
    BuildLoad {
        source: usize,
        target: usize,
    },
    BuildNot {
        source: usize,
        target: usize,
    },
    BuildRet {
        source: usize,
    },
    BuildRetVoid,
    ConstBool {
        value: bool,
        target: usize,
    },
    ConstInt {
        value: i64,
        target: usize,
    },
    ConstI32 {
        value: i32,
        target: usize,
    },
    GetParam {
        arg_num: u32,
        target: usize,
    },
    PositionBuilderAtEnd {
        block: usize,
    },
}

impl LLVMInstruction {
    /// returns true if the current instruction is a terminator.
    pub fn is_terminator(&self) -> bool {
        match self {
            LLVMInstruction::BuildRet { source: _ } => true,
            LLVMInstruction::BuildRetVoid {} => true,
            LLVMInstruction::BuildBr { block: _ } => true,
            LLVMInstruction::BuildCondBr {
                value: _,
                true_block: _,
                false_block: _,
            } => true,
            LLVMInstruction::BuildSwitch {
                value: _,
                post_switch_block: _,
                num_cases: _,
                target: _,
            } => true,
            _ => false,
        }
    }
}

/// convert a string into an llvm compatible literal
pub fn to_ptr(s: &str) -> *const c_char {
    let c_string = CString::new(s.clone()).unwrap();
    c_string.into_raw()
}
