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

    /// Add, 寄存器A的值 += 目标值
    ADD(ArithmeticTarget), ///< 123
    /// Add with carry, the value of the carry flag is also added to the number
    ADC(),
    /// Add to HL, 寄存器HL的值(16位) += 目标值, 可以理解为16位加
    ADDHL(),
    /// Add stack pointer, SP += value
    ADDSP(),
    /// Sub, A -= value
    SUB(),
    /// Subtract with carry
    SBC(),
    /// AND, A &= value
    AND(),
    /// OR, A |= value
    OR(),
    /// XOR, A ^= value
    XOR(),
    /// compare, return A - value, (A is not overwritten)
    CP(),

    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    DAA,

    // Prefix Instructions
    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget),

    // Jump Instructions
    JP(JumpTest),
    JR(JumpTest),
    JPI,

    // Load Instructions
    LD(LoadType),

    // Stack Instructions
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    RETI,
    RST(RSTLocation),

    // Control Instructions
    HALT,
    NOP,
    DI,
    EI,
}
