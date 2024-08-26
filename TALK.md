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

## interleave ideas

a consistent theme you'll have seen with Soroban is that we have tried to learn from other blockchains:
 - what hasn't worked for them
 - what has worked
 - what's the best way to build great tooling
 - what's the most effective way to build tests

and so two things we wanted to do, and also influenced our choice of wasm + rust as the contract language was:
- we wanted devs to be able to byo their own rust tooling
  - do you use 
fully integrated the real vm

## What is testing?

unit testing
integration testing
fuzz testing
end-to-end testing
system testing
formal verification
state machine testing

functional testing
performance testing
regression testing

differential testing / fuzzing
invariant testing

user acceptance testing
ux testing

---

![](https://i.imgflip.com/61fuow.jpg)

<!--
testing doesn't always start in tests though
-->
