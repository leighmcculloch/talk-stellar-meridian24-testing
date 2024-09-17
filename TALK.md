---
title: Definitive Guide for Testing Smart Contracts
tags: Talk
---

# Definitive Guide for Testing Smart Contracts

[hackmd.io/@leighmcculloch/m24](https://hackmd.io/@leighmcculloch/m24)

---

## Who am I?

![](https://hackmd.io/_uploads/H1_DtRF9C.jpg)

Leigh McCulloch

Software Engineer
Rust • Go • Wasm

---

# Definitive Guide for Testing Smart Contracts

---

## why?

<!--
why testing
best practice
build confidence in the programs we develop
failure has a cost
prevent cost
give others confidence
confidence in changes not just existing capabilities
confidence at new protocol releases
confidence when dependencies upgrade
-->

---

## soroban

<!--
a consistent theme with Soroban is that we have tried to learn from other blockchains:
 - what hasn't worked for them
 - what has worked
 - what's the best way to build great tooling
 - what's the most effective way to build reliable and safe contracts
 - what gives developers confidence

And a huge area of that was... testing
-->

---

## testing

<!--
rust
byo rust tooling
leverage an entire ecosystem of test capabilities
and so with that we're going to walk through what testing looks like with Stellar contracts
-->

---

unit testing 👀
 - write in rust
 - create an env, register your contract, call it with the client

mocks 👀
 - can mock in unit tests
 - if your contract calls another contract, write a mini replacement with the same interface and register it
 - or use mockall or another mocking fwk (note that only static fns mocking is supported, so there's some constraints, but watch this space)

integration testing 👀
 - if a dep is too complex to mock, or you just don't love mocks (hand up)
 - test against the actual dependency
 - stellar contract fetch to download the .wasm file
 - import the .wasm file
 - register the .wasm file in tests, and the actual code will be called

snapshot testing (real mainnet data)
 - need data, not just the code of a dep?
 - stellar snapshot create --ledger --address Cdep...
 - import the snapshot
 - write tests against the actual contract with some actual real data
 - when to use this?
   - complex deps that you want to understand better
   - complex deps that you want to include in your tests or fuzzes for more realistic expectations and outcomes
   - limited to point-in-time snapshots, have to update snashots manually if you want newer data on some cadence
   - good throw away tool for debugging, if you have a bug that you don't understand, take a snapshot, and test against the state
     - then use that test to confirm the bug is fixed

fuzz testing 👀
 - identify unexpected failures
   - failures always rollback, unless you try invoke another contract then its failures won't necessarily depending on what your contract does with the error returned
   - more about identifying failures that might mean your contract is pegged in some state
 - identify guarantees that aren't always true, and might not be true in cases not considered
 - use cargo-fuzz
 - inputs to fuzz tests are randomly generated
 - all sdk types can be inputs to fuzz tests

end-to-end testing
 - run quickstart in ci

state machine testing 👀

differential testing / fuzzing 👀
 - contracts change over time
 - goal is to identify changes
 - write your tests so they can be run over multiple versions of your contract
 - run tests on past deployed wasms
 - compare results against native build

invariant testing 👀
 - forces you to say what is always true, what guarantees does your contract make
 - debug_assert in code
 - run a function at the end of every test to check that the contract state fits some rules

formal verification 👀
 - whole new level of invariant testing
 - come to chandra's talk later today

---

![](https://i.imgflip.com/61fuow.jpg)

<!--
testing doesn't always start in tests though
-->
