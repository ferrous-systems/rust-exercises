# STM32 TrustZone Exercise

In this exercise you'll get familiar with:

- Arm TrustZone-M (also known as *Cortex-M Security Extensions* or *CMSE*)
- The Armv8-M Security Attribution Unit
- The STM32U5 Global Trust Zone Controller
- Starting a Nonsecure Mode binary from Secure Mode
- Calling Secure Mode APIs from Nonsecure Mode
- Using *semihosting* and *defmt over RTT* for debug output at the same time from two different programs

To put these concepts in practice you'll write two applications that run at the same time on a NUCLEO-U5A5ZJ-Q board, powered by an STM32U5A5ZJ-Q microcontroller.

You will need to complete the [STM32 Introduction and Preparation](./preparation.md) before continuing.
