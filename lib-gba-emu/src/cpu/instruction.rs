pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

pub enum Instruction {
    // Arithmetic Instructions
    INC(IncDecTarget),
    DEC(IncDecTarget),

    /// ADD, 寄存器A的值 += 目标值
    ADD(ArithmeticTarget), ///< 123
    /// ADD with carry, the value of the carry flag is also added to the number
    ADC(),
    /// ADD to HL, 寄存器HL的值(16位) += 目标值, 可以理解为16位加
    ADDHL(),
    /// add stack pointer, SP += value
    ADDSP(),
    /// Subtract with carry
    SUB(),
    //
    SBC(),
    // 与
    AND(),
    // 或
    OR(),
    // 异或
    XOR(),
    //
    CP(), //
}
