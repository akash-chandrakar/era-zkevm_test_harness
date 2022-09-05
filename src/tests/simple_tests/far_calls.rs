use super::*;

#[test_log::test]
fn test_far_call_and_read_fat_pointer() {
    // makes 36 bytes of calldata in aux heap and calls with it
    let asm = r#"
        .text
        .file	"Test_26"
        .rodata.cst32
        .p2align	5
    CPI0_0:
	    .cell 30272441630670900764332283662402067049651745785153368133042924362431065855
        .cell 30272434434303437454318367229716471635614919446304865000139367529706422272
    CPI0_1:
	    .cell 65536
        .text
        .globl	__entry
    __entry:
    .main:
        add 64, r0, r2
        add @CPI0_0[0], r0, r3
        st.2.inc r2, r3, r2
        add @CPI0_0[1], r0, r3
        st.2 r2, r3
        add 2, r0, r1
        shl.s 136, r1, r1
        add 36, r1, r1
        shl.s 32, r1, r1
        add 64, r1, r1
        shl.s 64, r1, r1
        add @CPI0_1[0], r0, r2
        far_call r1, r2, @catch_all
        ret.ok r0
    catch_all:
        ret.panic r0
    "#;

    // this one reads some calldata, including partially beyond the bound,
    // and completely beoynd the bound, and returns
    let other_asm = r#"
        .text
        .file	"Test_26"
        .rodata.cst32
        .p2align	5
        .text
        .globl	__entry
    __entry:
    .main:
        sstore r1, r1
        event.first r1, r0
        to_l1.first r0, r1
        ld.inc r1, r2, r1
        ld.inc r1, r3, r1
        ld r1, r4
        ret.ok r0
    "#;

    let entry_bytecode = Assembly::try_from(asm.to_owned()).unwrap().compile_to_bytecode().unwrap();
    use crate::ethereum_types::Address;
    let other_address = Address::from_low_u64_be(1u64 << 16);
    let other_bytecode = Assembly::try_from(other_asm.to_owned()).unwrap().compile_to_bytecode().unwrap();

    run_and_try_create_witness_for_extended_state(
        entry_bytecode, 
        vec![(other_address, other_bytecode)], 
        50
    );
}


#[test_log::test]
fn test_simple_mimic_call() {
    // makes 36 bytes of calldata in aux heap and calls with it
    let asm = r#"
        .text
        .file	"Test_26"
        .rodata.cst32
        .p2align	5
    CPI0_0:
	    .cell 30272441630670900764332283662402067049651745785153368133042924362431065855
        .cell 30272434434303437454318367229716471635614919446304865000139367529706422272
    CPI0_1:
	    .cell 65536
        .text
        .globl	__entry
    __entry:
    .main:
        add 64, r0, r2
        add @CPI0_0[0], r0, r3
        st.2.inc r2, r3, r2
        add @CPI0_0[1], r0, r3
        st.2 r2, r3
        add 2, r0, r1
        shl.s 136, r1, r1
        add 36, r1, r1
        shl.s 32, r1, r1
        add 64, r1, r1
        shl.s 64, r1, r1
        add @CPI0_1[0], r0, r2
        add @CPI0_0[0], r0, r3
        far_call.mimic r1, r2, @catch_all
        ret.ok r0
    catch_all:
        ret.panic r0
    "#;

    // this one reads some calldata, including partially beyond the bound,
    // and completely beoynd the bound, and returns
    let other_asm = r#"
        .text
        .file	"Test_26"
        .rodata.cst32
        .p2align	5
        .text
        .globl	__entry
    __entry:
    .main:
        sstore r1, r1
        event.first r1, r0
        to_l1.first r0, r1
        ld.inc r1, r2, r1
        ld.inc r1, r3, r1
        ld r1, r4
        ret.ok r0
    "#;

    let entry_bytecode = Assembly::try_from(asm.to_owned()).unwrap().compile_to_bytecode().unwrap();
    use crate::ethereum_types::Address;
    let other_address = Address::from_low_u64_be(1u64 << 16);
    let other_bytecode = Assembly::try_from(other_asm.to_owned()).unwrap().compile_to_bytecode().unwrap();

    run_and_try_create_witness_for_extended_state(
        entry_bytecode, 
        vec![(other_address, other_bytecode)], 
        50
    );
}