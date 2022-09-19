/*
 *  @date   : 2018/04/18
 *  @author : Rprop (r_prop@outlook.com)
 *  https://github.com/Rprop/And64InlineHook
 */
/*
 MIT License

 Copyright (c) 2018 Rprop (r_prop@outlook.com)

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
 */
#pragma once

#include "common.hpp"

#include <cstring>
#include <type_traits>
#include "lib/reloc/rtld.hpp"

namespace exl::hook::nx64 {

    enum HookHandlerType : unsigned char {
        Hook = 0,
        Inline = 1,
        InlineEx = 2,
        Detour = 3
    };

    struct HookData;
    struct HookCtx {
        HookData* data;
    };
    struct PltHookData {
        uintptr_t callback;
        PltHookData* next;
        uintptr_t* p_callback_trampoline;
        HookHandlerType ty;
    };

    static_assert(sizeof(HookCtx) == 0x8);
    static_assert(offsetof(HookCtx,    data) == 0x00);

    struct HookData {
        uintptr_t trampoline;
        uintptr_t callback;
        uintptr_t handler;
        bool is_enabled;
    };

    static_assert(sizeof(HookData) == 0x20);
    static_assert(offsetof(HookData, trampoline) == 0x00);
    static_assert(offsetof(HookData,   callback) == 0x08);
    static_assert(offsetof(HookData,    handler) == 0x10);
    static_assert(offsetof(HookData, is_enabled) == 0x18);


    template<typename T>
    concept RealFunction = std::is_function_v<T> || std::is_function_v<std::remove_pointer_t<T>> || std::is_function_v<std::remove_reference_t<T>>;

    uintptr_t HookFuncCommon(uintptr_t hook, uintptr_t callback, bool do_trampoline = false);
    Result AllocForTrampoline(uint32_t** rx, uint32_t** rw);

    typedef union {
        u64 x;  ///< 64-bit AArch64 register view.
        u32 w;  ///< 32-bit AArch64 register view.
        u32 r;  ///< AArch32 register view.
    } CpuRegister;

    struct InlineCtx {
        CpuRegister registers[29];
    };
    using InlineCallback = void (*)(InlineCtx*);

    void Initialize();

    template<typename R, typename A1, typename A2>
    R HookFuncRaw(A1 hook, A2 callback, bool do_trampoline = false) {
        auto r = HookFuncCommon(
            reinterpret_cast<uintptr_t>(hook),
            reinterpret_cast<uintptr_t>(callback),
            do_trampoline
        );

        /* Return nothing if return type is void. */
        if constexpr(std::is_same_v<R, void>)
            return;
        /* Otherwise return the value. */
        else {
            return reinterpret_cast<R>(r);
        }
    }

    template<typename Func> requires RealFunction<Func> || std::is_member_function_pointer_v<Func>
    Func HookFunc(Func hook, Func callback, bool do_trampoline = false) {

        /* Workaround for being unable to cast member functions. */
        /* Probably some horrible UB here? */
        uintptr_t hookp;
        uintptr_t callbackp;
        memcpy(&hookp, &hook, sizeof(hookp));
        memcpy(&callbackp, &callback, sizeof(callbackp));

        uintptr_t trampoline = HookFuncCommon(hookp, callbackp, do_trampoline);

        /* Workaround for being unable to cast member functions. */
        /* Probably some horrible UB here? */
        Func ret;
        memcpy(&ret, &trampoline, sizeof(trampoline));

        return ret;
    }

    template<typename Func> requires RealFunction<Func>
    Func HookFunc(uintptr_t hook, Func callback, bool do_trampoline = false) {
        return HookFunc(reinterpret_cast<Func>(hook), callback, do_trampoline);
    }
    
    template<typename Func> requires RealFunction<Func>
    Func HookFunc(uintptr_t hook, uintptr_t callback, bool do_trampoline = false) {
        return HookFunc(reinterpret_cast<Func>(hook), reinterpret_cast<Func>(callback), do_trampoline);
    }

    template<typename Func1, typename Func2> 
    requires 
    /* Both funcs are member pointers. */
    std::is_member_function_pointer_v<Func1> && std::is_member_function_pointer_v<Func2>
    /* TODO: ensure safety that Func2 can be casted to Func1 */
    Func1 HookFunc(Func1 hook, Func2 callback, bool do_trampoline = false) {
        return HookFunc(reinterpret_cast<Func1>(hook), reinterpret_cast<Func1>(callback), do_trampoline);
    }

    extern "C" const void* install_hook(const void* symbol, const void* replace, HookHandlerType ty);
    extern "C" void install_hook_in_plt(rtld::ModuleObject* host_object, const void* function, const void* replace, void** out_trampoline, HookHandlerType ty);

    //void InlineHook(uintptr_t addr, InlineCallback* callback);
};