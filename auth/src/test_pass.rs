#![cfg(test)]

use soroban_sdk::{
    contract, contractimpl, log,
    testutils::MockAuthContract,
    xdr::{
        HashIdPreimage, HashIdPreimageSorobanAuthorization, InvokeContractArgs, Limits, ScBytes, ScMap, ScMapEntry,
        ScVal, SorobanAddressCredentials, SorobanAuthorizationEntry, SorobanAuthorizedFunction,
        SorobanAuthorizedInvocation, SorobanCredentials, WriteXdr,
    },
    Address, Env, IntoVal,
};

use sha2::Digest;

use crate::Auth;

#[contract]
struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn exec(addr: Address) {
        addr.require_auth();
    }
}

mockall::mock! {
    pub F {
        pub fn balance(addr: Address) -> i128;
    }
}

#[contract]
pub struct TestToken;

#[contractimpl]
impl TestToken {
    pub fn balance(addr: Address) -> i128 {
        MockF::balance(addr)
    }
}

#[test]
fn pass() {
    let env = Env::default();

    // Deploy a token, that accounts will hold to gain authority over the auth
    // contract. The token contract pretends that everyone has one token.
    let token_id = env.register_contract(None, TestToken);

    // Deploy the auth contract, that accounts holding the token can control.
    let threshold = 2i128;
    let auth_id = env.register_contract_with_constructor(None, Auth, (&token_id, threshold).into_val(&env));
    log!(&env, "auth_id", auth_id);

    // Deploy a test contract that'll be used to require auth on the auth
    // contract.
    let test_id = env.register_contract(None, TestContract);
    log!(&env, "test_id", test_id);

    // Create two accounts who will hold 1 token each.
    let holder_1 = env.register_contract(None, MockAuthContract);
    log!(&env, "holder_1", holder_1);
    let holder_2 = env.register_contract(None, MockAuthContract);
    log!(&env, "holder_2", holder_2);

    let mock_ctx = MockF::balance_context();
    mock_ctx.expect().returning(|_| 1);

    let root_invocation = SorobanAuthorizedInvocation {
        function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
            contract_address: (&test_id).into(),
            function_name: "exec".try_into().unwrap(),
            args: [(&auth_id).into()].try_into().unwrap(),
        }),
        sub_invocations: [].try_into().unwrap(),
    };
    let signed_payload = sha2::Sha256::digest(
        HashIdPreimage::SorobanAuthorization(HashIdPreimageSorobanAuthorization {
            network_id: soroban_sdk::xdr::Hash(env.ledger().network_id().into()),
            nonce: 0,
            signature_expiration_ledger: 0,
            invocation: root_invocation.clone(),
        })
        .to_xdr(Limits::none())
        .unwrap(),
    );

    let test = TestContractClient::new(&env, &test_id);
    test.set_auths(&[
        SorobanAuthorizationEntry {
            credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                address: (&auth_id).clone().into(),
                nonce: 0,
                signature_expiration_ledger: 0,
                signature: ScVal::Map(Some(ScMap(
                    [
                        ScMapEntry {
                            key: (&holder_1).into(),
                            val: ScVal::Void,
                        },
                        ScMapEntry {
                            key: (&holder_2).into(),
                            val: ScVal::Void,
                        },
                    ]
                    .try_into()
                    .unwrap(),
                ))),
            }),
            root_invocation,
        },
        SorobanAuthorizationEntry {
            credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                address: (&holder_1).clone().into(),
                nonce: 0,
                signature_expiration_ledger: 0,
                signature: ScVal::Void,
            }),
            root_invocation: SorobanAuthorizedInvocation {
                function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                    contract_address: (&auth_id).into(),
                    function_name: "__check_auth".try_into().unwrap(),
                    args: [ScVal::Bytes(ScBytes(signed_payload.as_slice().try_into().unwrap()))]
                        .try_into()
                        .unwrap(),
                }),
                sub_invocations: [].try_into().unwrap(),
            },
        },
        SorobanAuthorizationEntry {
            credentials: SorobanCredentials::Address(SorobanAddressCredentials {
                address: (&holder_2).clone().into(),
                nonce: 0,
                signature_expiration_ledger: 0,
                signature: ScVal::Void,
            }),
            root_invocation: SorobanAuthorizedInvocation {
                function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
                    contract_address: (&auth_id).into(),
                    function_name: "__check_auth".try_into().unwrap(),
                    args: [ScVal::Bytes(ScBytes(signed_payload.as_slice().try_into().unwrap()))]
                        .try_into()
                        .unwrap(),
                }),
                sub_invocations: [].try_into().unwrap(),
            },
        },
    ])
    .exec(&auth_id);
}
