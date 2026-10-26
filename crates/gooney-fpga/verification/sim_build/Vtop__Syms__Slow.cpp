// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "Vtop__pch.h"

extern const VlVarTableEntry Vtop___024root__VpiVarTable0[];
extern const VlVarTableEntry Vtop___024root__VpiVarTable1[];
extern const VlScopeTableEntry Vtop__Syms__VpiScopeTable[];


// VPI VARIABLE/SCOPE TABLES
#if defined(__GNUC__)
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Winvalid-offsetof"
#endif
extern const VlVarTableEntry Vtop___024root__VpiVarTable0[] = {
    {"clk", offsetof(Vtop___024root, clk), VLVT_UINT8, (VLVD_IN|VLVF_PUB_RW), 0, 0, {0, 0, 0, 0, 0, 0}},
    {"rst_n", offsetof(Vtop___024root, rst_n), VLVT_UINT8, (VLVD_IN|VLVF_PUB_RW), 0, 0, {0, 0, 0, 0, 0, 0}},
};
extern const VlVarTableEntry Vtop___024root__VpiVarTable1[] = {
    {"clk", offsetof(Vtop___024root, core_top__DOT__clk), VLVT_UINT8, (VLVD_NODIR|VLVF_PUB_RW|VLVF_CONTINUOUSLY), 0, 0, {0, 0, 0, 0, 0, 0}},
    {"rst_n", offsetof(Vtop___024root, core_top__DOT__rst_n), VLVT_UINT8, (VLVD_NODIR|VLVF_PUB_RW|VLVF_CONTINUOUSLY), 0, 0, {0, 0, 0, 0, 0, 0}},
};
extern const VlScopeTableEntry Vtop__Syms__VpiScopeTable[] = {
    {offsetof(Vtop__Syms, __Vscopep_TOP), "TOP", "TOP", "<null>", 0, VerilatedScope::SCOPE_OTHER},
    {offsetof(Vtop__Syms, __Vscopep_core_top), "core_top", "core_top", "core_top", -9, VerilatedScope::SCOPE_MODULE},
};
#if defined(__GNUC__)
# pragma GCC diagnostic pop
#endif
Vtop__Syms::Vtop__Syms(VerilatedContext* contextp, const char* namep, Vtop* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    , __Vm_didInit{modelp->m_didInit}
    , __Vm_threadPoolp{static_cast<VlThreadPool*>(contextp->threadPoolp())}
    // Setup top module instance
    , TOP{this, namep}
{
    // Check resources
    Verilated::stackCheck(92);
    // Setup sub module instances
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-9);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
    // Setup scopes
    VerilatedScope::scopesConstructFromTable(Vtop__Syms__VpiScopeTable, 2, this);
    // Set up scope hierarchy
    __Vhier.add(0, __Vscopep_core_top);
    // Setup export functions - final: 0
    // Setup export functions - final: 1
    // Setup public variables
    __Vscopep_TOP->varsInsertFromTable(Vtop___024root__VpiVarTable0, 2, &(TOP));
    __Vscopep_core_top->varsInsertFromTable(Vtop___024root__VpiVarTable1, 2, &(TOP));
}

Vtop__Syms::~Vtop__Syms() {
    // Tear down scope hierarchy
    __Vhier.remove(0, __Vscopep_core_top);
    // Clear keys from hierarchy map after values have been removed
    __Vhier.clear();
    // Tear down scopes
    VL_DO_CLEAR(delete __Vscopep_TOP, __Vscopep_TOP = nullptr);
    VL_DO_CLEAR(delete __Vscopep_core_top, __Vscopep_core_top = nullptr);
    // Tear down sub module instances
}
