#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;
extern crate pbc_lib;

mod zk_compute;

use pbc_contract_common::context::ContractContext;
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::zk::ZkClosed;
use pbc_contract_common::zk::{SecretVarId, ZkState, ZkStateChange};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[derive(ReadWriteState, ReadWriteRPC, Debug)]
#[repr(C)]
enum SecretVarType {
    #[discriminant(0)]
    Multiply {},
}

#[state]
struct ContractState {
    result: Option<bool>,
}

#[init(zk = true)]
fn initialize(ctx: ContractContext, zk_state: ZkState<SecretVarType>) -> ContractState {
    ContractState { result: None }
}

#[action(shortname = 0x00, zk = true)]
fn multiply(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarType>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    (
        state,
        vec![],
        vec![zk_compute::multiply_start(&SecretVarType::Multiply {})],
    )
}

#[zk_on_compute_complete]
fn open_result(
    context: ContractContext,
    state: ContractState,
    zk_state: ZkState<SecretVarType>,
    output_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    let mut variables_to_open = vec![];
    for variable_id in output_variables {
        let variable = zk_state.get_variable(variable_id).unwrap();
        variables_to_open.push(variable_id);
    }

    (
        state,
        vec![],
        vec![ZkStateChange::OpenVariables {
            variables: variables_to_open,
        }],
    )
}

#[zk_on_variables_opened]
fn guess_variables_opened(
    context: ContractContext,
    mut state: ContractState,
    zk_state: ZkState<SecretVarType>,
    opened_variables: Vec<SecretVarId>,
) -> (ContractState, Vec<EventGroup>, Vec<ZkStateChange>) {
    for variable_id in opened_variables {
        let variable = zk_state.get_variable(variable_id).unwrap();
        let correct = read_variable_boolean(variable);
        state.result = Some(correct);
    }

    (state, vec![], vec![])
}

fn read_variable_boolean(guess_variable: &ZkClosed<SecretVarType>) -> bool {
    *guess_variable.data.as_ref().unwrap().first().unwrap() == 1u8
}
