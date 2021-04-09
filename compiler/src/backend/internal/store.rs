use crate::asm;

pub fn store_u16(register: u8, value: u16) -> Vec<asm::Instruction> {
    if value == 0 {
        return vec![asm::Instruction::Xor(register, register)];
    }

    let bytes = value.to_be_bytes();
    vec![
        asm::Instruction::MovW(
            asm::Operand::Register(register),
            asm::Operand::Displacement8(2),
        ),
        asm::Instruction::Nop,
        asm::Instruction::BRA(1),
        asm::Instruction::Nop,
        asm::Instruction::Literal(bytes[0], bytes[1]),
    ]
}

pub fn store_u32(register: u8, value: u32) -> Vec<asm::Instruction> {
    if value == 0 {
        return vec![asm::Instruction::Xor(register, register)];
    }

    let other_reg = if register == 1 { 0 } else { 1 };
    let mut result = vec![asm::Instruction::Push(other_reg)];

    // First 2 Bytes into the register
    result.append(&mut store_u16(register, (value >> 16) as u16));

    // Next 2 Bytes into the other register
    result.append(&mut store_u16(other_reg, value as u16));

    result.extend_from_slice(&[
        // Shift first 2 bytes into correct spot
        asm::Instruction::Shll16(register),
        // Zero out top 2 byte of other_reg
        asm::Instruction::Shll16(other_reg),
        asm::Instruction::Shlr16(other_reg),
        // Add these two together
        asm::Instruction::Add(register, other_reg),
        // Restore previous value in other_reg
        asm::Instruction::Pop(other_reg),
    ]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16() {
        let result = store_u16(0, 0x1234);

        let target_pc = (result.len() * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut test_em = emulator::Emulator::new_test(&mut input, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let expected_registers = [
            0x1234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000,
        ];
        let final_registers = test_em.clone_registers();

        assert_eq!(expected_registers, final_registers);
    }

    #[test]
    fn u32() {
        let result = store_u32(0, 0x12345678);

        let target_pc = (result.len() * 2) as u32 + emulator::CODE_MAPPING_OFFSET;

        let mut input = emulator::MockInput::new(vec![]);
        let mut test_em = emulator::Emulator::new_test(&mut input, result);

        assert!(test_em.run_until(target_pc).is_ok());

        let expected_registers = [
            0x12345678, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80000, 0x80000,
        ];
        let final_registers = test_em.clone_registers();

        assert_eq!(expected_registers, final_registers);
    }
}