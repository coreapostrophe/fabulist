use inkwell::{
    context::Context,
    module::Module,
    types::{FloatType, IntType, PointerType},
    values::FunctionValue,
    AddressSpace,
};

pub const OUTCOME_KIND_GOTO: u64 = 2;

pub struct RuntimeAbi<'ctx> {
    pub bool_type: IntType<'ctx>,
    pub size_type: IntType<'ctx>,
    pub outcome_kind_type: IntType<'ctx>,
    pub f64_type: FloatType<'ctx>,
    pub value_ptr_type: PointerType<'ctx>,
    pub value_ptr_ptr_type: PointerType<'ctx>,
    pub outcome_ptr_type: PointerType<'ctx>,
    pub value_none: FunctionValue<'ctx>,
    pub value_number: FunctionValue<'ctx>,
    pub value_boolean: FunctionValue<'ctx>,
    pub value_string: FunctionValue<'ctx>,
    pub value_story_ref: FunctionValue<'ctx>,
    pub context_value: FunctionValue<'ctx>,
    pub object_new: FunctionValue<'ctx>,
    pub object_insert: FunctionValue<'ctx>,
    pub env_child: FunctionValue<'ctx>,
    pub env_load: FunctionValue<'ctx>,
    pub env_define: FunctionValue<'ctx>,
    pub env_assign: FunctionValue<'ctx>,
    pub member_get: FunctionValue<'ctx>,
    pub member_assign: FunctionValue<'ctx>,
    pub call: FunctionValue<'ctx>,
    pub closure_new: FunctionValue<'ctx>,
    pub unary_not: FunctionValue<'ctx>,
    pub unary_negate: FunctionValue<'ctx>,
    pub binary_add: FunctionValue<'ctx>,
    pub binary_subtract: FunctionValue<'ctx>,
    pub binary_multiply: FunctionValue<'ctx>,
    pub binary_divide: FunctionValue<'ctx>,
    pub binary_equal: FunctionValue<'ctx>,
    pub binary_not_equal: FunctionValue<'ctx>,
    pub binary_greater: FunctionValue<'ctx>,
    pub binary_greater_equal: FunctionValue<'ctx>,
    pub binary_less: FunctionValue<'ctx>,
    pub binary_less_equal: FunctionValue<'ctx>,
    pub binary_and: FunctionValue<'ctx>,
    pub binary_or: FunctionValue<'ctx>,
    pub is_truthy: FunctionValue<'ctx>,
    pub outcome_continue: FunctionValue<'ctx>,
    pub outcome_return: FunctionValue<'ctx>,
    pub outcome_goto: FunctionValue<'ctx>,
    pub outcome_kind: FunctionValue<'ctx>,
    pub outcome_into_value: FunctionValue<'ctx>,
}

impl<'ctx> RuntimeAbi<'ctx> {
    pub fn declare(context: &'ctx Context, module: &Module<'ctx>) -> Self {
        let void_type = context.void_type();
        let bool_type = context.bool_type();
        let size_type = context.i64_type();
        let outcome_kind_type = context.i64_type();
        let f64_type = context.f64_type();
        let byte_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let value_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let value_ptr_ptr_type = value_ptr_type.ptr_type(AddressSpace::default());
        let outcome_ptr_type = context.i8_type().ptr_type(AddressSpace::default());

        let value_none = module.add_function(
            "fabc_rt_value_none",
            value_ptr_type.fn_type(&[], false),
            None,
        );
        let value_number = module.add_function(
            "fabc_rt_value_number",
            value_ptr_type.fn_type(&[f64_type.into()], false),
            None,
        );
        let value_boolean = module.add_function(
            "fabc_rt_value_boolean",
            value_ptr_type.fn_type(&[bool_type.into()], false),
            None,
        );
        let value_string = module.add_function(
            "fabc_rt_value_string",
            value_ptr_type.fn_type(&[byte_ptr_type.into(), size_type.into()], false),
            None,
        );
        let value_story_ref = module.add_function(
            "fabc_rt_value_story_ref",
            value_ptr_type.fn_type(&[byte_ptr_type.into(), size_type.into()], false),
            None,
        );
        let context_value = module.add_function(
            "fabc_rt_context_value",
            value_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let object_new = module.add_function(
            "fabc_rt_object_new",
            value_ptr_type.fn_type(&[], false),
            None,
        );
        let object_insert = module.add_function(
            "fabc_rt_object_insert",
            void_type.fn_type(
                &[
                    value_ptr_type.into(),
                    byte_ptr_type.into(),
                    size_type.into(),
                    value_ptr_type.into(),
                ],
                false,
            ),
            None,
        );
        let env_child = module.add_function(
            "fabc_rt_env_child",
            value_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let env_load = module.add_function(
            "fabc_rt_env_load",
            value_ptr_type.fn_type(
                &[
                    value_ptr_type.into(),
                    byte_ptr_type.into(),
                    size_type.into(),
                ],
                false,
            ),
            None,
        );
        let env_define = module.add_function(
            "fabc_rt_env_define",
            void_type.fn_type(
                &[
                    value_ptr_type.into(),
                    byte_ptr_type.into(),
                    size_type.into(),
                    value_ptr_type.into(),
                ],
                false,
            ),
            None,
        );
        let env_assign = module.add_function(
            "fabc_rt_env_assign",
            void_type.fn_type(
                &[
                    value_ptr_type.into(),
                    byte_ptr_type.into(),
                    size_type.into(),
                    value_ptr_type.into(),
                ],
                false,
            ),
            None,
        );
        let member_get = module.add_function(
            "fabc_rt_member_get",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let member_assign = module.add_function(
            "fabc_rt_member_assign",
            void_type.fn_type(
                &[
                    value_ptr_type.into(),
                    value_ptr_type.into(),
                    value_ptr_type.into(),
                ],
                false,
            ),
            None,
        );
        let call = module.add_function(
            "fabc_rt_call",
            outcome_ptr_type.fn_type(
                &[
                    value_ptr_type.into(),
                    value_ptr_type.into(),
                    value_ptr_type.into(),
                    value_ptr_ptr_type.into(),
                    size_type.into(),
                ],
                false,
            ),
            None,
        );
        let closure_new = module.add_function(
            "fabc_rt_closure_new",
            value_ptr_type.fn_type(
                &[
                    byte_ptr_type.into(),
                    size_type.into(),
                    value_ptr_type.into(),
                ],
                false,
            ),
            None,
        );
        let unary_not = module.add_function(
            "fabc_rt_unary_not",
            value_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let unary_negate = module.add_function(
            "fabc_rt_unary_negate",
            value_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let binary_add = module.add_function(
            "fabc_rt_binary_add",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_subtract = module.add_function(
            "fabc_rt_binary_subtract",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_multiply = module.add_function(
            "fabc_rt_binary_multiply",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_divide = module.add_function(
            "fabc_rt_binary_divide",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_equal = module.add_function(
            "fabc_rt_binary_equal",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_not_equal = module.add_function(
            "fabc_rt_binary_not_equal",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_greater = module.add_function(
            "fabc_rt_binary_greater",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_greater_equal = module.add_function(
            "fabc_rt_binary_greater_equal",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_less = module.add_function(
            "fabc_rt_binary_less",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_less_equal = module.add_function(
            "fabc_rt_binary_less_equal",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_and = module.add_function(
            "fabc_rt_binary_and",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let binary_or = module.add_function(
            "fabc_rt_binary_or",
            value_ptr_type.fn_type(&[value_ptr_type.into(), value_ptr_type.into()], false),
            None,
        );
        let is_truthy = module.add_function(
            "fabc_rt_is_truthy",
            bool_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let outcome_continue = module.add_function(
            "fabc_rt_outcome_continue",
            outcome_ptr_type.fn_type(&[], false),
            None,
        );
        let outcome_return = module.add_function(
            "fabc_rt_outcome_return",
            outcome_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let outcome_goto = module.add_function(
            "fabc_rt_outcome_goto",
            outcome_ptr_type.fn_type(&[value_ptr_type.into()], false),
            None,
        );
        let outcome_kind = module.add_function(
            "fabc_rt_outcome_kind",
            outcome_kind_type.fn_type(&[outcome_ptr_type.into()], false),
            None,
        );
        let outcome_into_value = module.add_function(
            "fabc_rt_outcome_into_value",
            value_ptr_type.fn_type(&[outcome_ptr_type.into()], false),
            None,
        );

        Self {
            bool_type,
            size_type,
            outcome_kind_type,
            f64_type,
            value_ptr_type,
            value_ptr_ptr_type,
            outcome_ptr_type,
            value_none,
            value_number,
            value_boolean,
            value_string,
            value_story_ref,
            context_value,
            object_new,
            object_insert,
            env_child,
            env_load,
            env_define,
            env_assign,
            member_get,
            member_assign,
            call,
            closure_new,
            unary_not,
            unary_negate,
            binary_add,
            binary_subtract,
            binary_multiply,
            binary_divide,
            binary_equal,
            binary_not_equal,
            binary_greater,
            binary_greater_equal,
            binary_less,
            binary_less_equal,
            binary_and,
            binary_or,
            is_truthy,
            outcome_continue,
            outcome_return,
            outcome_goto,
            outcome_kind,
            outcome_into_value,
        }
    }
}
