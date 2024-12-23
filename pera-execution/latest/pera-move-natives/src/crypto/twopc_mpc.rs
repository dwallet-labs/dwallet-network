use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::loaded_data::runtime_types::Type;
use move_vm_types::natives::function::NativeResult;
use move_vm_types::pop_arg;
use move_vm_types::values::Value;
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use smallvec::smallvec;
use std::collections::VecDeque;

/// Verifies that the user's centralized party signatures are valid.
pub fn verify_partially_signed_signatures(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 4);
    let cost = context.gas_used();
    let dkg_output = pop_arg!(args, Vec<u8>);
    let presigns = pop_arg!(args, Vec<Value>);
    let presigns = presigns
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;
    let messages = pop_arg!(args, Vec<Value>);
    let messages = messages
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;
    let partial_signatures = pop_arg!(args, Vec<Value>);
    let partial_signatures = partial_signatures
        .into_iter()
        .map(|m| m.value_as::<Vec<u8>>())
        .collect::<PartialVMResult<Vec<_>>>()?;
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(
            mock_verify_partial_signature(&partial_signatures, &messages, &presigns, &dkg_output,)
                .is_ok()
        )],
    ))
}

/// TODO (#415): Replace with actual verification function
fn mock_verify_partial_signature(
    partial_signatures: &Vec<Vec<u8>>,
    messages: &Vec<Vec<u8>>,
    presigns: &Vec<Vec<u8>>,
    dkg_output: &[u8],
) -> DwalletMPCResult<()> {
    Ok(())
}
