use super::{to_ptr, Type};
use llvm_sys::{core::*, execution_engine::*, prelude::*, support::*, target::*, *};
use std::collections::HashMap;

pub struct LLVMTypeCache {
    context: LLVMContextRef,
    cache: HashMap<Type, LLVMTypeRef>,
}

impl LLVMTypeCache {
    pub fn new(context: LLVMContextRef) -> LLVMTypeCache {
        LLVMTypeCache {
            context,
            cache: HashMap::new(),
        }
    }

    pub fn get(&mut self, typ: &Type) -> LLVMTypeRef {
        if let Some(entry) = self.cache.get(typ) {
            return *entry;
        }
        let llvm_type = self.to_llvm_type(typ);
        self.cache.insert(typ.clone(), llvm_type);
        return llvm_type;
    }

    fn to_llvm_type(&self, t: &Type) -> LLVMTypeRef {
        unsafe {
            match t {
                &Type::Array(ref subtype) => self.llvm_declare_array(subtype),
                &Type::Bool => LLVMInt1TypeInContext(self.context),
                &Type::Bytes => LLVMPointerType(self.llvm_declare_array(&Type::Byte), 0),
                &Type::Byte => LLVMInt8TypeInContext(self.context),
                &Type::FunctionPrototype => LLVMVoidTypeInContext(self.context),
                &Type::Int => LLVMInt64TypeInContext(self.context),
                &Type::None => LLVMVoidTypeInContext(self.context),
                &Type::String => LLVMPointerType(LLVMInt8TypeInContext(self.context), 0),
                &Type::Map(ref k, ref v) => LLVMPointerType(LLVMVoidTypeInContext(self.context), 0),
            }
        }
    }

    // Declare the LLVM Array Object Type
    fn llvm_declare_array(&self, base_type: &Type) -> LLVMTypeRef {
        // Our array struct is two values:
        // a pointer of the base type, representing the raw array
        // an i64 representing the type
        unsafe {
            let mut types = [
                LLVMPointerType(self.to_llvm_type(base_type), 0),
                LLVMInt64TypeInContext(self.context),
            ];
            let struct_ref =
                LLVMStructCreateNamed(self.context, to_ptr(&format!("Array<{:?}>", base_type)));
            LLVMStructSetBody(struct_ref, types.as_mut_ptr(), 2, 1);
            struct_ref
        }
    }
}
