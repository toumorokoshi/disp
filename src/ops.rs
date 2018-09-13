use std::sync::Arc;
use std::fmt;

pub type OpList = Vec<Op>;

/*
MOV <s:REG> <t:REG>, copy a value from s to t

 */

#[derive(Clone, Debug)]
pub enum Op {
    ArrayCreate{target: usize, length_source: usize},
    ArraySet{source: usize, target: usize, index_source: usize},
    ArrayLoad{source: usize, target: usize, index_source: usize},
    Assign{target: usize, source: usize},
    BoolNot{source: usize, target: usize},
    BoolLoad{register: usize, constant: bool},
    // if the condition is true, continue down the registry.
    // if the condition is false, jump n instructions to the registry.
    BranchTrue{condition: usize, if_true: usize},
    BranchFalse{condition: usize, if_false: usize},
    CallNative{function: usize, args: Vec<usize>, target: usize},
    FloatAdd{lhs: usize, rhs: usize, target: usize},
    FloatCmp{lhs: usize, rhs: usize, target: usize},
    FloatDiv{lhs: usize, rhs: usize, target: usize},
    FloatLoad{register: usize, constant: f64},
    FloatLessEqual{lhs: usize, rhs: usize, target: usize},
    FloatLessThan{lhs: usize, rhs: usize, target: usize},
    FloatMul{lhs: usize, rhs: usize, target: usize},
    FloatSub{lhs: usize, rhs: usize, target: usize},
    /// Load a Function from the VM's Function Table
    /// into the desired register
    FunctionNativeLoad{func_name: String, target: usize},
    FunctionVMLoad{func_index: usize, target: usize},
    Goto{position: usize},
    IntAdd{lhs: usize, rhs: usize, target: usize},
    IntCmp{lhs: usize, rhs: usize, target: usize},
    IntDiv{lhs: usize, rhs: usize, target: usize},
    IntLessEqual{lhs: usize, rhs: usize, target: usize},
    IntLessThan{lhs: usize, rhs: usize, target: usize},
    IntLoad{register: usize, constant: i64},
    IntMul{lhs: usize, rhs: usize, target: usize},
    IntSub{lhs: usize, rhs: usize, target: usize},
    MapCreate{target: usize},
    Noop{},
    StringLoad{register: usize, constant: Arc<String>},
    Return{register: usize},
}

impl Op {
    pub fn to_string(&self) -> String {
        match self {
            &Op::Assign{target, source} => format!("{0} <= {1}", target, source),
            &Op::ArrayCreate{target, length_source} => format!("{0} <= [{1}]", target, length_source),
            &Op::ArraySet{source, target, index_source} => format!("{1}[{2}] <= {0}", source, target, index_source),
            &Op::ArrayLoad{source, target, index_source} => format!("{{{0}}} <= {{{1}}}[{{{2}}}]", target, source, index_source),
            &Op::BoolNot{source, target} => format!("{1} = !{0}", source, target),
            &Op::BoolLoad{register, constant} => format!("{1} = {0}", register, constant),
            &Op::BranchTrue{condition, if_true} => format!("branch to {1} if {0} is true", condition, if_true),
            &Op::BranchFalse{condition, if_false} => format!("branch to {1} if {0} is false", condition, if_false),
            &Op::CallNative{function, ref args, target} => {
                format!("{1} <= [{0}]({2:?})", function, target, args)
            }
            &Op::Goto{position} => format!("goto {0}", position),
            &Op::FloatAdd{lhs, rhs, target} => format!("{2} <= {0} + {1} (float)", lhs, rhs, target),
            &Op::FloatCmp{lhs, rhs, target} => format!("{2} <= {0} == {1} (float)", lhs, rhs, target),
            &Op::FloatSub{lhs, rhs, target} => format!("{2} <= {0} - {1} (float)", lhs, rhs, target),
            &Op::FloatMul{lhs, rhs, target} => format!("{2} <= {0} + {1} (float)", lhs, rhs, target),
            &Op::FloatDiv{lhs, rhs, target} => format!("{2} <= {0} + {1} (float)", lhs, rhs, target),
            &Op::FloatLoad{register, constant} => format!("{0} <= {1} (float)", register, constant),
            &Op::FloatLessEqual{lhs, rhs, target} => format!("{2} <= {0} <= {1} (float)", lhs, rhs, target),
            &Op::FloatLessThan{lhs, rhs, target} => format!("{2} <= {0} < {1} (float)", lhs, rhs, target),
            &Op::FunctionNativeLoad{ref func_name, target} => format!("{1} <= functions_native[{0}]", func_name, target),
            &Op::FunctionVMLoad{func_index, target} => format!("{1} <= functions_vm[{0}]", func_index, target),
            &Op::IntAdd{lhs, rhs, target} => format!("{2} <= {0} + {1} (Int)", lhs, rhs, target),
            &Op::IntCmp{lhs, rhs, target} => format!("{2} <= {0} == {1} (Int)", lhs, rhs, target),
            &Op::IntSub{lhs, rhs, target} => format!("{2} <= {0} - {1} (Int)", lhs, rhs, target),
            &Op::IntMul{lhs, rhs, target} => format!("{2} <= {0} + {1} (Int)", lhs, rhs, target),
            &Op::IntDiv{lhs, rhs, target} => format!("{2} <= {0} + {1} (Int)", lhs, rhs, target),
            &Op::IntLoad{register, constant} => format!("{0} <= {1} (Int)", register, constant),
            &Op::IntLessEqual{lhs, rhs, target} => format!("{2} <= {0} < {1} (int)", lhs, rhs, target),
            &Op::IntLessThan{lhs, rhs, target} => format!("{2} <= {0} <= {1} (int)", lhs, rhs, target),
            &Op::MapCreate{target} => format!("{0} <= map()", target),
            &Op::Noop{} => format!("noop"),
            &Op::StringLoad{register, ref constant} => format!("{0} <= {1} (String)", register, constant),
            &Op::Return{register} => format!("return {0}", register),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
