#ifndef RISCV_TEST_H_
#define RISCV_TEST_H_

#define RVTEST_RV32U

#define TESTNUM gp

#define RVTEST_CODE_BEGIN                                                      \
  .section .text;                                                              \
  .align 2;                                                                    \
  .globl _start;                                                               \
  _start:

#define RVTEST_CODE_END                                                        \
  unimp

// PASS: set a0 (x10) to 0 and execute ECALL
#define RVTEST_PASS                                                            \
  li a0, 0;                                                                    \
  ecall;

// FAIL: set a0 (x10) to failed test number (gp/x3 value) and execute ECALL
#define RVTEST_FAIL                                                            \
  addi a0, gp, 0;                                                              \
  bnez a0, 1f;                                                                 \
  li a0, 1;                                                                    \
  1: ecall;

#define RVTEST_DATA_BEGIN                                                      \
  .balign 4;                                                                   \
  .globl begin_signature;                                                      \
  begin_signature:

#define RVTEST_DATA_END                                                        \
  .balign 4;                                                                   \
  .globl end_signature;                                                        \
  end_signature:

#endif
