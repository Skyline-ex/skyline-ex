#include "mem_layout.hpp"

#include <reloc/rtld.hpp>
#include <program/setting.hpp>
    
/* Provided by linkerscript, the start of our executable. */
extern "C" {
    extern char __module_start;
}

namespace exl::util {

    
    void impl::InitMemLayout() {
        /* Setup loop, starting address zero. */
        MemoryInfo meminfo { };
        u32 pageinfo;

        enum class State {
            LookingForCodeStatic,
            ExpectingRodata,
            ExpectingData,
        } state = State::LookingForCodeStatic;
        int nextModIdx = 0;

        uintptr_t offset = 0;
        uintptr_t prevOffset = 0;
        util::ModuleInfo curModInfo {};

        /* Utility to reset state.  */
        auto reset = [&curModInfo, &state] {
            curModInfo = {};
            state =  State::LookingForCodeStatic;
        };

        do {
            /* Ensure we don't find too many modules. */
            if(util::mem_layout::s_MaxModules <= nextModIdx)
                EXL_ABORT(result::TooManyStaticModules);

            prevOffset = offset;

            /* Query next range. */
            R_ABORT_UNLESS(svcQueryMemory(&meminfo, &pageinfo, meminfo.addr + meminfo.size));

            /* Setup variables for state machine. */
            u32 memtype = meminfo.type & MemState_Type;
            offset = meminfo.addr;

            switch (state) {
                case State::LookingForCodeStatic: {
                    if(memtype != MemType_CodeStatic || meminfo.perm != Perm_Rx) {
                        /* No module here, keep going... */
                        continue;
                    }

                    /* Store text/total info and move to next state. */
                    curModInfo.m_Total.m_Start = meminfo.addr;
                    curModInfo.m_Text.m_Start = meminfo.addr;
                    curModInfo.m_Text.m_Size = meminfo.size;

                    rtld::ModuleHeader* header = reinterpret_cast<rtld::ModuleHeader*>(meminfo.addr + *reinterpret_cast<u32*>(meminfo.addr + 4));
                    curModInfo.m_ModuleHeader = header;
                    curModInfo.m_ModuleObject = reinterpret_cast<rtld::ModuleObject*>(reinterpret_cast<char*>(header) + header->module_object_offset);
                    curModInfo.m_Bss.m_Start = reinterpret_cast<uintptr_t>(reinterpret_cast<char*>(header) + header->bss_start_offset);
                    curModInfo.m_Bss.m_Size = static_cast<size_t>(header->bss_end_offset - header->bss_start_offset);
                    state =  State::ExpectingRodata;
                    break;
                }
                case State::ExpectingRodata: {
                    if(memtype != MemType_CodeStatic || meminfo.perm != Perm_R) {
                        /* Not a proper module, reset. */
                        reset();
                        continue;
                    }

                    /* Store rodata range and move to next state. */
                    curModInfo.m_Rodata.m_Start = meminfo.addr;
                    curModInfo.m_Rodata.m_Size = meminfo.size;
                    state =  State::ExpectingData;
                    break;
                }
                case State::ExpectingData: {
                    if(memtype != MemType_CodeMutable || meminfo.perm != Perm_Rw) {
                        /* Not a proper module, reset. */
                        reset();
                        continue;
                    }

                    auto& total = curModInfo.m_Total;
                    auto& data = curModInfo.m_Data;

                    /* Assign the data range and finish total range. */
                    data.m_Start = meminfo.addr;
                    data.m_Size = meminfo.size;
                    total.m_Size = data.GetEnd() - total.m_Start;

                    /* This only needs to be determined at runtime if we are in an application. */
                    #ifdef EXL_AS_MODULE
                    if(total.m_Start == (uintptr_t) &__module_start)
                        util::mem_layout::s_SelfModuleIdx = nextModIdx;
                    #endif

                    /* Store built module info. */
                    impl::mem_layout::s_ModuleInfos[nextModIdx] = curModInfo;
                    nextModIdx++;
                    util::mem_layout::s_ModuleCount = nextModIdx;

                    /* Back to initial state. */
                    reset();
                    break;
                }
                EXL_UNREACHABLE_DEFAULT_CASE();
            }

        /* Exit once we've wrapped the address space. */
        } while(offset >= prevOffset);

        /* Ensure we found a valid self index and module count. */
        EXL_ASSERT(util::mem_layout::s_SelfModuleIdx != -1);
        EXL_ASSERT(util::mem_layout::s_ModuleCount != -1);
        EXL_ASSERT(util::mem_layout::s_SelfModuleIdx < util::mem_layout::s_MaxModules);
        EXL_ASSERT(util::mem_layout::s_ModuleCount <= util::mem_layout::s_MaxModules);
    }
};