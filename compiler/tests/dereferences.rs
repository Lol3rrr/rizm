use compiler;
use emulator;

#[tokio::test]
async fn simple_dereference() {
    let target_address: usize = 13123;
    let target_value: u8 = 1;
    let program = "int main() {
        *13123 = 1;
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn dereference_expression_1() {
    let target_address: usize = 13123 + 2;
    let target_value: u8 = 1;
    let program = "int main() {
        *(13123 + 2) = 1;
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn dereference_expression_2() {
    let target_address: usize = 13123 + 2;
    let target_value: u8 = 1;
    let program = "int main() {
        unsigned int address = 13123;
        *(address + 2) = 1;
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn dereference_expression_3() {
    let target_address: usize = 13123 + 2 * 3;
    let target_value: u8 = 1;
    let program = "int main() {
        unsigned int address = 13123;
        *(address + 2 * 3) = 1;
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn dereference_expression_4() {
    let target_address: usize = 13123 + 2 * 6;
    let target_value: u8 = 1;
    let program = "int main() {
        unsigned int address = 13123;
        int width = 6;
        *(address + 2 * width) = 1;
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}
