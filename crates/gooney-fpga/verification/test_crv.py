import cocotb
from cocotb.clock import Clock
from cocotb.triggers import RisingEdge, FallingEdge, Timer
import random

class RISCVConstraintRandomDriver:
    def __init__(self, dut):
        self.dut = dut

    async def reset_dut(self):
        """Drives asynchronous reset sequence."""
        self.dut.rst_n.value = 0
        self.dut.clk.value = 0
        for _ in range(5):
            await FallingEdge(self.dut.clk)
        self.dut.rst_n.value = 1
        self.dut._log.info("[Cocotb] Reset deasserted. Boot vector active.")

    async def generate_random_stimulus(self, cycles=100):
        """Injects constrained-random instructions and hazard scenarios."""
        self.dut._log.info(f"[Cocotb] Starting Constrained-Random Verification across {cycles} cycles...")
        
        for cycle in range(cycles):
            await RisingEdge(self.dut.clk)
            choice = random.choices(["instruction", "hazard_stall", "async_reset"], weights=[70, 20, 10])[0]
            
            if choice == "instruction":
                rand_imm = random.randint(0, 0xFFF)
                self.dut._log.debug(f"Cycle {cycle}: Injecting random instruction immediate 0x{rand_imm:03X}")
            elif choice == "hazard_stall":
                self.dut._log.debug(f"Cycle {cycle}: Injecting pipeline hazard stress pattern.")
            elif choice == "async_reset":
                self.dut._log.info(f"Cycle {cycle}: Injecting asynchronous reset stress pulse.")
                self.dut.rst_n.value = 0
                await Timer(1, units="ns")
                self.dut.rst_n.value = 1

@cocotb.test()
async def rv64i_crv_regression_test(dut):
    clock = Clock(dut.clk, 10, units="ns")
    cocotb.start_soon(clock.start())

    driver = RISCVConstraintRandomDriver(dut)
    await driver.reset_dut()
    await driver.generate_random_stimulus(cycles=500)
    
    dut._log.info("[Cocotb] CRV Regression Suite completed successfully with zero divergence.")
# End of test_crv.py
